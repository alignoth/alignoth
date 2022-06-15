use std::cmp::{max, min};
use crate::cli;
use crate::cli::Region;
use anyhow::{Context, Result};
use bio::io::fasta;
use itertools::Itertools;
use rand::prelude::IteratorRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rust_htslib::bam;
use rust_htslib::bam::record::{Cigar, CigarStringView};
use rust_htslib::bam::FetchDefinition::Region as FetchRegion;
use rust_htslib::bam::Read as HtslibRead;
use serde::{Serialize, Serializer};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::fmt::Display;
use std::path::Path;

/// Generates the plot data for a given region of a bam file
pub(crate) fn create_plot_data<P: AsRef<Path> + std::fmt::Debug>(
    bam_path: P,
    ref_path: P,
    region: &Region,
    max_read_depth: usize,
) -> Result<(serde_json::Value, serde_json::Value, u32)> {
    let mut bam = bam::IndexedReader::from_path(&bam_path)?;
    let tid = bam.header().tid(region.target.as_bytes()).unwrap() as i32;
    bam.fetch(FetchRegion(tid, region.start, region.end))?;
    let mut data: Vec<_> = bam
        .records()
        .filter_map(|r| r.ok())
        .map(|r| Read::from_record(r, &ref_path, &region.target).unwrap())
        .collect();
    data.order(max_read_depth)?;
    let read_depth = data.iter().map(|r| r.row.unwrap()).max().unwrap();
    let data: Vec<_> = data.iter().map(|r| json!(r)).collect();
    let reference_data = Reference {
        start: region.start,
        reference: read_fasta(ref_path, region)?.iter().collect(),
    };
    Ok((json!(data), json!(reference_data), read_depth))
}

/// Reads the given region from the given fasta file and returns it as a vec of the bases as chars
fn read_fasta<P: AsRef<Path> + std::fmt::Debug>(path: P, region: &Region) -> Result<Vec<char>> {
    let mut reader = fasta::IndexedReader::from_file(&path).unwrap();
    let index =
        fasta::Index::with_fasta_file(&path).context("error reading index file of input FASTA")?;
    let _sequences = index.sequences();

    let mut seq: Vec<u8> = Vec::new();

    reader.fetch(&region.target, region.start as u64, region.end as u64)?;
    reader.read(&mut seq)?;

    Ok(seq.iter().map(|u| char::from(*u)).collect_vec())
}

/// A Read containing all relevant information for being plotted in a read plot
#[derive(Serialize, Debug)]
pub struct Read {
    name: String,
    cigar: PlotCigar,
    position: i64,
    flags: u16,
    mapq: u8,
    row: Option<u32>,
    #[serde(skip)]
    end_position: i64,
    #[serde(skip)]
    mpos: i64,
}

/// A reference with all relevant information base for being plotted in a read plot
#[derive(Serialize, Debug, Eq, PartialEq)]
pub(crate) struct Reference {
    start: i64,
    reference: String,
}

/// A more detailed version of a CigarString with all relevant information base for being plotted in a read plot.
///
/// | Cigar         | Syntax          |
/// |---------------|-----------------|
/// | Match         | `<#matches>=`   |
/// | Deletion      | `<#deletions>d` |
/// | Substitutions | `<#><base>`     |
/// | Insertions    | `i<bases>`      |
///
/// Example: `50=|3d|10=|1C|1G|iGGT`
#[derive(Debug, Eq, PartialEq)]
struct PlotCigar(Vec<InnerPlotCigar>);

