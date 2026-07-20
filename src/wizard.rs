use crate::cli::{Alignoth, Around, FromAround, Interval, Region};
use crate::utils::{
    bam_index_present, build_bam_index, build_fasta_index, build_vcf_index, fasta_index_present,
    get_fasta_contigs, vcf_index_present, FileKind,
};
use anyhow::{bail, Result};
use inquire::autocompletion::Replacement;
use inquire::{Autocomplete, Confirm, CustomUserError, Select, Text};
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub(crate) async fn wizard_mode() -> Result<Alignoth> {
    println!("Welcome to Alignoth wizard mode 🪄 Let's build your plot interactively.\n");

    let current_dir = std::env::current_dir()?;
    let bam_files = files_in(&current_dir, FileKind::Alignment)?;
    let fasta_files = files_in(&current_dir, FileKind::Fasta)?;
    let vcf_files = files_in(&current_dir, FileKind::Vcf)?;
    let bed_files = files_in(&current_dir, FileKind::Bed)?;

    let bam_path = select_required_file(
        "Path to BAM file:",
        "Select BAM file:",
        &bam_files,
        FileKind::Alignment,
    )?;
    ensure_required_index(&bam_path, bam_index_present(&bam_path), || {
        build_bam_index(&bam_path)
    })?;

    let reference_path = select_required_file(
        "Path to reference FASTA file:",
        "Select reference FASTA file:",
        &fasta_files,
        FileKind::Fasta,
    )?;
    ensure_required_index(
        &reference_path,
        fasta_index_present(&reference_path),
        || build_fasta_index(&reference_path),
    )?;

    let contigs = get_fasta_contigs(&reference_path)?;
    let target = Select::new("Select target contig/chromosome:", contigs).prompt()?;

    let region = match Select::new(
        "Do you want to visualize around a certain position or a specific region?",
        vec!["Around a position", "Region"],
    )
    .prompt()?
    {
        "Around a position" => {
            let around = Around {
                position: prompt_parse("Position:", None)?,
                target: target.clone(),
            };
            Region::from_around(&around)
        }
        "Region" => {
            let start: i64 = prompt_parse("Start coordinate:", None)?;
            let end: i64 = prompt_parse("End coordinate:", None)?;
            Region {
                target: target.clone(),
                start: start - 1, // Adjust for 1-based
                end,
            }
        }
        _ => unreachable!(),
    };

    let clamp_reads: bool = Text::new("Clamp reads to the specified region?")
        .with_default("false")
        .prompt()?
        .parse()?;

    let vcf_input = match select_optional_file(
        "Do you want to provide a VCF file to highlight variant positions?",
        "path/to/file.vcf",
        &vcf_files,
        FileKind::Vcf,
    )? {
        Some(vcf) => ensure_optional_vcf_index(vcf)?,
        None => None,
    };
    let bed_input = select_optional_file(
        "Do you want to provide a BED file to highlight certain regions?",
        "path/to/file.bed",
        &bed_files,
        FileKind::Bed,
    )?;
    let highlight = prompt_parse_optional::<Interval>("Do you want to highlight a specific region or position? (Example: some_interval:1000-2000 or some_position:1200, press Enter to skip)")?
        .map(|interval| vec![interval]);
    let aux_tag_input =
        Text::new("Optional auxiliary tags (whitespace-separated, press Enter to skip):")
            .prompt_skippable()?;
    let aux_tags = aux_tag_input.and_then(|s| {
        let trimmed = s.trim();
        if trimmed.is_empty() {
            None
        } else {
            Some(trimmed.split_whitespace().map(String::from).collect())
        }
    });

    let max_read_depth: usize = prompt_parse("Max read depth (default 500):", Some("500"))?;

    let html_output = Select::new(
        "Choose output type:",
        vec!["Interactive HTML", "Vega-Lite Specs"],
    )
    .prompt()?
        == "Interactive HTML";

    Ok(Alignoth {
        bam_path: vec![bam_path],
        reference: Some(reference_path),
        region: Some(region),
        aux_tag: aux_tags,
        max_read_depth,
        max_width: 1024,
        output: None,
        data_format: Default::default(),
        html: html_output,
        around: None,
        around_vcf_record: None,
        plot_all: false,
        highlight,
        vcf: vcf_input,
        bed: bed_input,
        highlight_data_output: None,
        spec_output: None,
        ref_data_output: None,
        read_data_output: None,
        coverage_output: None,
        no_embed_js: false,
        mismatch_display_min_percent: 1.0,
        clamp_reads,
    })
}

