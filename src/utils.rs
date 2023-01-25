use anyhow::Result;
use std::fs;
use std::path::PathBuf;

// Check if the cwd contains only one fasta (.fa, .fasta, .fasta.gz, .fa.gz) and only one single bam (.bam, .bam.gz) file and if it does returns both paths.
pub(crate) fn get_ref_and_bam_from_cwd() -> Result<Option<(PathBuf, PathBuf)>> {
    let mut fasta_path: Option<PathBuf> = None;
    let mut bam_path: Option<PathBuf> = None;
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();
        if path.is_file() {
            if path.extension().is_some() {
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
    }
    if fasta_path.is_some() && bam_path.is_some() {
        Ok(Some((fasta_path.unwrap(), bam_path.unwrap())))
    } else {
        Ok(None)
    }
}