impl Serialize for PlotCigar {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl ToString for PlotCigar {
    fn to_string(&self) -> String {
        self.0.iter().join("|")
    }
}

#[derive(Serialize, Debug, Eq, PartialEq)]
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

#[derive(Serialize, Debug, Eq, PartialEq)]
enum CigarType {
    Match,
    Ins,
    Del,
    Sub,
}

impl PlotCigar {
    /// Creates a detailed PlotCigar from a given rust_htslib CigarStringView
    fn from_cigar(
        cigar: CigarStringView,
        read_seq: Vec<char>,
        ref_seq: Vec<char>,
    ) -> Result<PlotCigar> {
        let mut inner_plot_cigars = Vec::new();
        let (mut read_index, mut ref_index) = (0, 0);
        for c in &cigar {
            match c {
                Cigar::Match(length) | Cigar::SoftClip(length) => {
                    inner_plot_cigars.extend(match_bases(
                        &read_seq[read_index..read_index + *length as usize],
                        &ref_seq[ref_index..ref_index + *length as usize],
                    ));
                    read_index += *length as usize;
                    ref_index += *length as usize;
                }
                Cigar::Ins(length) => {
                    inner_plot_cigars.push(InnerPlotCigar {
                        cigar_type: CigarType::Ins,
                        bases: Some(read_seq[read_index..read_index + *length as usize].to_vec()),
                        length: Some(*length),
                    });
                    read_index += *length as usize;
                }
                Cigar::Del(length) => {
                    inner_plot_cigars.push(InnerPlotCigar {
                        cigar_type: CigarType::Del,
                        bases: None,
                        length: Some(*length),
                    });
                    ref_index += *length as usize;
                }
                _ => {}
            }
        }
        Ok(PlotCigar(inner_plot_cigars))
    }
}

/// Matches a given read sequence against a given reference sequence and returning the result as Vec<InnerPlotCigar>
fn match_bases(read_seq: &[char], ref_seq: &[char]) -> Vec<InnerPlotCigar> {
    let mut inner_plot_cigars = Vec::new();
    for (is_match, group) in &read_seq
        .iter()
        .zip_eq(ref_seq.iter())
        .group_by(|(read, reference)| read == reference)
    {
        if is_match {
            inner_plot_cigars.push(InnerPlotCigar {
                cigar_type: CigarType::Match,
                bases: None,
                length: Some(group.count() as u32),
            });
        } else {
            let substitutions = group.into_iter().map(|(r, _)| *r).collect_vec();
            for (length, base) in substitutions.iter().dedup_with_count() {
                inner_plot_cigars.push(InnerPlotCigar {
                    cigar_type: CigarType::Sub,
                    bases: Some(vec![*base]),
                    length: Some(length as u32),
                })
            }
        };
    }
    inner_plot_cigars
}

impl Read {
    /// Creates a Read from a given rust_htslib bam record
    fn from_record<P: AsRef<Path> + std::fmt::Debug>(
        record: rust_htslib::bam::record::Record,
        ref_path: P,
        target: &str,
    ) -> Result<Read> {
        let region = cli::Region {
            target: target.to_string(),
            start: record.pos() - record.cigar().leading_softclips(),
            end: record.pos() + record.seq_len() as i64,
        };
        let ref_seq = read_fasta(ref_path, &region)?;
        let read_seq = record
            .seq()
            .as_bytes()
            .iter()
            .map(|u| char::from(*u))
            .collect_vec();
        Ok(Read {
            name: String::from_utf8(record.qname().to_vec())?,
            cigar: PlotCigar::from_cigar(record.cigar(), read_seq, ref_seq)?,
            position: record.pos(),
            flags: record.flags(),
            mapq: record.mapq(),
            row: None,
            end_position: record.pos() + record.seq_len() as i64,
            mpos: record.mpos(),
        })
    }