/// Asks whether to create a missing index for `path`, defaulting to yes.
fn confirm_create_index(path: &Path) -> Result<bool> {
    Ok(Confirm::new(&format!(
        "No index found for {}. Create it now?",
        path.display()
    ))
    .with_default(true)
    .prompt()?)
}

/// Ensures a required index exists, offering to build it. Aborts the wizard if the user declines.
fn ensure_required_index(
    path: &Path,
    present: bool,
    build: impl FnOnce() -> Result<()>,
) -> Result<()> {
    if present {
        Ok(())
    } else if confirm_create_index(path)? {
        build()
    } else {
        bail!("An index is required for {}.", path.display());
    }
}

/// Ensures an index for an optional VCF, offering to build it (bgzipping a plain `.vcf` if needed).
/// Returns the effective path, or `None` if the user declines and the VCF is skipped.
fn ensure_optional_vcf_index(path: PathBuf) -> Result<Option<PathBuf>> {
    if vcf_index_present(&path) {
        Ok(Some(path))
    } else if confirm_create_index(&path)? {
        Ok(Some(build_vcf_index(&path)?))
    } else {
        println!("Skipping VCF highlighting for {}.", path.display());
        Ok(None)
    }
}

fn files_in(dir: &Path, kind: FileKind) -> Result<Vec<PathBuf>> {
    Ok(fs::read_dir(dir)?
        .filter_map(|entry| entry.ok())
        .map(|entry| entry.path())
        .filter(|path| kind.matches(path))
        .collect())
}

fn choices_from(candidates: &[PathBuf]) -> Vec<String> {
    candidates.iter().map(|p| p.display().to_string()).collect()
}

fn select_required_file(
    manual_question: &str,
    select_question: &str,
    candidates: &[PathBuf],
    kind: FileKind,
) -> Result<PathBuf> {
    if candidates.is_empty() {
        Ok(PathBuf::from(
            Text::new(manual_question)
                .with_autocomplete(FilePathCompleter::new(kind))
                .prompt()?,
        ))
    } else {
        Ok(PathBuf::from(
            Select::new(select_question, choices_from(candidates)).prompt()?,
        ))
    }
}

#[derive(Clone, Copy)]
struct FilePathCompleter {
    kind: FileKind,
}

impl FilePathCompleter {
    fn new(kind: FileKind) -> Self {
        Self { kind }
    }
}

impl Autocomplete for FilePathCompleter {
    fn get_suggestions(&mut self, input: &str) -> Result<Vec<String>, CustomUserError> {
        let (dir, prefix) = input.split_at(input.rfind('/').map_or(0, |i| i + 1));
        Ok(fs::read_dir(if dir.is_empty() { "." } else { dir })?
            .flatten()
            .filter_map(|entry| {
                let name = entry.file_name().into_string().ok()?;
                if !name.starts_with(prefix) {
                    return None;
                }
                let path = entry.path();
                if path.is_dir() {
                    return Some(format!("{dir}{name}/"));
                }
                self.kind.matches(&path).then(|| format!("{dir}{name}"))
            })
            .collect())
    }

    fn get_completion(
        &mut self,
        _: &str,
        highlighted: Option<String>,
    ) -> Result<Replacement, CustomUserError> {
        Ok(highlighted)
    }
}

/// Asks the user for an optional file, letting them pick from `candidates` found in the current
/// directory or enter a path manually. Returns `None` if the user chooses to skip.
fn select_optional_file(
    question: &str,
    example: &str,
    candidates: &[PathBuf],
    kind: FileKind,
) -> Result<Option<PathBuf>> {
    if candidates.is_empty() {
        let input = Text::new(&format!(
            "{question} (Example: {example}, press Enter to skip)"
        ))
        .with_autocomplete(FilePathCompleter::new(kind))
        .prompt()?;
        Ok((!input.trim().is_empty()).then(|| PathBuf::from(input.trim())))
    } else {
        let mut choices = choices_from(candidates);
        choices.push("Skip".to_string());
        let selection = Select::new(question, choices).prompt()?;
        Ok((selection != "Skip").then(|| PathBuf::from(&selection)))
    }
}

