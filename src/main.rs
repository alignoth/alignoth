mod cli;
mod plot;

use crate::plot::create_plot_data;
use anyhow::Result;
use serde_json::{json, Value};
use std::io::{stdout, Write};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = cli::Bamboo::from_args();
    let (read_data, reference_data) =
        create_plot_data(opt.bam_path, opt.reference, &opt.region, opt.max_read_depth)?;
    let mut plot_specs: Value = serde_json::from_str(include_str!("../resources/plot.vl.json"))?;
    plot_specs["datasets"]["reference"] = reference_data;
    plot_specs["datasets"]["reads"] = read_data;
    plot_specs["encoding"]["x"]["scale"]["domain"] = json!(vec![
        opt.region.start as f32 - 0.5,
        opt.region.end as f32 - 0.5
    ]);
    stdout().write_all(plot_specs.to_string().as_bytes())?;
    Ok(())
}
