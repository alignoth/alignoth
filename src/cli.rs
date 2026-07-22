use crate::utils::{
    ensure_bam_index, ensure_fasta_index, get_fasta_length, get_ref_and_bam_from_cwd,
};
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
    /// BAM files to be visualized.
    #[structopt(long, short = "b", parse(from_os_str))]
    pub(crate) bam_path: Vec<PathBuf>,

    /// Path to the reference fasta file.
    #[structopt(long, short = "r", parse(from_os_str))]
    pub(crate) reference: Option<PathBuf>,

    /// Chromosome and region (1-based, fully inclusive) for the visualization. Example: 2:132424-132924
    #[structopt(long, short = "g")]
    pub(crate) region: Option<Region>,

    /// Chromosome and single base for the visualization. The plotted region will start 500bp before and end 500bp after the given base. Example: 2:20000
    #[structopt(long, short = "a")]
    pub(crate) around: Option<Around>,

    /// Plots a region around a specified VCF record taken via its index (starting at 0) from the VCF file given via the --vcf option.
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

    /// Sets the maximum width of the resulting plot. Defaults to 1024, or to the available width when rendering to HTML.
    #[structopt(long, short = "w")]
    pub(crate) max_width: Option<i64>,

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

    /// If set, reads are clamped to the boundaries of the specified region before processing.
    #[structopt(long)]
    pub(crate) clamp_reads: bool,
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
        if self.bam_path.is_empty() && self.reference.is_none() {
            if let Some((reference, bams)) = get_ref_and_bam_from_cwd()? {
                self.reference = Some(reference);
                self.bam_path = bams;
            } else {
                return Err(anyhow::anyhow!(
                    "Could not unambiguously find a single reference and at least one bam file in the current working directory. Please use the -r and -b flags to specify the reference and bam files."
                ));
            }
        }
        if self.bam_path.is_empty() {
            return Err(anyhow!(
                "Missing bam file. Please use the -b flag to specify the bam file."
            ));
        }
        if self.reference.is_none() {
            return Err(anyhow!(
                "Missing reference file. Please use the -r flag to specify the reference file."
            ));
        }
        for bam in &self.bam_path {
            ensure_bam_index(bam)?;
        }
        ensure_fasta_index(self.reference.as_ref().unwrap())?;
        if self.plot_all {
            warn!("You are using the --plot-all option. This is not recommended for large bam files or files with multiple targets.");
            let mut min_start = i64::MAX;
            let mut max_end = i64::MIN;
            let mut target = String::new();
            for bam in &self.bam_path {
                let r = Region::from_bam(bam)?;
                if target.is_empty() {
                    target = r.target.clone();
                } else if target != r.target {
                    return Err(anyhow::anyhow!("Cannot use --plot-all with multiple BAM files that have different targets."));
                }
                min_start = std::cmp::min(min_start, r.start);
                max_end = std::cmp::max(max_end, r.end);
            }
            self.region = Some(Region {
                target,
                start: min_start,
                end: max_end,
            });
        }
        if let Some(around) = &self.around {
            self.region = Some(Region::from_around(around));
        } else if let Some(vcf_record_index) = &self.around_vcf_record {
            self.region = Some(Region::from_vcf_record(
                *vcf_record_index,
                self.vcf.as_ref().unwrap(),
            )?);
        }
        let region = self.region.as_ref().unwrap();
        let target_length =
            get_fasta_length(self.reference.as_ref().unwrap(), &region.target)? as i64;
        self.region = Some(region.clamp(0, target_length));
        Ok(())
    }
}

impl Alignoth {
    /// Renders the non-interactive `alignoth` command that reproduces this configuration.
    pub(crate) fn to_command(&self) -> String {
        let mut args = vec!["alignoth".to_string()];
        for bam in &self.bam_path {
            args.push("-b".to_string());
            args.push(quote(&bam.display().to_string()));
        }
        if let Some(reference) = &self.reference {
            args.push("-r".to_string());
            args.push(quote(&reference.display().to_string()));
        }
        if let Some(region) = &self.region {
            args.push("-g".to_string());
            args.push(quote(&region.to_string()));
        }
        for highlight in self.highlight.iter().flatten() {
            args.push("-h".to_string());
            args.push(quote(&highlight.to_string()));
        }
        if let Some(vcf) = &self.vcf {
            args.push("-v".to_string());
            args.push(quote(&vcf.display().to_string()));
        }
        if let Some(bed) = &self.bed {
            args.push("--bed".to_string());
            args.push(quote(&bed.display().to_string()));
        }
        for tag in self.aux_tag.iter().flatten() {
            args.push("-x".to_string());
            args.push(quote(tag));
        }
        if self.max_read_depth != 500 {
            args.push("-d".to_string());
            args.push(self.max_read_depth.to_string());
        }
        if self.html {
            args.push("--html".to_string());
        }
        args.join(" ")
    }
}

