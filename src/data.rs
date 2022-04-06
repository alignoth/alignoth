use crate::cli::Region;
use anyhow::Result;
use bio::io::fasta;
use itertools::Itertools;
use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rust_htslib::bam;
use rust_htslib::bam::record::{Cigar, CigarString, CigarStringView};
use rust_htslib::bam::{FetchDefinition, Read as HtslibRead};
use serde::Serialize;
use serde_json::{json, Value};
use std::fmt;
use std::fmt::Display;
use std::path::Path;

pub(crate) fn create_plot_data<P: AsRef<Path> + std::fmt::Debug>(
    bam_path: P,
    ref_path: P,
    region: Region,
    max_read_depth: usize,
) -> Result<serde_json::Value> {
    let mut bam = bam::IndexedReader::from_path(&bam_path)?;
    let tid = bam.header().tid(&region.target.as_bytes()).unwrap() as i32;
    bam.fetch(FetchDefinition::Region(tid, region.start, region.end))?;
    let data: Vec<_> = bam
        .records()
        .filter_map(|r| r.ok())
        .map(|r| Read::from_record(r, &ref_path))
        .collect();
    let mut data: Vec<_> = data
        .order(max_read_depth)?
        .iter()
        .map(|r| json!(r))
        .collect();
    let mut reference_data = fetch_reference(ref_path, region)?;
    data.append(&mut reference_data);
    Ok(json!(data))
}

pub(crate) fn fetch_reference<P: AsRef<Path> + std::fmt::Debug>(
    ref_path: P,
    region: Region,
) -> Result<Vec<serde_json::Value>> {
    let mut reader = fasta::IndexedReader::from_file(&ref_path).unwrap();
    let mut seq: Vec<u8> = Vec::new();
    reader.fetch(&region.target, region.start as u64, region.end as u64)?;
    reader.read(&mut seq)?;
    Ok(seq
        .iter()
        .map(|c| Reference {
            position: 0,
            base: char::from(*c),
        })
        .map(|b| json!(b))
        .collect())
}

#[derive(Serialize, Debug)]
pub struct Read {
    cigar: PlotCigar,
    position: i64,
    flags: u16,
    mapq: u8,
    row: Option<u32>,
    #[serde(skip)]
    end_postion: i64,
}

#[derive(Serialize, Debug)]
struct Reference {
    position: u64,
    base: char,
}

#[derive(Serialize, Debug)]
struct PlotCigar(Vec<InnerPlotCigar>);

impl ToString for PlotCigar {
    fn to_string(&self) -> String {
        self.0.iter().join("")
    }
}

#[derive(Serialize, Debug)]
struct InnerPlotCigar {
    cigar_type: CigarType,
    bases: Option<Vec<char>>,
    length: Option<u32>,
}

impl Display for InnerPlotCigar {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.cigar_type {
            CigarType::Match => write!(f, "{}=", self.length.unwrap()),
            CigarType::Ins => write!(
                f,
                "i{}",
                self.bases.as_ref().unwrap().iter().collect::<String>()
            ),
            CigarType::Del => write!(f, "{}d", self.length.unwrap()),
            CigarType::Sub => write!(
                f,
                "{}{}",
                self.length.unwrap(),
                self.bases.as_ref().unwrap().iter().collect::<String>()
            ),
        }
    }
}

#[derive(Serialize, Debug)]
enum CigarType {
    Match,
    Ins,
    Del,
    Sub,
}

impl PlotCigar {
    fn from_cigar(cigar: CigarStringView, ref_seq: Vec<char>) {
        for c in &cigar {
            match c {
                Cigar::Match(_) => {}
                Cigar::Ins(_) => {}
                Cigar::Del(_) => {}
                Cigar::RefSkip(_) => {}
                Cigar::SoftClip(_) => {}
                Cigar::HardClip(_) => {}
                Cigar::Pad(_) => {}
                Cigar::Equal(_) => {}
                Cigar::Diff(_) => {}
            }
        }
        unimplemented!()
    }
}

impl Read {
    fn from_record<P: AsRef<Path>>(record: rust_htslib::bam::record::Record, ref_path: P) -> Read {
        unimplemented!()
    }

    fn set_row(&mut self, row: u32) {
        self.row = Some(row);
    }
}

pub trait PlotOrder {
    fn order(&self, max_read_depth: usize) -> Result<Vec<Read>>;
}

impl PlotOrder for Vec<Read> {
    fn order(&self, max_read_depth: usize) -> Result<Vec<Read>> {
        unimplemented!()
    }
}
