use anyhow::Result;
use std::path::Path;
use std::fs;

use crate::constants::Text;
use crate::detector::{detect_project_tech, get_exclude_patterns, get_extensions_by_tech};

pub fn aggregate_repo(repo_path: &Path, text: &Text) -> Result<bool> {
    // Check if the repository exists
    if !repo_path.exists() {
        println!(
            "{}",
            text.directory_not_found
                .replace("{path}", repo_path.to_string_lossy().as_ref())
        );
        return Ok(false);
    }

    println!("{}{}", text.analyzing, repo_path.to_string_lossy());
    println!("{}", text.scanning);

    // Detect the project technology
    let detected_techs = detect_project_tech(repo_path);
    if !detected_techs.is_empty() {
        println!("{}{}", text.tech_detected, detected_techs.join(", "));
    } else {
        println!("{}", text.no_tech);
    }

    // Get the target extensions
    let target_extensions = get_extensions_by_tech(&detected_techs);
    let (exclude_dirs, exclude_files) = get_exclude_patterns();

    let extensions_str = target_extensions
        .iter()
        .map(|s| s.as_str())
        .collect::<Vec<_>>()
        .join(", ");
    println!("{}{}", text.included_ext, extensions_str);
    println!(
        "{}{}",
        text.excluded_dirs,
        exclude_dirs
            .iter()
            .map(|s| s.as_str())
            .collect::<Vec<_>>()
            .join(", ")
    );

    let mut content = Vec::new();
    content.push(format!("# {}", "=".repeat(50)));
    content.push(format!("# {}", "Source Code Aggregation"));
    content.push(format!("# Project path: {}", repo_path.to_string_lossy()));
    content.push(format!(
        "# Detected technologies: {}",
        if detected_techs.is_empty() {
            "Unknown".to_string()
        } else {
            detected_techs
                .iter()
                .map(|s| s.split_whitespace().last().unwrap_or(s))
                .collect::<Vec<_>>()
                .join(", ")
        }
    ));
    content.push(format!("# {}", "=".repeat(50)));

    // Write the content to the output file
    let output_path = repo_path.join("source_dump.txt");
    let final_content = content.join("\n");
    match fs::write(&output_path, &final_content) {
        Ok(_) => {
            Ok(true)
        }
        Err(e) => {
            Ok(false)
        }
    }
}
