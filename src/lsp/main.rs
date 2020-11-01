use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use tower_lsp::{LspService, Server};

extern crate log_panics;

use backend::Slop;

mod backend;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn run() {
    CombinedLogger::init(vec![
        TermLogger::new(LevelFilter::Warn, Config::default(), TerminalMode::Mixed).unwrap(),
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("/tmp/slop-lsp.log").unwrap(),
        ),
    ])
    .unwrap();
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let (service, messages) = LspService::new(|client| Slop::new(client));
    Server::new(stdin, stdout)
        .interleave(messages)
        .serve(service)
        .await;
}

#[tokio::main]
async fn main() {
    log_panics::init();
    run().await;
}
