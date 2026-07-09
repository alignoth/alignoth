use crate::cli::{Alignoth, Around, Clamp, FromAround, Interval, Region};
use crate::utils::{
    bam_index_present, build_bam_index, build_fasta_index, build_vcf_index, fasta_index_present,
    get_fasta_contigs, get_fasta_length, vcf_index_present,
};
use anyhow::{bail, Result};
use inquire::{Confirm, Select, Text};
use std::fmt::Display;
use std::fs;
use std::path::{Path, PathBuf};
use std::str::FromStr;

pub(crate) async fn wizard_mode() -> Result<Alignoth> {
    println!("Welcome to Alignoth wizard mode 🪄 Let's build your plot interactively.\n");

    let current_dir = std::env::current_dir()?;
    let bam_files: Vec<_> = fs::read_dir(&current_dir)?
        .filter_map(|entry| entry.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .is_some_and(|ext| ext == "bam" || ext == "sam" || ext == "cram")
        })
        .collect();
    let fasta_files: Vec<_> = fs::read_dir(&current_dir)?
        .filter_map(|entry| entry.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .is_some_and(|ext| ext == "fa" || ext == "fasta")
        })
        .collect();
    let vcf_files: Vec<_> = fs::read_dir(&current_dir)?
        .filter_map(|entry| entry.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.file_name().and_then(|n| n.to_str()).is_some_and(|name| {
                name.ends_with(".vcf.gz") || name.ends_with(".bcf") || name.ends_with(".vcf")
            })
        })
        .collect();
    let bed_files: Vec<_> = fs::read_dir(&current_dir)?
        .filter_map(|entry| entry.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|ext| ext == "bed"))
        .collect();

    let bam_path = if bam_files.is_empty() {
        PathBuf::from(Text::new("Path to BAM file:").prompt()?)
    } else {
        let choices: Vec<_> = bam_files.iter().map(|p| p.display().to_string()).collect();
        PathBuf::from(Select::new("Select BAM file:", choices).prompt()?)
    };
    ensure_required_index(&bam_path, bam_index_present(&bam_path), || {
        build_bam_index(&bam_path)
    })?;

    let reference_path = if fasta_files.is_empty() {
        PathBuf::from(Text::new("Path to reference FASTA file:").prompt()?)
    } else {
        let choices: Vec<_> = fasta_files
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        PathBuf::from(Select::new("Select reference FASTA file:", choices).prompt()?)
    };
    ensure_required_index(
        &reference_path,
        fasta_index_present(&reference_path),
        || build_fasta_index(&reference_path),
    )?;

    let contigs = get_fasta_contigs(&reference_path)?;
    let target = Select::new("Select target contig/chromosome:", contigs).prompt()?;

    let mut region = match Select::new(
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

    let target_length = get_fasta_length(&reference_path, &target)? as i64;
    region = region.clamp(0, target_length - 1);
    let vcf_input = match select_optional_file(
        "Do you want to provide a VCF file to highlight variant positions?",
        "path/to/file.vcf",
        &vcf_files,
    )? {
        Some(vcf) => ensure_optional_vcf_index(vcf)?,
        None => None,
    };
    let bed_input = select_optional_file(
        "Do you want to provide a BED file to highlight certain regions?",
        "path/to/file.bed",
        &bed_files,
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

/// Asks the user for an optional file, letting them pick from `candidates` found in the current
/// directory or enter a path manually. Returns `None` if the user chooses to skip.
fn select_optional_file(
    question: &str,
    example: &str,
    candidates: &[PathBuf],
) -> Result<Option<PathBuf>> {
    if candidates.is_empty() {
        let input = Text::new(&format!(
            "{question} (Example: {example}, press Enter to skip)"
        ))
        .prompt()?;
        Ok((!input.trim().is_empty()).then(|| PathBuf::from(input.trim())))
    } else {
        let mut choices: Vec<_> = candidates.iter().map(|p| p.display().to_string()).collect();
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
