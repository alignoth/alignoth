use crate::cli::Region;
use anyhow::Result;
use bio::io::fasta;
use rand::prelude::IteratorRandom;
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rust_htslib::bam;
use rust_htslib::bam::{FetchDefinition, Read as HtslibRead};
use serde::Serialize;
use serde_json::{json, Value};
use std::path::Path;

pub(crate) fn create_plot_data<P: AsRef<Path> + std::fmt::Debug>(
    bam_path: P,
    ref_path: P,
    region: Region,
    max_reads: usize,
) -> Result<serde_json::Value> {
    let mut bam = bam::IndexedReader::from_path(&bam_path)?;
    let tid = bam.header().tid(&region.target.as_bytes()).unwrap() as i32;
    bam.fetch(FetchDefinition::Region(tid, region.start, region.end))?;
    let mut data: Vec<_> = reads_from_records(bam.records().filter_map(|r| r.ok()).collect(), &ref_path);
    if data.len() > max_reads {
        let mut rng = StdRng::seed_from_u64(42);
        data = data
            .into_iter()
            .choose_multiple(&mut rng, max_reads)
            .into_iter()
            .collect();
    }
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
struct Read {
    cigar: String,
    position: u64,
    flags: u16,
    mapq: u8,
}

#[derive(Serialize, Debug)]
struct Reference {
    position: u64,
    base: char,
}


fn reads_from_records<P: AsRef<Path>>(records: Vec<rust_htslib::bam::record::Record>, ref_path: P) -> Vec<Value> {
    unimplemented!()
}

