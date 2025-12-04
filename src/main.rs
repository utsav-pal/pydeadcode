use clap::Parser;
use colored::*;
use std::path::PathBuf;
use pydeadcode::analyzer::DeadCodeAnalyzer;

#[derive(Parser, Debug)]
#[command(name = "pydeadcode")]
#[command(about = "Fast Python dead code finder, built in Rust", long_about = None)]
struct Args {
    /// Path to Python file or directory
    #[arg(value_name = "PATH")]
    paths: Vec<PathBuf>,

    /// Minimum confidence level (60-100)
    #[arg(short, long, default_value = "60")]
    min_confidence: u8,

    /// Sort results by size
    #[arg(short, long)]
    sort_by_size: bool,

    /// Output as JSON
    #[arg(short, long)]
    json: bool,

    /// Exclude patterns (comma-separated)
    #[arg(short, long)]
    exclude: Option<String>,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.paths.is_empty() {
        eprintln!("{}", "Error: No paths specified".red());
        std::process::exit(1);
    }

    let exclude_patterns: Vec<&str> = args
        .exclude
        .as_ref()
        .map(|s| s.split(',').collect())
        .unwrap_or_default();

    let mut analyzer = DeadCodeAnalyzer::new(args.min_confidence, exclude_patterns);

    for path in &args.paths {
        analyzer.analyze_path(path)?;
    }

    let results = analyzer.get_results();

    if results.is_empty() {
        println!("{}", "✓ No dead code found!".green());
        return Ok(());
    }

    if args.json {
        let json = serde_json::to_string_pretty(&results)?;
        println!("{}", json);
    } else {
        let mut sorted_results = results;
        if args.sort_by_size {
            sorted_results.sort_by_key(|r| std::cmp::Reverse(r.size));
        } else {
            sorted_results.sort_by(|a, b| {
                a.file.cmp(&b.file).then_with(|| a.line.cmp(&b.line))
            });
        }

        println!("{}", "\nDead Code Found:\n".yellow().bold());
        for result in &sorted_results {
            println!(
                "{}: {} - {} {} ({}% confidence)",
                result.file.bright_blue(),
                format!("line {}", result.line).cyan(),
                result.name.red(),
                format!("[{}]", result.code_type).dimmed(),
                result.confidence
            );
        }

        println!("\n{} dead code items found", sorted_results.len().to_string().yellow());
    }

    Ok(())
}