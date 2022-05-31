mod cli;
mod plot;

use crate::plot::create_plot_data;
use anyhow::Result;
use serde_json::Value;
use std::io::{stdout, Write};
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = cli::Bamboo::from_args();
    let data = create_plot_data(opt.bam_path, opt.reference, opt.region, opt.max_read_depth)?;
    let mut plot_specs: Value = serde_json::from_str(include_str!("../resources/plot.vl.json"))?;
    plot_specs["data"]["values"] = data;
    stdout().write_all(plot_specs.to_string().as_bytes())?;
    Ok(())
}
