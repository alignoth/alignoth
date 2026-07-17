use anyhow::{anyhow, Context, Result};
use bio::io::fasta;
use itertools::Itertools;
use rust_htslib::bcf::{Format, Header, Read as BcfRead, Reader, Writer};
use rust_htslib::{bam, bcf};
use std::fs;
use std::path::{Path, PathBuf};

// Check if the cwd contains only one fasta (.fa, .fasta, .fasta.gz, .fa.gz) file and at least one bam (.bam, .bam.gz) file. Returns the fasta path and a vector of all bam paths.
pub(crate) fn get_ref_and_bam_from_cwd() -> Result<Option<(PathBuf, Vec<PathBuf>)>> {
    let mut fasta_path: Option<PathBuf> = None;
    let mut bam_paths: Vec<PathBuf> = Vec::new();
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
                bam_paths.push(path);
            }
        }
    }
    if let Some(fasta) = fasta_path {
        if !bam_paths.is_empty() {
            return Ok(Some((fasta, bam_paths)));
        }
    }
    Ok(None)
}

// Get length of fasta file and given target
pub(crate) fn get_fasta_length(fasta_path: &PathBuf, target: &str) -> Result<usize> {
    let mut fasta_reader = fasta::IndexedReader::from_file(fasta_path)?;
    let mut seq: Vec<u8> = Vec::new();
    fasta_reader.fetch_all(target)?;
    fasta_reader.read(&mut seq)?;
    Ok(seq.len())
}

// Get all contigs/chromosomes from fasta file
pub(crate) fn get_fasta_contigs(fasta_path: &PathBuf) -> Result<Vec<String>> {
    let fasta_reader = fasta::Reader::from_file(fasta_path)?;
    let mut contigs = Vec::new();
    for record in fasta_reader.records() {
        let record = record?;
        contigs.push(record.id().to_string());
    }
    Ok(contigs)
}

/// Returns `path` with `.extension` appended, e.g. `reads.bam` + `bai` -> `reads.bam.bai`.
fn appended_extension(path: &Path, extension: &str) -> PathBuf {
    let mut name = path.as_os_str().to_owned();
    name.push(".");
    name.push(extension);
    PathBuf::from(name)
}

/// Returns whether a coordinate index (`.bai` or `.csi`) exists next to the given BAM/CRAM file.
pub(crate) fn bam_index_present(path: &Path) -> bool {
    appended_extension(path, "bai").exists() || appended_extension(path, "csi").exists()
}

/// Builds a `.bai` index for the given (coordinate-sorted) BAM/CRAM file.
pub(crate) fn build_bam_index(path: &Path) -> Result<()> {
    bam::index::build(path, None, bam::index::Type::Bai, 1).with_context(|| {
        format!(
            "Failed to build index for {}. Is the file coordinate-sorted?",
            path.display()
        )
    })
}

/// Returns whether a `.fai` index exists next to the given FASTA file.
pub(crate) fn fasta_index_present(path: &Path) -> bool {
    appended_extension(path, "fai").exists()
}

/// Builds a `.fai` index for the given FASTA file.
pub(crate) fn build_fasta_index(path: &Path) -> Result<()> {
    rust_htslib::faidx::build(path)
        .map_err(|e| anyhow!("Failed to build index for {}: {e}", path.display()))
}

/// Returns whether a `.tbi` or `.csi` index exists next to the given VCF/BCF file.
pub(crate) fn vcf_index_present(path: &Path) -> bool {
    appended_extension(path, "tbi").exists() || appended_extension(path, "csi").exists()
}

/// Builds an index for the given VCF/BCF file, bgzipping a plain `.vcf` first if necessary.
/// Returns the path that should be used downstream (unchanged, or the newly written `.vcf.gz`).
pub(crate) fn build_vcf_index(path: &Path) -> Result<PathBuf> {
    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or_default();
    if name.ends_with(".bcf") {
        bcf::index::build(path, None, 1, bcf::index::Type::Csi(14))?;
        Ok(path.to_path_buf())
    } else if name.ends_with(".vcf.gz") {
        build_bgzf_index(path)?;
        Ok(path.to_path_buf())
    } else {
        let bgzipped = bgzip_vcf(path)?;
        build_bgzf_index(&bgzipped)?;
        Ok(bgzipped)
    }
}

