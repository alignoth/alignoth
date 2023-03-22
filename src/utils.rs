use anyhow::Result;
use bio::io::fasta;
use std::fs;
use std::path::PathBuf;

// Check if the cwd contains only one fasta (.fa, .fasta, .fasta.gz, .fa.gz) and only one single bam (.bam, .bam.gz) file and if it does returns both paths.
pub(crate) fn get_ref_and_bam_from_cwd() -> Result<Option<(PathBuf, PathBuf)>> {
    let mut fasta_path: Option<PathBuf> = None;
    let mut bam_path: Option<PathBuf> = None;
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() && path.extension().is_some() {
            let ext = path.extension().unwrap().to_str().unwrap();
            if ext == "fa" || ext == "fasta" || ext == "fa.gz" || ext == "fasta.gz" {
                if fasta_path.is_some() {
                    // There is already a fasta file in the cwd
                    return Ok(None);
                }
                fasta_path = Some(path);
            } else if ext == "bam" || ext == "bam.gz" {
                if bam_path.is_some() {
                    // There is already a bam file in the cwd
                    return Ok(None);
                }
                bam_path = Some(path);
            }
        }
    }
    if let (Some(fasta), Some(bam)) = (fasta_path, bam_path) {
        Ok(Some((fasta, bam)))
    } else {
        Ok(None)
    }
}

// Get length of fasta file and given target
pub(crate) fn get_fasta_length(fasta_path: &PathBuf, target: &str) -> Result<usize> {
    let mut fasta_reader = fasta::IndexedReader::from_file(fasta_path)?;
    let mut seq: Vec<u8> = Vec::new();
    fasta_reader.fetch_all(target)?;
    fasta_reader.read(&mut seq)?;
    Ok(seq.len())
}