    /// Sets the row of the Read
    fn set_row(&mut self, row: u32) {
        self.row = Some(row);
    }
}

pub trait PlotOrder {
    fn order(&mut self, max_read_depth: usize) -> Result<()>;
}

impl PlotOrder for Vec<Read> {
    /// Assigns given Reads their vertical position (row) in the read plot respecting the given max_read_depth by subsampling rows.
    fn order(&mut self, max_read_depth: usize) -> Result<()> {
        let mut row_ends = vec![0; 10000];
        let mut ordered_reads = HashMap::new();
        for read in self.iter_mut() {
            if let Some(row) = ordered_reads.get(&read.name) {
                read.set_row(*row as u32);
                if row_ends[*row] < read.end_position {
                    row_ends[*row] = read.end_position;
                }
                continue;
            }
            for (row, row_end) in row_ends.iter().enumerate().take(10000).skip(1) {
                if min(read.position, read.mpos) > *row_end + 5 {
                    read.set_row(row as u32);
                    row_ends[row] = max(read.end_position, read.mpos);
                    ordered_reads.insert(&read.name, row);
                    break;
                }
            }
        }
        let used_rows = *ordered_reads.values().max().unwrap();
        if max_read_depth < used_rows {
            let mut rng = StdRng::seed_from_u64(42);
            let random_rows: HashSet<_> = (0..used_rows as u32)
                .choose_multiple(&mut rng, max_read_depth as usize)
                .into_iter()
                .collect();
            self.retain(|read| random_rows.contains(&read.row.unwrap()));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::cli::Region;
    use crate::plot::CigarType::{Del, Ins, Match, Sub};
    use crate::plot::{
        match_bases, read_fasta, CigarType, InnerPlotCigar, PlotCigar, PlotOrder, Read,
    };
    use itertools::Itertools;
    use rust_htslib::bam::record::{Cigar, CigarString, CigarStringView};

    #[test]
    fn test_plot_cigar_string_serialization() {
        let plot_cigar = PlotCigar(vec![
            InnerPlotCigar {
                cigar_type: Match,
                bases: None,
                length: Some(50),
            },
            InnerPlotCigar {
                cigar_type: Del,
                bases: None,
                length: Some(3),
            },
            InnerPlotCigar {
                cigar_type: Match,
                bases: None,
                length: Some(10),
            },
            InnerPlotCigar {
                cigar_type: Sub,
                bases: Some(vec!['C']),
                length: Some(1),
            },
            InnerPlotCigar {
                cigar_type: Sub,
                bases: Some(vec!['G']),
                length: Some(1),
            },
            InnerPlotCigar {
                cigar_type: Ins,
                bases: Some(vec!['G', 'G', 'T']),
                length: None,
            },
        ]);
        let expected_string = "50=|3d|10=|1C|1G|iGGT".to_string();
        assert_eq!(plot_cigar.to_string(), expected_string);
    }

    #[test]
    fn test_read_ordering() {
        let read1 = Read {
            name: "read1".to_string(),
            cigar: PlotCigar(vec![]),
            position: 20,
            flags: 0,
            mapq: 0,
            row: None,
            end_position: 120,
            mpos: 0
        };

        let read2 = Read {
            name: "read2".to_string(),
            cigar: PlotCigar(vec![]),
            position: 40,
            flags: 0,
            mapq: 0,
            row: None,
            end_position: 140,
            mpos: 0
        };

        let mut reads = vec![read1, read2];
        reads.order(100).unwrap();
        assert_ne!(reads.first().unwrap().row, reads.last().unwrap().row);
    }

    #[test]
    fn test_read_ordering_with_max_read_depth() {
        let read1 = Read {
            name: "read1".to_string(),
            cigar: PlotCigar(vec![]),
            position: 20,
            flags: 0,
            mapq: 0,
            row: None,
            end_position: 120,
            mpos: 0
        };

        let read2 = Read {
            name: "read2".to_string(),
            cigar: PlotCigar(vec![]),
            position: 40,
            flags: 0,
            mapq: 0,
            row: None,
            end_position: 140,
            mpos: 0
        };

        let read3 = Read {
            name: "read3".to_string(),
            cigar: PlotCigar(vec![]),
            position: 50,
            flags: 0,
            mapq: 0,
            row: None,
            end_position: 150,
            mpos: 0
        };

        let mut reads = vec![read1, read2, read3];
        reads.order(2).unwrap();
        assert_eq!(reads.len(), 2);
    }

    #[test]
    fn test_matching_bases() {
        let reference = vec!['A', 'A', 'G', 'C', 'T', 'A'];
        let read = vec!['A', 'A', 'G', 'C', 'C', 'A'];
        let inner_plot_cigars = match_bases(&read, &reference);
        let expected_inner_plot_cigars = vec![
            InnerPlotCigar {
                cigar_type: CigarType::Match,
                bases: None,
                length: Some(4),
            },
            InnerPlotCigar {
                cigar_type: CigarType::Sub,
                bases: Some(vec!['C']),
                length: Some(1),
            },
            InnerPlotCigar {
                cigar_type: CigarType::Match,
                bases: None,
                length: Some(1),
            },
        ];
        assert_eq!(inner_plot_cigars, expected_inner_plot_cigars)
    }

    #[test]
    fn test_plot_cigar_match() {
        let cigar_string = CigarStringView::new(CigarString::from(vec![Cigar::Match(10)]), 0);
        let reference = vec!['A', 'A', 'G', 'C', 'T', 'A', 'T', 'A', 'T', 'A'];
        let read = vec!['A', 'A', 'G', 'C', 'C', 'A', 'T', 'A', 'T', 'A'];
        let cigar = PlotCigar::from_cigar(cigar_string, read, reference).unwrap();
        let expected_cigar = PlotCigar(vec![
            InnerPlotCigar {
                cigar_type: CigarType::Match,
                bases: None,
                length: Some(4),
            },
            InnerPlotCigar {
                cigar_type: CigarType::Sub,
                bases: Some(vec!['C']),
                length: Some(1),
            },
            InnerPlotCigar {
                cigar_type: CigarType::Match,
                bases: None,
                length: Some(5),
            },
        ]);
        assert_eq!(cigar, expected_cigar);
    }

    #[test]
    fn test_plot_cigar_insertion() {
        let cigar_string = CigarStringView::new(
            CigarString::from(vec![Cigar::Match(2), Cigar::Ins(1), Cigar::Match(2)]),
            0,
        );
        let reference = vec!['A', 'A', 'G', 'C'];
        let read = vec!['A', 'A', 'A', 'G', 'C'];
        let cigar = PlotCigar::from_cigar(cigar_string, read, reference).unwrap();
        let expected_cigar = PlotCigar(vec![
            InnerPlotCigar {
                cigar_type: CigarType::Match,
                bases: None,
                length: Some(2),
            },
            InnerPlotCigar {
                cigar_type: CigarType::Ins,
                bases: Some(vec!['A']),
                length: Some(1),
            },
            InnerPlotCigar {
                cigar_type: CigarType::Match,
                bases: None,
                length: Some(2),
            },
        ]);
        assert_eq!(cigar, expected_cigar);
    }

    #[test]
    fn test_plot_cigar_deletion() {
        let cigar_string = CigarStringView::new(
            CigarString::from(vec![Cigar::Match(2), Cigar::Del(2), Cigar::Match(2)]),
            0,
        );
        let reference = vec!['A', 'A', 'A', 'A', 'G', 'C'];
        let read = vec!['A', 'A', 'G', 'C'];
        let cigar = PlotCigar::from_cigar(cigar_string, read, reference).unwrap();
        let expected_cigar = PlotCigar(vec![
            InnerPlotCigar {
                cigar_type: CigarType::Match,
                bases: None,
                length: Some(2),
            },
            InnerPlotCigar {
                cigar_type: CigarType::Del,
                bases: None,
                length: Some(2),
            },
            InnerPlotCigar {
                cigar_type: CigarType::Match,
                bases: None,
                length: Some(2),
            },
        ]);
        assert_eq!(cigar, expected_cigar);
    }

    #[test]
    fn test_fetch_reference() {
        let reference = read_fasta(
            "tests/reference.fa",
            &Region {
                target: "chr1".to_string(),
                start: 0,
                end: 20,
            },
        )
        .unwrap();
        let expected_reference = "TTGCCGGGGTGGGGAGAGAG".chars().collect_vec();
        assert_eq!(reference, expected_reference);
    }
}
