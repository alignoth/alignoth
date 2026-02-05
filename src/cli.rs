use crate::utils::{get_fasta_length, get_ref_and_bam_from_cwd};
use anyhow::{anyhow, Context, Result};
use log::warn;
use rust_htslib::bam;
use rust_htslib::bam::{FetchDefinition, Read};
use rust_htslib::bcf::{Read as BCFRead, Reader};
use serde::Deserialize;
use serde::Serialize;
use std::cmp;
use std::fmt::Display;
use std::path::{Path, PathBuf};
use std::str::FromStr;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    about = "A tool to create alignment plots from bam files.",
    name = "alignoth"
)]
pub struct Alignoth {
    /// BAM file to be visualized.
    #[structopt(long, short = "b", parse(from_os_str))]
    pub(crate) bam_path: Option<PathBuf>,

    /// Path to the reference fasta file.
    #[structopt(long, short = "r", parse(from_os_str))]
    pub(crate) reference: Option<PathBuf>,

    /// Chromosome and region (1-based, fully inclusive) for the visualization. Example: 2:132424-132924
    #[structopt(long, short = "g")]
    pub(crate) region: Option<Region>,

    /// Chromosome and single base for the visualization. The plotted region will start 500bp before and end 500bp after the given base. Example: 2:20000
    #[structopt(long, short = "a")]
    pub(crate) around: Option<Around>,

    /// Plots a region around a specified VCF record taken via its index from the VCF file given via the --vcf option.
    #[structopt(long, conflicts_with_all = &["around", "region", "plot_all"], requires("vcf"))]
    pub(crate) around_vcf_record: Option<u64>,

    /// A short command to plot the whole bam file. We advise to only use this command for small bam files.
    #[structopt(long)]
    pub(crate) plot_all: bool,

    /// Interval or single base position that will be highlighted in the visualization. Example: 132440-132450 or 132440
    #[structopt(long, short = "h")]
    pub(crate) highlight: Option<Vec<Interval>>,

    /// Path to a VCF file that will be used to highlight all variant position located within the given region.
    #[structopt(long, short = "v", parse(from_os_str))]
    pub(crate) vcf: Option<PathBuf>,

    /// Path to a BED file that will be used to highlight all BED records overlapping the given region.
    #[structopt(long, parse(from_os_str))]
    pub(crate) bed: Option<PathBuf>,

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

    /// If present coverage data will be written to the given file path
    #[structopt(long, parse(from_os_str), conflicts_with("output"))]
    pub(crate) coverage_output: Option<PathBuf>,

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

    /// Displays the given content of the aux tags in the tooltip of the plot. Multiple usage for more than one tag is possible.
    #[structopt(long, short = "x")]
    pub(crate) aux_tag: Option<Vec<String>>,

    /// If present, the generated html will not embed javscript dependencies and therefore be considerably smaller but require internet access to load the dependencies.
    #[structopt(long, conflicts_with("output"))]
    pub(crate) no_embed_js: bool,

    /// The minimum percentage of mismatches compared to total read depth at that point to display in the coverage plot.
    #[structopt(long, default_value = "1.0")]
    pub(crate) mismatch_display_min_percent: f64,
}

pub(crate) trait Preprocess {
    fn preprocess(&mut self) -> anyhow::Result<()>;
}

