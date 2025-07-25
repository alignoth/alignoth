use crate::cli::{Alignoth, Around, FromAround, Interval, Region};
use crate::utils::get_fasta_contigs;
use anyhow::Result;
use inquire::{Select, Text};
use std::fs;
use std::path::PathBuf;
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

    let bam_path = if bam_files.is_empty() {
        Text::new("Path to BAM file:").prompt()?
    } else {
        let choices: Vec<_> = bam_files.iter().map(|p| p.display().to_string()).collect();
        Select::new("Select BAM file:", choices).prompt()?
    };

    let reference_path = if fasta_files.is_empty() {
        Text::new("Path to reference FASTA file:").prompt()?
    } else {
        let choices: Vec<_> = fasta_files
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        Select::new("Select reference FASTA file:", choices).prompt()?
    };

    let contigs = get_fasta_contigs(&PathBuf::from(reference_path.clone()))?;
    let target = Select::new("Select target contig/chromosome:", contigs).prompt()?;

    let region = match Select::new(
        "Do you want to visualize around a certain position or a specific region?",
        vec!["Around a position", "Region"],
    )
    .prompt()?
    {
        "Around a position" => {
            let around = Around {
                position: Text::new("Position:").prompt()?.parse()?,
                target: target.clone(),
            };
            Region::from_around(&around)
        }
        "Region" => {
            let start = Text::new("Start coordinate:").prompt()?.parse()?;
            let end = Text::new("End coordinate:").prompt()?.parse()?;
            Region { target, start, end }
        }
        _ => unreachable!(),
    };

    let highlight_input = Text::new("Do you want to highlight a specific region or position? (Example: 1000-2000 or 1200, press Enter to skip)").prompt()?;
    let highlight = if highlight_input.is_empty() {
        None
    } else {
        Some(Interval::from_str(&highlight_input)?)
    };
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

    let max_depth = Text::new("Max read depth (default 500):")
        .with_default("500")
        .prompt()?;

    let html_output = Select::new(
        "Choose output type:",
        vec!["Interactive HTML", "Vega-Lite Specs"],
    )
    .prompt()?
        == "Interactive HTML";

    Ok(Alignoth {
        bam_path: Some(bam_path.into()),
        reference: Some(reference_path.into()),
        region: Some(region),
        aux_tag: aux_tags,
        max_read_depth: max_depth.parse()?,
        max_width: 1024,
        output: None,
        data_format: Default::default(),
        html: html_output,
        around: None,
        plot_all: false,
        highlight,
        highlight_data_output: None,
        spec_output: None,
        ref_data_output: None,
        read_data_output: None,
        coverage_output: None,
    })
}
