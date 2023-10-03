use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Args, Parser, Subcommand};
use tokio::fs;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Debug)]
enum Command {
    /// Format a slop file
    Fmt(FmtOpts),
    /// Check if a slop file is valid.
    Check(CheckOpts),
}

#[derive(Args, Debug)]
struct FmtOpts {
    /// Path to slop file
    #[arg()]
    file: PathBuf,
}

#[derive(Args, Debug)]
struct CheckOpts {
    /// Path to slop file
    #[arg()]
    file: PathBuf,
}
pub async fn run() -> Result<()> {
    let args = Cli::parse();
    match args.command {
        Command::Fmt(opts) => {
            let source = fs::read_to_string(opts.file).await?;
            let formatted = slop::format(&source)?;
            println!("{formatted}");
            Ok(())
        }
        Command::Check(opts) => {
            let source = fs::read_to_string(opts.file).await?;
            let _ast = slop::parse(&source)?;
            Ok(())
        }
    }
}