impl Preprocess for Alignoth {
    fn preprocess(&mut self) -> anyhow::Result<()> {
        if self.region.is_some() && self.around.is_some() && !self.plot_all {
            return Err(anyhow!(
                "You can only specify either a region or a base to plot around."
            ));
        }
        if self.region.is_none()
            && self.around.is_none()
            && !self.plot_all
            && self.around_vcf_record.is_none()
        {
            return Err(anyhow!(
                "You have to specify either a region or a base to plot around or use the --plot-all or --around-vcf-record option."
            ));
        }
        if self.bam_path.is_none() && self.reference.is_none() {
            if let Some(files) = get_ref_and_bam_from_cwd()? {
                self.reference = Some(files.0);
                self.bam_path = Some(files.1);
            } else {
                return Err(anyhow!(
                    "Could not find single reference and single bam file in current working directory. Please use the -r and -b flags to specify the reference and bam file."
                ));
            }
        }
        if self.bam_path.is_none() {
            return Err(anyhow!(
                "Missing bam file. Please use the -b flag to specify the bam file."
            ));
        }
        if self.reference.is_none() {
            return Err(anyhow!(
                "Missing reference file. Please use the -r flag to specify the reference file."
            ));
        }
        if self.plot_all {
            warn!("You are using the --plot-all option. This is not recommended for large bam files or files with multiple targets.");
            self.region = Some(Region::from_bam(self.bam_path.as_ref().unwrap())?);
        }
        if let Some(around) = &self.around {
            self.region = Some(Region::from_around(around));
            let target = self.region.as_ref().unwrap().target.clone();
            let target_length =
                get_fasta_length(self.reference.as_ref().unwrap(), &target).unwrap() as i64;
            let region = self.region.as_mut().unwrap();
            self.region = Some(region.clamp(0, target_length - 1));
        } else if let Some(vcf_record_index) = &self.around_vcf_record {
            self.region = Some(Region::from_vcf_record(
                *vcf_record_index,
                self.vcf.as_ref().unwrap(),
            )?);
            let target = self.region.as_ref().unwrap().target.clone();
            let target_length =
                get_fasta_length(self.reference.as_ref().unwrap(), &target).unwrap() as i64;
            let region = self.region.as_mut().unwrap();
            self.region = Some(region.clamp(0, target_length - 1));
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

pub(crate) trait Clamp {
    fn clamp(&self, min: i64, max: i64) -> Self;
}

impl Clamp for Region {
    fn clamp(&self, min: i64, max: i64) -> Self {
        Region {
            target: self.target.clone(),
            start: cmp::max(self.start, min),
            end: cmp::min(self.end, max),
        }
    }
}

impl FromStr for Region {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (target, range) = s.split_once(':').context("No ':' in region string")?;
        let (start, end) = range.split_once('-').context("No '-' in region string")?;
        let start = start.parse::<i64>().context(format!(
            "Could not parse integer from given region start {start}"
        ))? - 1; // Compensate 1-based region specification
        let end = end.parse::<i64>().context(format!(
            "Could not parse integer from given region end {end}"
        ))?; // No compensation so the region is fully inclusive
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

pub(crate) trait FromBam {
    fn from_bam(bam_path: &Path) -> Result<Self>
    where
        Self: Sized;
}

impl FromBam for Region {
    fn from_bam(bam_path: &Path) -> Result<Self> {
        let mut bam = bam::IndexedReader::from_path(bam_path)?;
        let header = bam.header();
        let target = header.target_names()[0];
        let target = std::str::from_utf8(target)?.to_string();
        bam.fetch(FetchDefinition::All)?;
        let start = bam
            .records()
            .next()
            .context(
                "Could not find first alignment in bam file. Please specify a region with -g.",
            )??
            .pos();
        let end = bam
            .records()
            .last()
            .context(
                "Could not find last alignment in bam file. Please specify a region with -g.",
            )??
            .cigar()
            .end_pos();
        Ok(Region { target, start, end })
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
        let position = p.parse::<i64>().context(format!(
            "Could not parse integer from given base position {p}"
        ))?;
        Ok(Around {
            target: target.into(),
            position,
        })
    }
}

#[derive(Debug, Clone, Deserialize, PartialEq, Eq, Default)]
pub enum DataFormat {
    #[default]
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

    pub(crate) fn _contains(&self, pos: i64, target: &str) -> bool {
        pos >= self.start && pos <= self.end && target == self.target
    }

    pub(crate) fn overlaps(&self, start: i64, end: i64, target: &str) -> bool {
        target == self.target && start <= self.end && end >= self.start
    }

    pub(crate) fn from_vcf_record(vcf_record_index: u64, vcf: &PathBuf) -> Result<Self> {
        let mut reader = Reader::from_path(vcf)?;
        let header = reader.header().clone();
        let record = reader
            .records()
            .nth(vcf_record_index as usize)
            .context(format!(
                "Given vcf record index {vcf_record_index} not found in {}",
                vcf.display()
            ))??;
        let start = &record.pos();
        let end = &record.end();
        let target = String::from_utf8(header.rid2name(record.rid().unwrap())?.to_vec())?;
        Ok(Region {
            target,
            start: *start - 500,
            end: *end + 500,
        })
    }
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct Interval {
    pub(crate) name: String,
    pub(crate) start: f64,
    pub(crate) end: f64,
}

impl FromStr for Interval {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some((name, interval)) = s.split_once(':') {
            if let Some((start, end)) = interval.split_once('-') {
                Ok(Interval {
                    name: name.to_string(),
                    start: start.parse::<f64>().context(format!(
                        "Could not parse float from given interval start {start}"
                    ))?,
                    end: end.parse::<f64>().context(format!(
                        "Could not parse float from given interval end {end}"
                    ))?,
                })
            } else if let Ok(p) = interval.parse::<f64>() {
                Ok(Interval {
                    name: name.to_string(),
                    start: p,
                    end: p,
                })
            } else {
                Err(anyhow!(
                    "No '-' in interval string nor a single position to highlight."
                ))
            }
        } else {
            Err(anyhow!(
                "No ':' in interval string nor a single position to highlight."
            ))
        }
    }
}

impl Interval {
    pub fn new(name: String, start: f64, end: f64) -> Self {
        Self { name, start, end }
    }

    // Adjusts interval to match coordinate system of final vega-lite plot
    pub(crate) fn preprocess(&mut self) {
        self.start -= 0.5;
        self.end += 0.5;
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::{Around, DataFormat, FromAround, Interval, Region};
    use std::str::FromStr;

    #[test]
    fn test_region_deserialization() {
        let region = Region::from_str("X:2000-3000").unwrap();
        let expeceted_region = Region {
            target: "X".to_string(),
            start: 1999,
            end: 2999,
        };
        assert_eq!(region, expeceted_region);
    }

    #[test]
    fn test_interval_deserialization() {
        let interval = Interval::from_str("test:2000-3000").unwrap();
        let expeceted_interval = Interval {
            name: "test".to_string(),
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
    fn test_around_deserialization() {
        let around = Around::from_str("X:2000").unwrap();
        let expeceted_around = Around {
            target: "X".to_string(),
            position: 2000,
        };
        assert_eq!(around, expeceted_around);
    }

    #[test]
    fn test_region_from_around() {
        let around = Around::from_str("X:2000").unwrap();
        let region = Region::from_around(&around);
        let expeceted_region = Region {
            target: "X".to_string(),
            start: 1500,
            end: 2500,
        };
        assert_eq!(region, expeceted_region);
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
