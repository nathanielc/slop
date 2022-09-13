use std::{
    fs::{read_to_string, write},
    path::PathBuf,
};

use anyhow::Result;

use clap::{Args, Parser, Subcommand};
use slop::format;

/// Simple program to greet a person
#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct CLI {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    Fmt(FmtArgs),
}
#[derive(Args)]
struct FmtArgs {
    #[clap(parse(from_os_str))]
    file: PathBuf,
}

fn main() -> Result<()> {
    let args = CLI::parse();
    match args.command {
        Command::Fmt(args) => fmt(args.file),
    }
}

fn fmt(file: PathBuf) -> Result<()> {
    let contents = read_to_string(&file)?;
    let fmt_contents = format(&contents)?;
    println!("{}", fmt_contents);
    write(file, fmt_contents)?;
    Ok(())
}
