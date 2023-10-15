use std::path::PathBuf;

use anyhow::Result;
use clap::{command, Args, Parser, Subcommand};
use tokio::{fs, io::AsyncWriteExt};

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
    /// Check if a slop file is valid.
    Title(TitleOpts),
    /// Render recipe to an svg file
    Render(RenderOpts),
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

#[derive(Args, Debug)]
struct TitleOpts {
    /// Path to slop file
    #[arg()]
    file: PathBuf,
}

#[derive(Args, Debug)]
struct RenderOpts {
    /// Path to slop file
    #[arg()]
    in_file: PathBuf,
    /// Path to output svg file
    #[arg()]
    out_file: PathBuf,
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
        Command::Title(opts) => {
            let source = fs::read_to_string(opts.file).await?;
            let ast = slop::parse(&source)?;
            println!("{}", ast.recipes[0].title.as_ref().unwrap());
            Ok(())
        }
        Command::Render(opts) => {
            let source = fs::read_to_string(opts.in_file).await?;
            let ast = slop::parse(&source)?;
            let sem = slop::semantic::convert_source_file(ast);
            let svg = slop::svg::to_svg(&sem);
            let mut f = fs::File::create(opts.out_file).await?;
            f.write_all(&svg).await?;
            Ok(())
        }
    }
}
