mod cli;
mod plot;
mod utils;

use crate::cli::{DataFormat, Interval, Preprocess};
use crate::plot::create_plot_data;
use anyhow::Result;
use csv::WriterBuilder;
use log::LevelFilter;
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
    let mut opt = cli::Alignoth::from_args();
    let _ = TermLogger::init(
        LevelFilter::Warn,
        Config::default(),
        TerminalMode::Stderr,
        ColorChoice::Auto,
    );
    opt.preprocess()?;
    let (read_data, reference_data, total_reads, retained_reads) = create_plot_data(
        &opt.bam_path.as_ref().unwrap(),
        &opt.reference.as_ref().unwrap(),
        opt.region.as_ref().unwrap(),
        opt.max_read_depth,
        opt.aux_tag,
    )?;
    let highlight = opt.highlight.map(|h| Interval {
        start: h.start - 0.5,
        end: h.end + 0.5,
    });
    let mut plot_specs: Value = serde_json::from_str(include_str!("../resources/plot.vl.json"))?;
    plot_specs["width"] = json!(min(
        opt.max_width,
        5 * (opt.region.as_ref().unwrap().length())
    ));
    plot_specs["encoding"]["x"]["scale"]["domain"] = json!(vec![
        opt.region.as_ref().unwrap().start as f32 - 0.5,
        opt.region.as_ref().unwrap().end as f32 - 0.5
    ]);
    let subsampling_warning = if total_reads > retained_reads {
        format!("{retained_reads} of {total_reads} reads (subsampled)")
    } else {
        format!("{total_reads} reads")
    };
    plot_specs["title"] = json!({
            "text": &opt.region.unwrap().target,
            "subtitle": subsampling_warning,
    });
    let reference = match opt.data_format {
        DataFormat::Json => json!(reference_data).to_string().as_bytes().to_vec(),
        DataFormat::Tsv => {
            let mut writer = WriterBuilder::new().delimiter(b'\t').from_writer(vec![]);
            writer.serialize(&reference_data)?;
            writer.into_inner()?
        }
    };
    let reads = match opt.data_format {
        DataFormat::Json => json!(read_data).to_string().as_bytes().to_vec(),
        DataFormat::Tsv => {
            let mut writer = WriterBuilder::new().delimiter(b'\t').from_writer(vec![]);
            for record in &read_data {
                writer.serialize(record)?;
            }
            writer.into_inner()?
        }
    };
    let highlights = match opt.data_format {
        DataFormat::Json => json!(highlight).to_string().as_bytes().to_vec(),
        DataFormat::Tsv => {
            let mut writer = WriterBuilder::new().delimiter(b'\t').from_writer(vec![]);
            writer.serialize(highlight)?;
            writer.into_inner()?
        }
    };
    if let Some(out_path) = &opt.output {
        let bam_path = opt.bam_path.unwrap();
        let bam_file_name = bam_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".bam")
            .unwrap();
        let highlight_path = if opt.highlight.is_some() {
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
        )?;
    } else if let (Some(spec_output), Some(ref_data_output), Some(read_data_output)) = (
        &opt.spec_output,
        &opt.ref_data_output,
        &opt.read_data_output,
    ) {
        write_files(
            json!(plot_specs).to_string().as_bytes(),
            &reference,
            &reads,
            &highlights,
            spec_output,
            ref_data_output,
            read_data_output,
            opt.highlight_data_output,
        )?;
    } else {
        plot_specs["datasets"]["reference"] = json!(reference_data);
        plot_specs["datasets"]["reads"] = json!(read_data);
        plot_specs["datasets"]["highlight"] = json!(highlight);
        if opt.html {
            let mut templates = Tera::default();
            templates.add_raw_template("plot", include_str!("../resources/plot.html.tera"))?;
            let mut context = Context::new();
            context.insert("spec", &plot_specs.to_string());
            context.insert(
                "vega",
                &reqwest::get("https://cdn.jsdelivr.net/npm/vega@5")
                    .await?
                    .text()
                    .await?,
            );
            context.insert(
                "vegalite",
                &reqwest::get("https://cdn.jsdelivr.net/npm/vega-lite@5")
                    .await?
                    .text()
                    .await?,
            );
            context.insert(
                "vegaembed",
                &reqwest::get("https://cdn.jsdelivr.net/npm/vega-embed@6")
                    .await?
                    .text()
                    .await?,
            );
            let html = templates.render("plot", &context)?;
            stdout().write_all(html.as_bytes())?;
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
    spec_path: &Path,
    ref_path: &Path,
    read_path: &Path,
    highlight_path: Option<PathBuf>,
) -> Result<()> {
    let mut specs = File::create(spec_path).unwrap();
    specs.write_all(spec_data)?;
    let mut read_file = File::create(read_path).unwrap();
    read_file.write_all(read_data)?;
    let mut reference_file = File::create(ref_path).unwrap();
    reference_file.write_all(ref_data)?;
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
            Path::new("/tmp/test_spec.json"),
            Path::new("/tmp/test_ref.json"),
            Path::new("/tmp/test_read.json"),
            Some(PathBuf::from("/tmp/test_highlight.json")),
        )
        .unwrap();
        assert!(Path::new("/tmp/test_spec.json").exists());
        assert!(Path::new("/tmp/test_ref.json").exists());
        assert!(Path::new("/tmp/test_read.json").exists());
        assert!(Path::new("/tmp/test_highlight.json").exists());
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
