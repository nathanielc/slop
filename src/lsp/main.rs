use tower_lsp::{LspService, Server};

use backend::Slop;

mod backend;

pub type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

async fn run() {
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
    run().await;
}
