use std::collections::HashSet;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn generate_directory_tree(project_path: &Path, exclude_dirs: &HashSet<String>) -> String {
    let mut tree_lines = Vec::new();
    let project_name = project_path.file_name()
        .unwrap_or_else(|| project_path.as_os_str())
        .to_string_lossy();
    tree_lines.push(format!("{}/", project_name));

    fn add_directory_content(
        current_path: &Path,
        prefix: &str,
        tree_lines: &mut Vec<String>,
        exclude_dirs: &HashSet<String>,
    ) {
        let mut entries: Vec<PathBuf> = WalkDir::new(current_path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .map(|e| e.path().to_path_buf())
            .collect();
        entries.sort_by(|a, b| a.file_name().cmp(&b.file_name()));

        let mut dirs = Vec::new();
        let mut files = Vec::new();

        for path in entries {
            if path.is_dir() {
                dirs.push(path);
            } else {
                files.push(path);
            }
        }

        // Display directories
        for (i, dir) in dirs.iter().enumerate() {
            let is_last_dir = (i == dirs.len() - 1) && files.is_empty();
            let dirname = dir.file_name().unwrap().to_string_lossy();

            if exclude_dirs.contains(&dirname.to_lowercase()) {
                tree_lines.push(format!("{}{}{}/", 
                    prefix,
                    if is_last_dir { "└── " } else { "├── " },
                    dirname
                ));
                continue;
            }

            tree_lines.push(format!("{}{}{}/", 
                prefix,
                if is_last_dir { "└── " } else { "├── " },
                dirname
            ));

            let next_prefix = format!("{}{}", 
                prefix,
                if is_last_dir { "    " } else { "│   " }
            );
            add_directory_content(dir, &next_prefix, tree_lines, exclude_dirs);
        }

        // Display files
        for (i, file) in files.iter().enumerate() {
            let is_last = i == files.len() - 1;
            let filename = file.file_name().unwrap().to_string_lossy();
            tree_lines.push(format!("{}{}{}", 
                prefix,
                if is_last { "└── " } else { "├── " },
                filename
            ));
        }
    }

    add_directory_content(project_path, "", &mut tree_lines, exclude_dirs);
    tree_lines.join("\n")
} 