/// Prompts with `message`, re-asking until the input parses into `T` instead of aborting the
/// wizard on a parse error. An optional `default` pre-fills the prompt.
fn prompt_parse<T>(message: &str, default: Option<&str>) -> Result<T>
where
    T: FromStr,
    T::Err: Display,
{
    loop {
        let mut prompt = Text::new(message);
        if let Some(default) = default {
            prompt = prompt.with_default(default);
        }
        match prompt.prompt()?.parse() {
            Ok(value) => return Ok(value),
            Err(error) => eprintln!("Invalid input: {error}. Please try again."),
        }
    }
}

/// Like [`prompt_parse`], but treats empty input as `None` so the user can skip the prompt.
fn prompt_parse_optional<T>(message: &str) -> Result<Option<T>>
where
    T: FromStr,
    T::Err: Display,
{
    loop {
        let input = Text::new(message).prompt()?;
        if input.trim().is_empty() {
            return Ok(None);
        }
        match input.parse() {
            Ok(value) => return Ok(Some(value)),
            Err(error) => eprintln!("Invalid input: {error}. Please try again."),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    fn suggest_in_sample_dir(dir: &Path, kind: FileKind) -> Vec<String> {
        let mut completer = FilePathCompleter::new(kind);
        completer
            .get_suggestions(&format!("{}/", dir.display()))
            .unwrap()
            .iter()
            .map(|s| {
                let base = s.trim_end_matches('/').rsplit('/').next().unwrap();
                if s.ends_with('/') {
                    format!("{base}/")
                } else {
                    base.to_string()
                }
            })
            .collect()
    }

    fn sample_dir() -> tempfile::TempDir {
        let dir = tempfile::tempdir().unwrap();
        for name in [
            "sample.bam",
            "sample.cram",
            "ref.fa",
            "ref.fasta",
            "variants.vcf.gz",
            "regions.bed",
            "notes.txt",
        ] {
            File::create(dir.path().join(name)).unwrap();
        }
        fs::create_dir(dir.path().join("subdir")).unwrap();
        dir
    }

    #[test]
    fn fasta_prompt_does_not_suggest_bam_files() {
        let dir = sample_dir();
        let names = suggest_in_sample_dir(dir.path(), FileKind::Fasta);
        assert!(names.iter().any(|n| n == "ref.fa"));
        assert!(names.iter().any(|n| n == "ref.fasta"));
        assert!(
            !names
                .iter()
                .any(|n| n == "sample.bam" || n == "sample.cram"),
            "Can't suggest alignment files when asking for a FASTA file, got {names:?}"
        );
    }

    #[test]
    fn bam_prompt_only_suggests_alignment_files() {
        let dir = sample_dir();
        let names = suggest_in_sample_dir(dir.path(), FileKind::Alignment);
        assert!(names.iter().any(|n| n == "sample.bam"));
        assert!(names.iter().any(|n| n == "sample.cram"));
        assert!(
            !names.iter().any(|n| n == "ref.fa" || n == "regions.bed"),
            "Can't suggest non-alignment files, got {names:?}"
        );
    }

    #[test]
    fn vcf_and_bed_prompts_suggest_their_own_types() {
        let dir = sample_dir();
        let vcf = suggest_in_sample_dir(dir.path(), FileKind::Vcf);
        assert!(vcf.iter().any(|n| n == "variants.vcf.gz"));
        assert!(!vcf.iter().any(|n| n == "sample.bam"), "got {vcf:?}");

        let bed = suggest_in_sample_dir(dir.path(), FileKind::Bed);
        assert!(bed.iter().any(|n| n == "regions.bed"));
        assert!(!bed.iter().any(|n| n == "sample.bam"), "got {bed:?}");
    }

    #[test]
    fn directories_are_always_suggested_so_navigation_still_works() {
        let dir = sample_dir();
        for kind in [
            FileKind::Alignment,
            FileKind::Fasta,
            FileKind::Vcf,
            FileKind::Bed,
        ] {
            let names = suggest_in_sample_dir(dir.path(), kind);
            assert!(
                names.iter().any(|n| n == "subdir/"),
                "Directories must stay navigable, got {names:?}"
            );
            assert!(!names.iter().any(|n| n == "notes.txt"), "Got {names:?}");
        }
    }
}
