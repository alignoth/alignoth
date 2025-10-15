use crate::cli::Interval;
use crate::cli::Region;
use anyhow::Result;
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
