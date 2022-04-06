mod cli;
mod data;

use crate::data::create_plot_data;
use anyhow::Result;
use structopt::StructOpt;

fn main() -> Result<()> {
    let opt = cli::Bamboo::from_args();
    let data = create_plot_data(opt.bam_path, opt.reference, opt.region, opt.max_read_depth)?;
    Ok(())
}
