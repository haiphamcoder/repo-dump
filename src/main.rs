mod constants;
mod aggregator;
mod detector;

use clap::Parser;
use std::env;
use std::io::{self, Write};
use anyhow::Result;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Project path to analyze
    #[arg(short, long)]
    path: Option<String>,
}

fn main() -> Result<()> {
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
    let args = Args::parse();
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