/// Builds a tabix index for a bgzipped VCF, removing any pre-existing `.csi`/`.tbi` first.
fn build_bgzf_index(path: &Path) -> Result<()> {
    for extension in ["csi", "tbi"] {
        let index = appended_extension(path, extension);
        if index.exists() {
            std::fs::remove_file(&index)?;
        }
    }
    bcf::index::build(path, None, 1, bcf::index::Type::Tbx)?;
    Ok(())
}

/// Rewrites an uncompressed VCF as a bgzipped `.vcf.gz` next to it and returns the new path.
fn bgzip_vcf(path: &Path) -> Result<PathBuf> {
    let mut reader = Reader::from_path(path)?;
    let header = Header::from_template(reader.header());
    let output = appended_extension(path, "gz");
    let mut writer = Writer::from_path(&output, &header, false, Format::Vcf)?;
    for record in reader.records() {
        writer.write(&record?)?;
    }
    Ok(output)
}

/// Ensures a coordinate index exists for the given BAM/CRAM file, building one if it is missing.
pub(crate) fn ensure_bam_index(path: &Path) -> Result<()> {
    if !bam_index_present(path) {
        build_bam_index(path)?;
    }
    Ok(())
}

/// Ensures a `.fai` index exists for the given FASTA file, building one if it is missing.
pub(crate) fn ensure_fasta_index(path: &Path) -> Result<()> {
    if !fasta_index_present(path) {
        build_fasta_index(path)?;
    }
    Ok(())
}

/// Ensures an index exists for the given VCF/BCF file, building one (and bgzipping a plain `.vcf`
/// if necessary) when it is missing. Returns the path that should be used downstream.
pub(crate) fn ensure_vcf_index(path: &Path) -> Result<PathBuf> {
    if vcf_index_present(path) {
        Ok(path.to_path_buf())
    } else {
        build_vcf_index(path)
    }
}

/// Takes any given aux and returns a string representation of it.
pub(crate) fn aux_to_string(aux: rust_htslib::bam::record::Aux) -> String {
    match aux {
        rust_htslib::bam::record::Aux::Char(c) => String::from_utf8(vec![c]).unwrap(),
        rust_htslib::bam::record::Aux::I8(i) => i.to_string(),
        rust_htslib::bam::record::Aux::U8(i) => i.to_string(),
        rust_htslib::bam::record::Aux::I16(i) => i.to_string(),
        rust_htslib::bam::record::Aux::U16(i) => i.to_string(),
        rust_htslib::bam::record::Aux::I32(i) => i.to_string(),
        rust_htslib::bam::record::Aux::U32(i) => i.to_string(),
        rust_htslib::bam::record::Aux::Float(i) => i.to_string(),
        rust_htslib::bam::record::Aux::Double(i) => i.to_string(),
        rust_htslib::bam::record::Aux::String(s) => s.to_owned(),
        rust_htslib::bam::record::Aux::HexByteArray(i) => i.to_string(),
        rust_htslib::bam::record::Aux::ArrayI8(a) => a.iter().join(","),
        rust_htslib::bam::record::Aux::ArrayU8(a) => a.iter().join(","),
        rust_htslib::bam::record::Aux::ArrayU16(a) => a.iter().join(","),
        rust_htslib::bam::record::Aux::ArrayI16(a) => a.iter().join(","),
        rust_htslib::bam::record::Aux::ArrayU32(a) => a.iter().join(","),
        rust_htslib::bam::record::Aux::ArrayI32(a) => a.iter().join(","),
        rust_htslib::bam::record::Aux::ArrayFloat(a) => a.iter().join(","),
    }
}

