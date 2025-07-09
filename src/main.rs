// src/main.rs
/*
 * Main executable for GenAITestnetProtocolProtocolNext
 */

use clap::Parser;
use genaitestnetprotocolprotocolnext::{Result, run};

#[derive(Parser)]
#[command(version, about = "GenAITestnetProtocolProtocolNext - A Rust implementation")]
struct Cli {
    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = Cli::parse();
    run(args.verbose)
}