/// Wraps `value` in single quotes if a shell would otherwise split or drop it.
fn quote(value: &str) -> String {
    if value.is_empty() || value.contains(char::is_whitespace) {
        format!("'{value}'")
    } else {
        value.to_string()
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

impl Display for Region {
    /// Formats the region 1-based and fully inclusive, matching the `--region` input syntax.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}-{}", self.target, self.start + 1, self.end)
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

impl Display for Interval {
    /// Formats the interval matching the `--highlight` input syntax (`name:start-end` or `name:pos`).
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.start == self.end {
            write!(f, "{}:{}", self.name, self.start)
        } else {
            write!(f, "{}:{}-{}", self.name, self.start, self.end)
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
    use crate::cli::{Alignoth, Around, DataFormat, FromAround, Interval, Preprocess, Region};
    use std::path::PathBuf;
    use std::str::FromStr;

    fn base_alignoth() -> Alignoth {
        Alignoth {
            bam_path: vec![PathBuf::from("sample.bam")],
            reference: Some(PathBuf::from("ref.fa")),
            region: Some(Region {
                target: "chr1".to_string(),
                start: 999,
                end: 2000,
            }),
            around: None,
            around_vcf_record: None,
            plot_all: false,
            highlight: None,
            vcf: None,
            bed: None,
            max_read_depth: 500,
            data_format: DataFormat::Json,
            max_width: Some(1024),
            spec_output: None,
            ref_data_output: None,
            read_data_output: None,
            coverage_output: None,
            highlight_data_output: None,
            output: None,
            html: false,
            aux_tag: None,
            no_embed_js: false,
            mismatch_display_min_percent: 1.0,
            clamp_reads: false,
        }
    }

    fn preprocessed_region(region: Option<Region>, around: Option<Around>) -> Region {
        let mut opt = Alignoth {
            bam_path: vec![PathBuf::from("tests/sample_1/reads.bam")],
            reference: Some(PathBuf::from("tests/sample_1/reference.fa")),
            region,
            around,
            ..base_alignoth()
        };
        opt.preprocess().unwrap();
        opt.region.unwrap()
    }

    #[test]
    fn test_preprocess_clamps_region_to_target_bounds() {
        let clamped = preprocessed_region(Some(Region::from_str("chr1:110-140").unwrap()), None);
        assert_eq!((clamped.start, clamped.end), (109, 123));

        let whole = preprocessed_region(Some(Region::from_str("chr1:1-123").unwrap()), None);
        assert_eq!((whole.start, whole.end), (0, 123));

        let around = preprocessed_region(None, Some(Around::from_str("chr1:100").unwrap()));
        assert_eq!((around.start, around.end), (0, 123));
    }

    #[test]
    fn test_to_command_minimal() {
        assert_eq!(
            base_alignoth().to_command(),
            "alignoth -b sample.bam -r ref.fa -g chr1:1000-2000"
        );
    }

    #[test]
    fn test_to_command_full() {
        let opt = Alignoth {
            highlight: Some(vec![Interval::new("var".to_string(), 1200.0, 1200.0)]),
            vcf: Some(PathBuf::from("variants.vcf.gz")),
            bed: Some(PathBuf::from("regions.bed")),
            aux_tag: Some(vec!["HP".to_string(), "PS".to_string()]),
            max_read_depth: 200,
            html: true,
            ..base_alignoth()
        };
        assert_eq!(
            opt.to_command(),
            "alignoth -b sample.bam -r ref.fa -g chr1:1000-2000 -h var:1200 -v variants.vcf.gz --bed regions.bed -x HP -x PS -d 200 --html"
        );
    }

    #[test]
    fn test_region_deserialization() {
        let region = Region::from_str("X:2000-3000").unwrap();
        let expeceted_region = Region {
            target: "X".to_string(),
            start: 1999,
            end: 3000,
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
        assert_eq!(region.length(), 1001);
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