#[allow(dead_code)]
pub(crate) fn ellipsis(s: &str, max_len: usize) -> String {
    if s.len() > max_len {
        format!("{}…", &s[..max_len])
    } else {
        s.to_string()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::{
        bam_index_present, build_bam_index, build_fasta_index, build_vcf_index, ellipsis,
        fasta_index_present, get_fasta_contigs, get_fasta_length, get_ref_and_bam_from_cwd,
        vcf_index_present,
    };
    use std::path::{Path, PathBuf};
    use std::str::FromStr;
    use tempfile::TempDir;

    fn copy_to_temp(fixture: &str) -> (TempDir, PathBuf) {
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join(Path::new(fixture).file_name().unwrap());
        std::fs::copy(fixture, &path).unwrap();
        (dir, path)
    }

    #[test]
    fn test_build_bam_index() {
        let (_dir, bam) = copy_to_temp("tests/sample_1/reads.bam");
        assert!(!bam_index_present(&bam));
        build_bam_index(&bam).unwrap();
        assert!(bam_index_present(&bam));
        assert!(rust_htslib::bam::IndexedReader::from_path(&bam).is_ok());
    }

    #[test]
    fn test_build_fasta_index() {
        let (_dir, fasta) = copy_to_temp("tests/sample_1/reference.fa");
        assert!(!fasta_index_present(&fasta));
        build_fasta_index(&fasta).unwrap();
        assert!(fasta_index_present(&fasta));
    }

    #[test]
    fn test_build_vcf_index_for_bgzipped_vcf() {
        let (_dir, vcf) = copy_to_temp("tests/sample_3/1257A.vcf.gz");
        assert!(!vcf_index_present(&vcf));
        let indexed = build_vcf_index(&vcf).unwrap();
        assert_eq!(indexed, vcf);
        assert!(vcf_index_present(&vcf));
    }

    #[test]
    fn test_build_vcf_index_bgzips_plain_vcf() {
        let (_dir, vcf) = copy_to_temp("tests/sample_3/1257A.vcf");
        let indexed = build_vcf_index(&vcf).unwrap();
        assert_eq!(indexed, vcf.with_extension("vcf.gz"));
        assert!(vcf_index_present(&indexed));
        assert!(rust_htslib::bcf::IndexedReader::from_path(&indexed).is_ok());
    }

    #[test]
    fn test_build_vcf_index_removes_stale_index() {
        use rust_htslib::bcf::{IndexedReader, Read};
        let dir = tempfile::tempdir().unwrap();
        let vcf = dir.path().join("1257A.vcf");
        std::fs::copy("tests/sample_3/1257A.vcf", &vcf).unwrap();
        let stale_csi = dir.path().join("1257A.vcf.gz.csi");
        std::fs::copy("tests/sample_3/1257A.vcf.gz.csi", &stale_csi).unwrap();

        let indexed = build_vcf_index(&vcf).unwrap();
        assert_eq!(indexed, vcf.with_extension("vcf.gz"));
        assert!(
            !stale_csi.exists(),
            "stale .csi index should have been removed"
        );

        let mut reader = IndexedReader::from_path(&indexed).unwrap();
        let rid = reader.header().name2rid(b"1").unwrap();
        reader.fetch(rid, 0, None).unwrap();
        let records: Vec<_> = reader.records().collect();
        assert!(
            records.iter().all(|record| record.is_ok()),
            "reading a variant record failed"
        );
        assert_eq!(records.len(), 1);
    }

    #[test]
    fn test_get_fasta_length() {
        let length = get_fasta_length(
            &PathBuf::from_str("tests/sample_1/reference.fa").unwrap(),
            "chr1",
        )
        .unwrap();
        assert_eq!(length, 123)
    }

    #[test]
    fn test_get_ref_and_bam_from_empty_cwd() {
        let result = get_ref_and_bam_from_cwd().unwrap();
        assert_eq!(result, None)
    }

    #[test]
    fn test_get_fasta_contigs() {
        let contigs =
            get_fasta_contigs(&PathBuf::from_str("tests/sample_1/reference.fa").unwrap()).unwrap();
        assert_eq!(contigs, vec!["chr1".to_string()])
    }

    #[test]
    fn test_ellipsis() {
        assert_eq!(ellipsis("ABCDE", 5), "ABCDE");
        assert_eq!(ellipsis("ABCDEFG", 5), "ABCDE…");
        assert_eq!(ellipsis("", 5), "");
    }
}
