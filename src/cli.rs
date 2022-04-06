use anyhow::Context;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "A tool to create alignment plots from bam files.",
    name = "bamboo"
)]
pub struct Bamboo {
    /// BAM file to be visualized.
    #[structopt(long, short = "b", required = true, parse(from_os_str))]
    pub(crate) bam_path: PathBuf,

    /// Path to the reference fasta file.
    #[structopt(long, short = "r", required = true, parse(from_os_str))]
    pub(crate) reference: PathBuf,

    /// Chromosome and region for the visualization. Example: 2:132424-132924
    #[structopt(long, short = "g")]
    pub(crate) region: Region,

    /// Set the maximum rows of reads that will be shown in the alignment plots.
    #[structopt(long, short = "d", default_value = "500")]
    pub(crate) max_read_depth: usize,
}

#[derive(Debug, Clone)]
pub struct Region {
    pub(crate) target: String,
    pub(crate) start: i64,
    pub(crate) end: i64,
}

impl FromStr for Region {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, range) = s.split_once(':').context("No ':' in region string")?;
        let (start, end) = range.split_once('-').context("No '-' in region string")?;
        let start = start.parse::<i64>()?;
        let end = end.parse::<i64>()?;
        Ok(Region {
            target: target.into(),
            start,
            end,
        })
    }
}
