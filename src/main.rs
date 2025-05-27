mod constants;
mod aggregator;
mod detector;
mod tree_generator;

use clap::Parser;
use std::env;
use std::io::{self, Write};
use anyhow::Result;

#[derive(Parser)]
#[command(
    author = "Hai Pham Ngoc <ngochai285nd@gmail.com>",
    version = "v0.0.8",
    about = "A tool for analyzing and dumping source code from project directories",
    long_about = "Repo Dump is a powerful tool that helps you analyze and extract source code from project directories. It supports multiple languages and provides detailed insights about your codebase.",
    arg_required_else_help = false
)]
struct Args {
    /// Project path to analyze
    #[arg(short, long, help = "Path to the project directory you want to analyze")]
    path: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();
    
    // Print the logo and welcome message
    println!("{}", constants::LOGO);
    println!("{}", "=".repeat(40));
    println!("{}", "Welcome to Repo Dump!");
    println!(
        "{}",
        "This is a tool for analyzing and dumping source code from project directories."
    );
    println!("{}", "=".repeat(40));

    // Select the language
    let mut language = String::new();
    print!("Select language ([en]/vi): ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut language).unwrap();
    let language = language.trim().to_lowercase();

    // Validate the language
    let text = if language != "en" && language != "vi" {
        println!("⚠️ Invalid language. Language will be set to English.");
        constants::TEXT_EN
    } else {
        constants::TEXT_VI
    };

    // Parse the arguments and get the folder path
    let path = args.path.unwrap_or_else(|| {
        print!("{}", text.input_repo_path);
        io::stdout().flush().unwrap();
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim().to_string();

        if input.is_empty() {
            env::current_dir()
                .expect("❌ Failed to get current directory")
                .to_string_lossy()
                .into_owned()
        } else {
            input
        }
    });

    let repo_path = std::path::PathBuf::from(path);
    let success = aggregator::aggregate_repo(&repo_path, &text)?;

    if success {
        println!("{}", text.done);
    } else {
        println!("{}", text.error);
        std::process::exit(1);
    }

    Ok(())
}
