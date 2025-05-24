use std::collections::{HashMap, HashSet};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Clone)]
struct TechInfo {
    patterns: Vec<&'static str>,
    icon: &'static str,
}

pub fn detect_project_tech(project_path: &Path) -> Vec<String> {
    let tech_indicators: HashMap<&str, TechInfo> = [
        (
            "python",
            TechInfo {
                patterns: vec![
                    "requirements.txt",
                    "setup.py",
                    "pyproject.toml",
                    "Pipfile",
                    "*.py",
                    "*.ipynb",
                ],
                icon: "🐍",
            },
        ),
        (
            "javascript",
            TechInfo {
                patterns: vec!["package.json", "*.js"],
                icon: "📜",
            },
        ),
        (
            "typescript",
            TechInfo {
                patterns: vec!["tsconfig.json", "*.ts"],
                icon: "📘",
            },
        ),
        (
            "react",
            TechInfo {
                patterns: vec!["*.jsx", "*.tsx", "react.config.js"],
                icon: "⚛️",
            },
        ),
        (
            "vue",
            TechInfo {
                patterns: vec!["vue.config.js", "*.vue"],
                icon: "🟢",
            },
        ),
        (
            "svelte",
            TechInfo {
                patterns: vec!["svelte.config.js", "*.svelte"],
                icon: "✨",
            },
        ),
        (
            "nextjs",
            TechInfo {
                patterns: vec![
                    "next.config.js",
                    "pages/**/*.js",
                    "pages/**/*.jsx",
                    "pages/**/*.ts",
                    "pages/**/*.tsx",
                ],
                icon: "▲",
            },
        ),
        (
            "nuxt",
            TechInfo {
                patterns: vec!["nuxt.config.js"],
                icon: "🟢",
            },
        ),
        (
            "angular",
            TechInfo {
                patterns: vec!["angular.json", "main.ts"],
                icon: "🅰️",
            },
        ),
        (
            "flutter",
            TechInfo {
                patterns: vec!["pubspec.yaml", "*.dart"],
                icon: "🦋",
            },
        ),
        (
            "android",
            TechInfo {
                patterns: vec!["build.gradle", "AndroidManifest.xml"],
                icon: "🤖",
            },
        ),
        (
            "ios",
            TechInfo {
                patterns: vec!["*.xcodeproj", "*.xcworkspace"],
                icon: "🍎",
            },
        ),
        (
            "java",
            TechInfo {
                patterns: vec!["pom.xml", "*.java"],
                icon: "☕",
            },
        ),
        (
            "kotlin",
            TechInfo {
                patterns: vec!["*.kt"],
                icon: "🟣",
            },
        ),
        (
            "csharp",
            TechInfo {
                patterns: vec!["*.csproj", "Program.cs"],
                icon: "💎",
            },
        ),
        (
            "php",
            TechInfo {
                patterns: vec!["composer.json"],
                icon: "🐘",
            },
        ),
        (
            "ruby",
            TechInfo {
                patterns: vec!["Gemfile"],
                icon: "💎",
            },
        ),
        (
            "go",
            TechInfo {
                patterns: vec!["go.mod"],
                icon: "🦫",
            },
        ),
        (
            "rust",
            TechInfo {
                patterns: vec!["Cargo.toml"],
                icon: "🦀",
            },
        ),
        (
            "elixir",
            TechInfo {
                patterns: vec!["mix.exs"],
                icon: "💧",
            },
        ),
        (
            "dart",
            TechInfo {
                patterns: vec!["pubspec.yaml"],
                icon: "🎯",
            },
        ),
        (
            "r",
            TechInfo {
                patterns: vec!["*.R", "*.Rproj"],
                icon: "📊",
            },
        ),
        (
            "scala",
            TechInfo {
                patterns: vec!["build.sbt"],
                icon: "⚡",
            },
        ),
        (
            "docker",
            TechInfo {
                patterns: vec!["Dockerfile"],
                icon: "🐳",
            },
        ),
        (
            "kubernetes",
            TechInfo {
                patterns: vec!["k8s/", "helm/"],
                icon: "⚓",
            },
        ),
        (
            "terraform",
            TechInfo {
                patterns: vec!["*.tf"],
                icon: "🏗️",
            },
        ),
        (
            "ansible",
            TechInfo {
                patterns: vec!["ansible.cfg"],
                icon: "🤖",
            },
        ),
        (
            "github_actions",
            TechInfo {
                patterns: vec![".github/workflows/"],
                icon: "⚡",
            },
        ),
        (
            "gitlab_ci",
            TechInfo {
                patterns: vec![".gitlab-ci.yml"],
                icon: "🦊",
            },
        ),
        (
            "circleci",
            TechInfo {
                patterns: vec![".circleci/config.yml"],
                icon: "⭕",
            },
        ),
        (
            "deno",
            TechInfo {
                patterns: vec!["deno.json"],
                icon: "🦕",
            },
        ),
        (
            "bun",
            TechInfo {
                patterns: vec!["bun.lockb"],
                icon: "🍞",
            },
        ),
    ]
    .iter()
    .cloned()
    .collect();

    let mut detected_techs = HashSet::new();

    for entry in WalkDir::new(project_path) {
        if let Ok(entry) = entry {
            let rel_path = entry
                .path()
                .strip_prefix(project_path)
                .unwrap_or(entry.path());
            let rel_path_str = rel_path.to_string_lossy().replace("\\", "/");

            for (tech, tech_info) in &tech_indicators {
                for pattern in &tech_info.patterns {
                    if pattern.contains("**") || pattern.contains("*") {
                        if glob::Pattern::new(pattern)
                            .map(|p| p.matches(&rel_path_str))
                            .unwrap_or(false)
                        {
                            detected_techs.insert(format!("{} {}", tech_info.icon, tech));
                        }
                    } else if entry.file_name().to_string_lossy().to_lowercase()
                        == pattern.to_lowercase()
                    {
                        detected_techs.insert(format!("{} {}", tech_info.icon, tech));
                    }
                }
            }
        }
    }

    // Add implied techs
    if detected_techs.iter().any(|t| t.contains("nextjs")) {
        detected_techs.insert("⚛️ react".to_string());
        detected_techs.insert("📜 javascript".to_string());
        detected_techs.insert("📘 typescript".to_string());
    }
    if detected_techs.iter().any(|t| t.contains("nuxt")) {
        detected_techs.insert("🟢 vue".to_string());
        detected_techs.insert("📜 javascript".to_string());
        detected_techs.insert("📘 typescript".to_string());
    }

    let mut techs: Vec<String> = detected_techs.into_iter().collect();
    techs.sort();
    techs
}

