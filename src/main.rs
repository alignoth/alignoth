mod cli;
mod highlight;
mod plot;
mod utils;
mod wizard;

use crate::cli::{DataFormat, Preprocess};
use crate::highlight::{BedHighlight, Highlight, VcfHighlight};
use crate::plot::create_plot_data;
use crate::wizard::wizard_mode;
use anyhow::{bail, Result};
use csv::WriterBuilder;
use log::LevelFilter;
use lz_str::compress_to_utf16;
use serde_json::{json, Value};
use simplelog::{ColorChoice, Config, TermLogger, TerminalMode};
use std::cmp::min;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;
use tera::{Context, Tera};

#[tokio::main]
async fn main() -> Result<()> {
    let wizard = std::env::args().len() == 1;
    let mut opt = if wizard {
        wizard_mode().await?
    } else {
        cli::Alignoth::from_args()
    };
    let _ = TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    );
    opt.preprocess()?;
    let region = opt.region.as_ref().unwrap();

    let mut plot_specs: Value = serde_json::from_str(include_str!("../resources/plot.vl.json"))?;
    let width = json!(min(opt.max_width, 5 * region.length()));
    let domain = json!(vec![region.start as f32 - 0.5, region.end as f32 - 0.5]);

    let template_coverage = plot_specs["vconcat"][0].clone();
    let template_reads = plot_specs["vconcat"][1].clone();
    let mut new_vconcat = Vec::new();

    let mut all_read_data = Vec::new();
    let mut all_coverage_data = Vec::new();
    let mut reference_data = None;

    for (i, bam) in opt.bam_path.iter().enumerate() {
        let bam_name = bam
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .trim_end_matches(".bam")
            .trim_end_matches(".sam")
            .trim_end_matches(".cram")
            .to_string();

        let (mut read_data, ref_data, total_reads, coverage_data, retained_reads) =
            create_plot_data(
                bam,
                &opt.reference.as_ref().unwrap(),
                region,
                opt.max_read_depth,
                opt.aux_tag.clone(),
                opt.mismatch_display_min_percent,
                bam_name.clone(),
            )?;

        if reference_data.is_none() {
            reference_data = Some(ref_data);
        }
        all_read_data.append(&mut read_data);
        all_coverage_data.push(coverage_data);

        let subsampling_warning = if total_reads > retained_reads {
            format!("{} ({} of {} reads)", bam_name, retained_reads, total_reads)
        } else {
            format!("{} ({} reads)", bam_name, total_reads)
        };

        let mut cov = template_coverage.clone();
        cov["width"] = width.clone();
        if i == 0 {
            cov["title"] = json!({
                "text": &region.target,
            });
        } else if let Some(obj) = cov.as_object_mut() {
            obj.remove("title");
        }
        if let Some(arr) = cov["transform"].as_array_mut() {
            arr.insert(
                0,
                json!({ "filter": format!("datum.sample == '{}'", bam_name) }),
            );
        }
        cov["encoding"]["y"]["axis"]["title"] = json!(format!("{} cov", bam_name));

        let mut rds = template_reads.clone();
        rds["width"] = width.clone();
        rds["encoding"]["x"]["scale"]["domain"] = domain.clone();
        rds["encoding"]["y"]["axis"]["title"] = json!(subsampling_warning);

        if let Some(layers) = rds["layer"].as_array_mut() {
            for layer in layers {
                if layer["data"]["name"] == "reads" {
                    if let Some(arr) = layer["transform"].as_array_mut() {
                        arr.insert(
                            0,
                            json!({ "filter": format!("datum.sample == '{}'", bam_name) }),
                        );
                    }
                }

                if i > 0 && layer["data"]["name"] == "reference" {
                    if let Some(obj) = layer.as_object_mut() {
                        obj.remove("params");
                    }
                }

                if let Some(params) = layer.get_mut("params").and_then(|p| p.as_array_mut()) {
                    for param in params {
                        if param["name"] == "rplc" {
                            param["name"] = json!(format!("rplc_{}", i));
                        }
                    }
                }

                if let Some(encoding) = layer.get_mut("encoding") {
                    if let Some(opacity) = encoding.get_mut("opacity") {
                        if let Some(condition) = opacity.get_mut("condition") {
                            if condition["param"] == "rplc" {
                                condition["param"] = json!(format!("rplc_{}", i));
                            }
                        }
                    }
                }
            }
        }
        new_vconcat.push(cov);
        new_vconcat.push(rds);
    }
    plot_specs["vconcat"] = json!(new_vconcat);
    let reference_data = reference_data.unwrap();
    let reference = match opt.data_format {
        DataFormat::Json => json!(reference_data).to_string().as_bytes().to_vec(),
        DataFormat::Tsv => {
            let mut writer = WriterBuilder::new().delimiter(b'\t').from_writer(vec![]);
            writer.serialize(&reference_data)?;
            writer.into_inner()?
        }
    };
    let coverage = match opt.data_format {
        DataFormat::Json => json!(all_coverage_data).to_string().as_bytes().to_vec(),
        DataFormat::Tsv => {
            let mut writer = WriterBuilder::new().delimiter(b'\t').from_writer(vec![]);
            for item in &all_coverage_data {
                writer.serialize(item)?;
            }
            writer.into_inner()?
        }
    };
    let reads = match opt.data_format {
        DataFormat::Json => json!(all_read_data).to_string().as_bytes().to_vec(),
        DataFormat::Tsv => {
            let mut writer = WriterBuilder::new().delimiter(b'\t').from_writer(vec![]);
            for record in &all_read_data {
                writer.serialize(record)?;
            }
            writer.into_inner()?
        }
    };
    let mut highlight = opt.highlight.as_ref().cloned().unwrap_or_default();
    if let Some(vcf_path) = opt.vcf.as_ref() {
        let csi_index = PathBuf::from(&format!("{}.csi", vcf_path.display()));
        let tbi_index = PathBuf::from(&format!("{}.tbi", vcf_path.display()));
        if csi_index.exists() || tbi_index.exists() {
            highlight.extend(VcfHighlight::new(vcf_path.clone()).intervals(region)?);
        } else {
            bail!(
                "VCF/BCF index not found: {csi} or {tbi}. Please create an index using `bcftools index {vcf}` or `tabix {vcf}`.",
                csi = csi_index.display(),
                tbi = tbi_index.display(),
                vcf = vcf_path.display()
            );
        }
    }
    if let Some(bed_path) = opt.bed.as_ref() {
        highlight.extend(BedHighlight::new(bed_path.clone()).intervals(region)?);
    }

    highlight.iter_mut().for_each(|h| h.preprocess());
    let highlights = match opt.data_format {
        DataFormat::Json => json!(highlight).to_string().as_bytes().to_vec(),
        DataFormat::Tsv => {
            let mut writer = WriterBuilder::new().delimiter(b'\t').from_writer(vec![]);
            writer.serialize(&highlight)?;
            writer.into_inner()?
        }
    };

    if let Some(out_path) = &opt.output {
        if !out_path.exists() {
            std::fs::create_dir_all(out_path)?;
        }
        let bam_file_name = opt
            .bam_path
            .iter()
            .map(|p| {
                p.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .trim_end_matches(".bam")
                    .trim_end_matches(".sam")
                    .trim_end_matches(".cram")
            })
            .collect::<Vec<_>>()
            .join("_");
        let highlight_path = if opt.highlight.is_some() || opt.vcf.is_some() || opt.bed.is_some() {
            Some(Path::join(
                out_path,
                format!("{}.highlight.{}", bam_file_name, opt.data_format),
            ))
        } else {
            None
        };
        write_files(
            json!(plot_specs).to_string().as_bytes(),
            &reference,
            &reads,
            &highlights,
            &coverage,
            &Path::join(out_path, format!("{bam_file_name}.vl.json")),
            &Path::join(
                out_path,
                format!("{}.reference.{}", bam_file_name, opt.data_format),
            ),
            &Path::join(
                out_path,
                format!("{}.reads.{}", bam_file_name, opt.data_format),
            ),
            highlight_path,
            &Path::join(
                out_path,
                format!("{}.coverage.{}", bam_file_name, opt.data_format),
            ),
        )?;
    } else if let (
        Some(spec_output),
        Some(ref_data_output),
        Some(read_data_output),
        Some(coverage_output),
    ) = (
        &opt.spec_output,
        &opt.ref_data_output,
        &opt.read_data_output,
        &opt.coverage_output,
    ) {
        write_files(
            json!(plot_specs).to_string().as_bytes(),
            &reference,
            &reads,
            &highlights,
            &coverage,
            spec_output,
            ref_data_output,
            read_data_output,
            opt.highlight_data_output,
            coverage_output,
        )?;
    } else {
        plot_specs["datasets"]["reference"] = json!(reference_data);
        plot_specs["datasets"]["reads"] = json!(all_read_data);
        plot_specs["datasets"]["highlight"] = json!(highlight);
        plot_specs["datasets"]["coverage"] = json!(all_coverage_data);
        let bam_name = opt
            .bam_path
            .iter()
            .map(|p| {
                p.file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .trim_end_matches(".bam")
                    .trim_end_matches(".sam")
                    .trim_end_matches(".cram")
            })
            .collect::<Vec<_>>()
            .join("_");
        if opt.html {
            let mut templates = Tera::default();
            templates.add_raw_template("plot", include_str!("../resources/plot.html.tera"))?;
            let mut context = Context::new();
            context.insert("num_bams", &opt.bam_path.len());
            context.insert(
                "spec",
                &json!(compress_to_utf16(&plot_specs.to_string())).to_string(),
            );
            if opt.no_embed_js {
                context.insert(
                    "vega",
                    r#"<script src="https://cdn.jsdelivr.net/npm/vega@5"></script>"#,
                );
                context.insert(
                    "vegalite",
                    r#"<script src="https://cdn.jsdelivr.net/npm/vega-lite@5"></script>"#,
                );
                context.insert(
                    "vegaembed",
                    r#"<script src="https://cdn.jsdelivr.net/npm/vega-embed@6"></script>"#,
                );
                context.insert(
                    "lzstring",
                    r#"<script src="https://cdn.jsdelivr.net/npm/lz-string@1"></script>"#,
                );
            } else {
                context.insert(
                    "vega",
                    concat!(
                        "<script>",
                        include_str!("../resources/vega.min.js"),
                        "</script>"
                    ),
                );
                context.insert(
                    "vegalite",
                    concat!(
                        "<script>",
                        include_str!("../resources/vega-lite.min.js"),
                        "</script>"
                    ),
                );
                context.insert(
                    "vegaembed",
                    concat!(
                        "<script>",
                        include_str!("../resources/vega-embed.min.js"),
                        "</script>"
                    ),
                );
                context.insert(
                    "lzstring",
                    concat!(
                        "<script>",
                        include_str!("../resources/lz-string.min.js"),
                        "</script>"
                    ),
                );
            }
            let html = templates.render("plot", &context)?;
            if wizard {
                std::fs::write(format!("{bam_name}.html"), html.as_bytes())?;
                println!("Plot saved to {bam_name}.html 🪄");
            } else {
                stdout().write_all(html.as_bytes())?;
            }
        } else if wizard {
            std::fs::write(
                format!("{bam_name}.vl.json"),
                plot_specs.to_string().as_bytes(),
            )?;
            println!("Plot saved to {bam_name}.vl.json 🪄");
        } else {
            stdout().write_all(plot_specs.to_string().as_bytes())?;
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_files(
    spec_data: &[u8],
    ref_data: &[u8],
    read_data: &[u8],
    highlight_data: &[u8],
    coverage_data: &[u8],
    spec_path: &Path,
    ref_path: &Path,
    read_path: &Path,
    highlight_path: Option<PathBuf>,
    coverage_path: &Path,
) -> Result<()> {
    let mut specs = File::create(spec_path).unwrap();
    specs.write_all(spec_data)?;
    let mut read_file = File::create(read_path).unwrap();
    read_file.write_all(read_data)?;
    let mut reference_file = File::create(ref_path).unwrap();
    reference_file.write_all(ref_data)?;
    let mut coverage_file = File::create(coverage_path).unwrap();
    coverage_file.write_all(coverage_data)?;
    if let Some(path) = highlight_path {
        let mut highlight_file = File::create(path).unwrap();
        highlight_file.write_all(highlight_data)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::write_files;
    use std::fs;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_write_files() {
        write_files(
            "test spec".as_bytes(),
            "test ref".as_bytes(),
            "test read".as_bytes(),
            "test highlight".as_bytes(),
            "test coverage".as_bytes(),
            Path::new("/tmp/test_spec.json"),
            Path::new("/tmp/test_ref.json"),
            Path::new("/tmp/test_read.json"),
            Some(PathBuf::from("/tmp/test_highlight.json")),
            Path::new("/tmp/test_coverage.json"),
        )
        .unwrap();
        assert!(Path::new("/tmp/test_spec.json").exists());
        assert!(Path::new("/tmp/test_ref.json").exists());
        assert!(Path::new("/tmp/test_read.json").exists());
        assert!(Path::new("/tmp/test_highlight.json").exists());
        assert!(Path::new("/tmp/test_coverage.json").exists());
        assert_eq!(
            fs::read_to_string("/tmp/test_spec.json").unwrap(),
            "test spec"
        );
        assert_eq!(
            fs::read_to_string("/tmp/test_ref.json").unwrap(),
            "test ref"
        );
        assert_eq!(
            fs::read_to_string("/tmp/test_read.json").unwrap(),
            "test read"
        );
        assert_eq!(
            fs::read_to_string("/tmp/test_highlight.json").unwrap(),
            "test highlight"
        );
        fs::remove_file("/tmp/test_spec.json").unwrap();
        fs::remove_file("/tmp/test_ref.json").unwrap();
        fs::remove_file("/tmp/test_read.json").unwrap();
        fs::remove_file("/tmp/test_highlight.json").unwrap();
    }
}
