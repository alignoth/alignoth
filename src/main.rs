mod cli;
mod plot;

use crate::plot::create_plot_data;
use anyhow::Result;
use serde_json::{json, Value};
use std::cmp::min;
use std::fs::File;
use std::io::{stdout, Write};
use std::path::Path;
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
        let mut specs =
            File::create(Path::join(out_path, format!("{bam_file_name}.vl.json"))).unwrap();
        specs.write_all(plot_specs.to_string().as_bytes())?;
        let mut read_file =
            File::create(Path::join(out_path, format!("{bam_file_name}.reads.json"))).unwrap();
        read_file.write_all(read_data.to_string().as_bytes())?;
        let mut reference_file = File::create(Path::join(
            out_path,
            format!("{bam_file_name}.reference.json"),
        ))
        .unwrap();
        reference_file.write_all(reference_data.to_string().as_bytes())?;
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
        let mut specs = File::create(spec_output).unwrap();
        specs.write_all(plot_specs.to_string().as_bytes())?;
        let mut read_file = File::create(read_data_output).unwrap();
        read_file.write_all(read_data.to_string().as_bytes())?;
        let mut reference_file = File::create(ref_data_output).unwrap();
        reference_file.write_all(reference_data.to_string().as_bytes())?;
        if let (Some(highlight), Some(highlight_output)) =
            (opt.highlight, opt.highlight_data_output)
        {
            let mut highlight_file = File::create(highlight_output).unwrap();
            highlight_file.write_all(json!(vec![highlight]).to_string().as_bytes())?;
        }
    } else {
        plot_specs["datasets"]["reference"] = reference_data;
        plot_specs["datasets"]["reads"] = read_data;
        stdout().write_all(plot_specs.to_string().as_bytes())?;
    }

    Ok(())
}