pub fn get_extensions_by_tech(techs: &[String]) -> HashSet<String> {
    let tech_extensions: HashMap<&str, Vec<&str>> = [
        // Python & Data Science
        ("python", vec![".py", ".pyx", ".pyi"]),
        ("jupyter", vec![".ipynb"]),
        ("r", vec![".r", ".R", ".Rmd", ".Rproj"]),
        // JavaScript & Frontend
        ("javascript", vec![".js", ".jsx", ".mjs", ".cjs"]),
        ("typescript", vec![".ts", ".tsx"]),
        ("react", vec![".jsx", ".tsx", ".js", ".ts"]),
        ("vue", vec![".vue", ".js", ".ts"]),
        ("svelte", vec![".svelte"]),
        ("angular", vec![".ts", ".js", ".html", ".scss"]),
        ("nextjs", vec![".js", ".jsx", ".ts", ".tsx"]),
        ("nuxt", vec![".vue", ".js", ".ts"]),
        // Mobile
        ("flutter", vec![".dart"]),
        ("android", vec![".java", ".kt", ".xml"]),
        (
            "ios",
            vec![".swift", ".m", ".mm", ".h", ".xib", ".storyboard"],
        ),
        // Backend & Dev
        ("java", vec![".java", ".kt"]),
        ("kotlin", vec![".kt", ".kts"]),
        ("csharp", vec![".cs", ".vb"]),
        ("php", vec![".php"]),
        ("ruby", vec![".rb", ".erb"]),
        ("go", vec![".go"]),
        ("rust", vec![".rs"]),
        ("elixir", vec![".ex", ".exs"]),
        ("dart", vec![".dart"]),
        ("scala", vec![".scala", ".sc"]),
        // Infrastructure
        ("docker", vec!["Dockerfile", ".dockerignore"]),
        ("kubernetes", vec![".yaml", ".yml"]),
        ("terraform", vec![".tf", ".tf.json"]),
        ("ansible", vec![".yml", ".yaml"]),
        // CI/CD
        ("github_actions", vec![".yml"]),
        ("gitlab_ci", vec![".yml"]),
        ("circleci", vec![".yml"]),
        // Runtime Environments
        ("nodejs", vec![".js", ".mjs", ".cjs"]),
        ("bun", vec![".js", ".ts", ".jsx", ".tsx"]),
        ("deno", vec![".ts", ".tsx", ".js"]),
        // Config
        ("json", vec![".json"]),
        ("yaml", vec![".yml", ".yaml"]),
        ("toml", vec![".toml"]),
        ("xml", vec![".xml"]),
    ]
    .iter()
    .cloned()
    .collect();

    let mut extensions = HashSet::new();
    for tech in techs {
        // Remove icon from tech name if present
        let tech_name = tech.split_whitespace().last().unwrap_or(tech);
        if let Some(exts) = tech_extensions.get(tech_name) {
            extensions.extend(exts.iter().map(|&s| s.to_string()));
        }
    }
    extensions
}

