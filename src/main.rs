mod walker;
mod filter;
mod hasher;
mod group;
mod quarantine;
mod report;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "dupe")]
#[command(about = "Duplicate file scanner", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Scan {
        #[arg(short, long)]
        path: String,
        #[arg(short = 't', long)]
        pattern: Option<String>,
    },
    Clean {
        #[arg(short, long)]
        report: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { path, pattern } => {
            let files = walker::scan_dir(&path);
            let filtered = filter::apply_filter(files, pattern.as_deref());
            let hashed = hasher::hash_files(filtered);
            let groups = group::group_by_hash(hashed);
            let report = report::Report::new(groups);
            report.save("report.json");
            println!("Scan complete. Report written to report.json");
        }
        Commands::Clean { report } => {
            let report = report::Report::load(&report);
            quarantine::clean_duplicates(&report);
            println!("Clean complete.");
        }
    }
}
