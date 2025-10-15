use crate::cli::Interval;
use crate::cli::Region;
use anyhow::Result;
use bio::io::bed;
use rust_htslib::bcf::{Read, Reader};
use std::path::PathBuf;

pub(crate) trait Highlight {
    // Returns a vector of intervals that highlight the given region. Excludes any intervals outside the given region.
    fn intervals(&self, region: &Region) -> Result<Vec<Interval>>;
}

pub(crate) struct VcfHighlight {
    pub path: PathBuf,
}

impl VcfHighlight {
    pub fn new(path: PathBuf) -> Self {
        VcfHighlight { path }
    }
}

impl Highlight for VcfHighlight {
    fn intervals(&self, region: &Region) -> Result<Vec<Interval>> {
        let mut reader = Reader::from_path(&self.path)?;
        let header = reader.header().clone();
        let mut intervals = Vec::new();
        for record in reader.records() {
            let record = record?;
            let target =
                String::from_utf8(header.rid2name(record.rid().unwrap()).unwrap().to_vec())?;
            if region.contains(record.pos(), &target) {
                let start = record.pos();
                let alleles = record.alleles();
                let end = start + alleles[0].len() as i64;
                let id = {
                    if record.id() != b"." {
                        String::from_utf8(record.id().to_vec())?
                    } else {
                        let ref_allele = std::str::from_utf8(alleles[0]).unwrap_or("?");
                        let alt_allele = std::str::from_utf8(alleles[1]).unwrap_or("?");
                        format!("{}:{}{}>{}", target, start, ref_allele, alt_allele)
                    }
                };
                intervals.push(Interval::new(id, start as f64, end as f64));
            }
        }
        Ok(intervals)
    }
}

pub(crate) struct BedHighlight {
    pub path: PathBuf,
}

impl BedHighlight {
    pub fn new(path: PathBuf) -> Self {
        BedHighlight { path }
    }
}

impl Highlight for BedHighlight {
    fn intervals(&self, region: &Region) -> Result<Vec<Interval>> {
        let mut intervals = Vec::new();
        let mut reader = bed::Reader::from_file(&self.path)?;
        for record in reader.records() {
            let record = record?;
            if region.overlaps(record.start() as i64, record.end() as i64, record.chrom()) {
                let id = if let Some(name) = record.name() {
                    name.to_string()
                } else {
                    format!("{}:{}-{}", record.chrom(), record.start(), record.end())
                };
                intervals.push(Interval::new(
                    id,
                    record.start() as f64,
                    record.end() as f64,
                ));
            }
        }
        Ok(intervals)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    #[test]
    fn test_vcf_intervals() {
        let highlight = VcfHighlight::new(PathBuf::from("tests/sample_3/1:257A.vcf"));
        let region = Region::from_str("1:200-300").unwrap();
        let intervals = highlight.intervals(&region).unwrap();
        assert_eq!(intervals.len(), 1);
        assert_eq!(intervals[0].name, "1:257A>G");
        assert_eq!(intervals[0].start, 257.0);
        assert_eq!(intervals[0].end, 258.0);
    }

    #[test]
    fn test_vcf_interval_out_of_bounds() {
        let highlight = VcfHighlight::new(PathBuf::from("tests/sample_3/1:257A.vcf"));
        let region = Region::from_str("1:200-220").unwrap();
        let intervals = highlight.intervals(&region).unwrap();
        assert_eq!(intervals.len(), 0);
    }

    #[test]
    fn test_bed_intervals() {
        let highlight = BedHighlight::new(PathBuf::from("tests/sample_3/test.bed"));
        let region = Region::from_str("1:200-300").unwrap();
        let intervals = highlight.intervals(&region).unwrap();
        assert_eq!(intervals.len(), 1);
        assert_eq!(intervals[0].name, "MVSTP1");
        assert_eq!(intervals[0].start, 260.0);
        assert_eq!(intervals[0].end, 300.0);
    }
}