pub fn get_exclude_patterns() -> (HashSet<String>, HashSet<String>) {
    let exclude_dirs: HashSet<String> = [
        // Dependencies & environments
        "node_modules", "vendor", "venv", "env", ".venv", ".env", ".mypy_cache", ".ruff_cache", 
        ".pytest_cache", "__pycache__", ".cache", "pip-wheel-metadata", "site-packages", 
        "deps", "packages", ".tox",

        // Build artifacts
        "dist", "build", "target", "out", "bin", "obj", ".eggs", "lib", "lib64", "generated",

        // Framework build folders
        ".next", ".nuxt", ".angular", "coverage", ".turbo", ".vercel", ".expo", ".parcel-cache",

        // Version control & IDE tools
        ".git", ".svn", ".hg", ".idea", ".vscode", ".vs", ".history", ".vscode-test",

        // Temp & OS folders
        "temp", "tmp", ".tmp", ".DS_Store", "__MACOSX", "Thumbs.db", "System Volume Information",

        // CI/CD & Docker volumes
        ".github", ".gitlab", ".circleci", ".docker", "logs", "log", "docker", "containers",

        // Database & sessions
        "db", "database", "sqlite", "sessions", "flask_session", "instance",
    ].iter().map(|&s| s.to_string()).collect();

    let exclude_files: HashSet<String> = [
        // Logs
        "*.log", "*.log.*", "*.out",

        // Package manager lock files
        "package-lock.json", "yarn.lock", "pnpm-lock.yaml", "composer.lock", "poetry.lock", "Cargo.lock",

        // Compiled/intermediate binaries
        "*.pyc", "*.pyo", "*.pyd", "*.class", "*.o", "*.so", "*.dll", "*.exe", "*.dylib", "*.a",

        // Media files
        "*.jpg", "*.jpeg", "*.png", "*.gif", "*.svg", "*.ico", "*.webp",
        "*.mp3", "*.wav", "*.mp4", "*.avi", "*.mov", "*.mkv", "*.flac", "*.ogg",

        // Fonts
        "*.ttf", "*.otf", "*.woff", "*.woff2",

        // Archives & compressed
        "*.zip", "*.tar", "*.gz", "*.rar", "*.7z", "*.bz2", "*.xz", "*.lz", "*.lzma",

        // Office / documents
        "*.pdf", "*.docx", "*.doc", "*.ppt", "*.pptx", "*.xls", "*.xlsx", "*.csv",

        // OS/system files
        ".DS_Store", "Thumbs.db", "desktop.ini", "ehthumbs.db", "Icon\r",

        // Misc config/cache
        "*.env", "*.env.*", "*.ini", "*.toml", "*.bak", "*.swp", "*.swo",
    ].iter().map(|&s| s.to_string()).collect();

    (exclude_dirs, exclude_files)
}

pub fn should_exclude_path(path: &Path, exclude_dirs: &HashSet<String>) -> bool {
    path.iter()
        .any(|component| exclude_dirs.contains(&component.to_string_lossy().to_lowercase()))
}

pub fn should_exclude_file(filename: &str, exclude_files: &HashSet<String>) -> bool {
    let filename_lower = filename.to_lowercase();
    exclude_files.iter().any(|pattern| {
        if pattern.starts_with("*.") {
            filename_lower.ends_with(&pattern[1..].to_lowercase())
        } else {
            filename_lower == pattern.to_lowercase()
        }
    })
} 
