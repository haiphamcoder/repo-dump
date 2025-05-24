use anyhow::Result;
use std::path::Path;
use std::fs;

use crate::constants::{MAX_FILE_SIZE, Text};
use crate::detector::{detect_project_tech, get_exclude_patterns, get_extensions_by_tech, should_exclude_path, should_exclude_file};
use crate::tree_generator::generate_directory_tree;

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
    content.push("".to_string());

    // Generate the directory tree
    println!("{}", text.generating_tree);
    content.push("## DIRECTORY STRUCTURE".to_string());
    content.push("```".to_string());
    content.push(generate_directory_tree(repo_path, &exclude_dirs));
    content.push("```".to_string());
    content.push("".to_string());

    // Process the files
    println!("{}", text.processing_files);
    content.push("## FILE CONTENTS".to_string());
    content.push("".to_string());

    let mut file_count = 0;
    let mut total_size = 0;

    for entry in walkdir::WalkDir::new(repo_path) {
        let entry = match entry {
            Ok(e) => e,
            Err(_) => continue,
        };

        let path = entry.path();
        let rel_path = path.strip_prefix(repo_path).unwrap_or(path);

        if should_exclude_path(rel_path, &exclude_dirs) || 
           (path.is_file() && should_exclude_file(rel_path.file_name().unwrap().to_string_lossy().as_ref(), &exclude_files)) {
            continue;
        }

        if path.is_file() {
            let file_ext = path.extension()
                .and_then(|e| e.to_str())
                .map(|s| format!(".{}", s))
                .unwrap_or_default();

            if !target_extensions.contains(&file_ext) {
                continue;
            }

            match fs::metadata(path) {
                Ok(metadata) => {
                    if metadata.len() > MAX_FILE_SIZE {
                        println!("{}", text.skip_large
                            .replace("{file}", &rel_path.to_string_lossy())
                            .replace("{size}", &metadata.len().to_string())
                            .replace("{limit}", &MAX_FILE_SIZE.to_string()));
                        continue;
                    }
                }
                Err(_) => continue,
            }

            println!("{}", text.processing.replace("{file}", &rel_path.to_string_lossy()));

            match fs::read_to_string(path) {
                Ok(file_content) => {
                    let content_clone = file_content.clone();
                    content.push(format!("### {}", rel_path.to_string_lossy()));
                    content.push(format!("```{}", file_ext.trim_start_matches('.')));
                    content.push(file_content);
                    content.push("```".to_string());
                    content.push("".to_string());

                    file_count += 1;
                    total_size += content_clone.len();
                }
                Err(e) => {
                    content.push(format!("### {}", rel_path.to_string_lossy()));
                    content.push(format!("```\n# Error reading file: {}\n```", e));
                    content.push("".to_string());
                }
            }
        }
    }

    // Write the content to the output file
    let output_path = repo_path.join("source_dump.txt");
    let final_content = content.join("\n");
    match fs::write(&output_path, &final_content) {
        Ok(_) => {
            let line_count = final_content.lines().count();
            println!("");
            println!("{}", text.summary);
            println!("{}", text.file_count.replace("{count}", &file_count.to_string()));
            println!("{}", text.size
                .replace("{size}", &final_content.len().to_string())
                .replace("{kb}", &(total_size / 1024).to_string()));
            println!("{}", text.line_count.replace("{lines}", &line_count.to_string()));
            println!("");
            println!("{}{}", text.success, output_path.to_string_lossy());
            Ok(true)
        }
        Err(e) => {
            println!("{}", text.write_error.replace("{error}", &e.to_string()));
            Ok(false)
        }
    }
}
