mod cli;
mod plot;

use crate::cli::Interval;
use crate::plot::create_plot_data;
use anyhow::Result;
use serde_json::{json, Value};
use std::cmp::min;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::{Path, PathBuf};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = cli::Alignoth::from_args();
    let (read_data, reference_data, plot_depth) = create_plot_data(
        &opt.bam_path,
        &opt.reference,
        &opt.region,
        opt.max_read_depth,
    )?;
    let mut plot_specs: Value = serde_json::from_str(include_str!("../resources/plot.vl.json"))?;
    plot_specs["height"] = json!(8 + 8 * plot_depth);
    plot_specs["width"] = json!(min(opt.max_width, 5 * (opt.region.length())));
    plot_specs["encoding"]["x"]["scale"]["domain"] = json!(vec![
        opt.region.start as f32 - 0.5,
        opt.region.end as f32 - 0.5
    ]);
    if let Some(out_path) = &opt.output {
        let bam_file_name = &opt
            .bam_path
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .strip_suffix(".bam")
            .unwrap();
        let highlight_path = if opt.highlight.is_some() {
            Some(Path::join(
                out_path,
                format!("{bam_file_name}.highlight.json"),
            ))
        } else {
            None
        };
        write_files(
            plot_specs.to_string().as_bytes(),
            reference_data.to_string().as_bytes(),
            read_data.to_string().as_bytes(),
            opt.highlight,
            &Path::join(out_path, format!("{bam_file_name}.vl.json")),
            &Path::join(out_path, format!("{bam_file_name}.reference.json")),
            &Path::join(out_path, format!("{bam_file_name}.reads.json")),
            highlight_path,
        )?;
        if let Some(highlight) = opt.highlight {
            let mut highlight_file = File::create(Path::join(
                out_path,
                format!("{bam_file_name}.highlight.json"),
            ))
            .unwrap();
            highlight_file.write_all(json!(vec![highlight]).to_string().as_bytes())?;
        }
    } else if let (Some(spec_output), Some(ref_data_output), Some(read_data_output)) = (
        &opt.spec_output,
        &opt.ref_data_output,
        &opt.read_data_output,
    ) {
        write_files(
            plot_specs.to_string().as_bytes(),
            reference_data.to_string().as_bytes(),
            read_data.to_string().as_bytes(),
            opt.highlight,
            spec_output,
            ref_data_output,
            read_data_output,
            opt.highlight_data_output,
        )?;
    } else {
        plot_specs["datasets"]["reference"] = reference_data;
        plot_specs["datasets"]["reads"] = read_data;
        stdout().write_all(plot_specs.to_string().as_bytes())?;
    }

    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn write_files(
    spec_data: &[u8],
    ref_data: &[u8],
    read_data: &[u8],
    highlight_data: Option<Interval>,
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
    if let (Some(data), Some(path)) = (highlight_data, highlight_path) {
        let mut highlight_file = File::create(path).unwrap();
        highlight_file.write_all(json!(vec![data]).to_string().as_bytes())?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::write_files;
    use std::fs;
    use std::path::Path;

    #[test]
    fn test_write_files() {
        write_files(
            "test spec".as_bytes(),
            "test ref".as_bytes(),
            "test read".as_bytes(),
            None,
            &Path::new("/tmp/test_spec.json"),
            &Path::new("/tmp/test_ref.json"),
            &Path::new("/tmp/test_read.json"),
            None,
        )
        .unwrap();
        assert!(Path::new("/tmp/test_spec.json").exists());
        assert!(Path::new("/tmp/test_ref.json").exists());
        assert!(Path::new("/tmp/test_read.json").exists());
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
        fs::remove_file("/tmp/test_spec.json").unwrap();
        fs::remove_file("/tmp/test_ref.json").unwrap();
        fs::remove_file("/tmp/test_read.json").unwrap();
    }
}
