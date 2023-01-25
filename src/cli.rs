use anyhow::{anyhow, Context};
use serde::Deserialize;
use serde::Serialize;
use std::fmt::Display;
use std::path::PathBuf;
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "A tool to create alignment plots from bam files.",
    name = "alignoth"
)]
pub struct Alignoth {
    /// BAM file to be visualized.
    #[structopt(long, short = "b", required = true, parse(from_os_str))]
    pub(crate) bam_path: PathBuf,

    /// Path to the reference fasta file.
    #[structopt(long, short = "r", required = true, parse(from_os_str))]
    pub(crate) reference: PathBuf,

    /// Chromosome and region for the visualization. Example: 2:132424-132924
    #[structopt(long, short = "g")]
    pub(crate) region: Option<Region>,

    /// Chromosome and single base for the visualization. The plotted region will start 500bp before and end 500bp after the given base. Example: 2:20000
    #[structopt(long, short = "a")]
    pub(crate) around: Option<Around>,

    /// Interval that will be highlighted in the visualization. Example: 132440-132450
    #[structopt(long, short = "h")]
    pub(crate) highlight: Option<Interval>,

    /// Set the maximum rows of reads that will be shown in the alignment plots.
    #[structopt(long, short = "d", default_value = "500")]
    pub(crate) max_read_depth: usize,

    /// Set the data format of the read, reference and highlight data.
    #[structopt(long, short = "f", default_value)]
    pub(crate) data_format: DataFormat,

    /// Sets the maximum width of the resulting plot.
    #[structopt(long, short = "w", default_value = "1024")]
    pub(crate) max_width: i64,

    /// If present vega-lite specs will be written to the given file path
    #[structopt(long, parse(from_os_str), conflicts_with("output"))]
    pub(crate) spec_output: Option<PathBuf>,

    /// If present reference data will be written to the given file path
    #[structopt(long, parse(from_os_str), conflicts_with("output"))]
    pub(crate) ref_data_output: Option<PathBuf>,

    /// If present read data will be written to the given file path
    #[structopt(long, parse(from_os_str), conflicts_with("output"))]
    pub(crate) read_data_output: Option<PathBuf>,

    /// If present highlight data will be written to the given file path
    #[structopt(
        long,
        parse(from_os_str),
        conflicts_with("output"),
        requires("highlight")
    )]
    pub(crate) highlight_data_output: Option<PathBuf>,

    /// If present, data and vega-lite specs of the generated plot will be split and written to the given directory
    #[structopt(long, short = "o", parse(from_os_str))]
    pub(crate) output: Option<PathBuf>,

    /// If present, the generated plot will inserted into a plain html file containing the plot centered which is then written to stdout
    #[structopt(long, conflicts_with("output"))]
    pub(crate) html: bool,
}

pub(crate) trait Preprocess {
    fn preprocess(&mut self) -> anyhow::Result<()>;
}

impl Preprocess for Alignoth {
    fn preprocess(&mut self) -> anyhow::Result<()> {
        if self.region.is_some() && self.around.is_some() {
            return Err(anyhow!(
                "You can only specify either a region or a base to plot around."
            ));
        }
        if self.region.is_none() && self.around.is_none() {
            return Err(anyhow!(
                "You have to specify either a region or a base to plot around."
            ));
        }
        if let Some(around) = &self.around {
            self.region = Some(Region::from_around(around));
        }
        Ok(())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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

pub(crate) trait FromAround {
    fn from_around(around: &Around) -> Self;
}

impl FromAround for Region {
    fn from_around(around: &Around) -> Self {
        Region {
            target: around.target.to_string(),
            start: around.position - 500,
            end: around.position + 500,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Around {
    pub(crate) target: String,
    pub(crate) position: i64,
}

impl FromStr for Around {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, p) = s.split_once(':').context("No ':' in around string")?;
        let position = p.parse::<i64>()?;
        Ok(Around {
            target: target.into(),
            position,
        })
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub enum DataFormat {
    Json,
    Tsv,
}

impl Display for DataFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DataFormat::Json => write!(f, "json"),
            DataFormat::Tsv => write!(f, "tsv"),
        }
    }
}

impl Default for DataFormat {
    fn default() -> Self {
        DataFormat::Json
    }
}

impl FromStr for DataFormat {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "json" => Ok(DataFormat::Json),
            "tsv" => Ok(DataFormat::Tsv),
            _ => Err(anyhow!("Unknown data format: {}", s)),
        }
    }
}

impl Region {
    /// Returns the length of the Region
    pub(crate) fn length(&self) -> i64 {
        self.end - self.start
    }
}

#[derive(Debug, Clone, Serialize, PartialEq, Copy)]
pub struct Interval {
    pub(crate) start: f64,
    pub(crate) end: f64,
}

impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('-').context("No '-' in interval string")?;
        let start = start.parse::<f64>()?;
        let end = end.parse::<f64>()?;
        Ok(Interval { start, end })
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::{DataFormat, Interval, Region};
    use std::str::FromStr;

    #[test]
    fn test_region_deserialization() {
        let region = Region::from_str("X:2000-3000").unwrap();
        let expeceted_region = Region {
            target: "X".to_string(),
            start: 2000,
            end: 3000,
        };
        assert_eq!(region, expeceted_region);
    }

    #[test]
    fn test_interval_deserialization() {
        let interval = Interval::from_str("2000-3000").unwrap();
        let expeceted_interval = Interval {
            start: 2000.0,
            end: 3000.0,
        };
        assert_eq!(interval, expeceted_interval);
    }

    #[test]
    fn test_region_length() {
        let region = Region::from_str("X:2000-3000").unwrap();
        assert_eq!(region.length(), 1000);
    }

    #[test]
    fn test_data_format_deserialization() {
        let data_format = DataFormat::from_str("json").unwrap();
        assert_eq!(data_format, DataFormat::Json);
        let tsv_data_format = DataFormat::from_str("tsv").unwrap();
        assert_eq!(tsv_data_format, DataFormat::Tsv);
    }

    #[test]
    fn test_data_format_to_string() {
        let data_format = DataFormat::Json;
        assert_eq!(data_format.to_string(), "json");
        let data_format = DataFormat::Tsv;
        assert_eq!(data_format.to_string(), "tsv");
    }
}
