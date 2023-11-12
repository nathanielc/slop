use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode, WriteLogger};
use std::fs::File;
use tower_lsp::{LspService, Server};

extern crate log_panics;

use backend::Slop;

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

    let (service, messages) = LspService::new(Slop::new);
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
mod backend {
    use log::info;
    use std::collections::HashMap;

    use codespan::{FileId, Files};
    use codespan_lsp::{byte_span_to_range, range_to_byte_span};
    use tokio::sync::Mutex;
    use tower_lsp::jsonrpc::{Error, Result};
    use tower_lsp::lsp_types::*;
    use tower_lsp::{Client, LanguageServer};

    use slop::{compile, format};

    #[derive(Debug)]
    struct State {
        sources: HashMap<Url, FileId>,
        files: Files<String>,
    }

    #[derive(Debug)]
    pub struct Slop {
        client: Client,
        state: Mutex<State>,
    }

    impl Slop {
        pub fn new(client: Client) -> Self {
            Slop {
                client,
                state: Mutex::new(State {
                    sources: HashMap::new(),
                    files: Files::new(),
                }),
            }
        }
    }

    #[tower_lsp::async_trait]
    impl LanguageServer for Slop {
        async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
            Ok(InitializeResult {
                server_info: None,
                capabilities: ServerCapabilities {
                    text_document_sync: Some(TextDocumentSyncCapability::Kind(
                        TextDocumentSyncKind::Full,
                    )),
                    //completion_provider: Some(CompletionOptions {
                    //    resolve_provider: Some(true),
                    //    trigger_characters: Some(vec![".".to_string()]),
                    //    work_done_progress_options: WorkDoneProgressOptions::default(),
                    //}),
                    //signature_help_provider: Some(SignatureHelpOptions {
                    //    trigger_characters: None,
                    //    retrigger_characters: None,
                    //    work_done_progress_options: WorkDoneProgressOptions::default(),
                    //}),
                    //hover_provider: Some(true),
                    document_formatting_provider: Some(true),
                    //document_highlight_provider: Some(true),
                    //document_symbol_provider: Some(true),
                    //workspace_symbol_provider: Some(true),
                    //definition_provider: Some(true),
                    ..ServerCapabilities::default()
                },
            })
        }

        async fn shutdown(&self) -> Result<()> {
            Ok(())
        }

        async fn did_open(&self, params: DidOpenTextDocumentParams) {
            info!("did_open {}", &params.text_document.uri);
            let mut state = self.state.lock().await;
            let id = get_or_insert_source(&mut state, &params.text_document);
            let diags = get_diagnostics(&state, &params.text_document.uri, id);
            self.client
                .publish_diagnostics(params.text_document.uri, diags, None)
                .await;
        }

        async fn did_change(&self, params: DidChangeTextDocumentParams) {
            let mut state = self.state.lock().await;
            let id = reload_source(&mut state, &params.text_document, params.content_changes);
            let diags = get_diagnostics(&state, &params.text_document.uri, id);
            self.client
                .publish_diagnostics(params.text_document.uri, diags, None)
                .await;
        }
        async fn did_save(&self, params: DidSaveTextDocumentParams) {
            let mut state = self.state.lock().await;
            let id = save_source(&mut state, &params.text_document, &params.text);
            let diags = get_diagnostics(&state, &params.text_document.uri, id);
            self.client
                .publish_diagnostics(params.text_document.uri, diags, None)
                .await;
        }
        async fn formatting(
            &self,
            params: DocumentFormattingParams,
        ) -> Result<Option<Vec<TextEdit>>> {
            let state = self.state.lock().await;
            if let Some(id) = state.sources.get(&params.text_document.uri) {
                let src = state.files.source(*id);
                let (new_text, _errors) = format(src);
                let span = 0..(src.len());
                let range = byte_span_to_range(&state.files, *id, span);
                if let Ok(range) = range {
                    return Ok(Some(vec![TextEdit {
                        range: convert_range(range),
                        new_text,
                    }]));
                }
                Ok(None)
            } else {
                Err(Error::invalid_request())
            }
        }
    }

    fn get_or_insert_source(state: &mut State, document: &TextDocumentItem) -> FileId {
        if let Some(id) = state.sources.get(&document.uri) {
            *id
        } else {
            let id = state
                .files
                .add(document.uri.to_string(), document.text.clone());
            state.sources.insert(document.uri.clone(), id);
            id
        }
    }

    fn reload_source(
        state: &mut State,
        document: &VersionedTextDocumentIdentifier,
        changes: Vec<TextDocumentContentChangeEvent>,
    ) -> FileId {
        if let Some(id) = state.sources.get(&document.uri) {
            let mut source = state.files.source(*id).to_owned();
            for change in changes {
                if let (None, None) = (change.range, change.range_length) {
                    info!("reload: full source sync");
                    source = change.text;
                } else if let Some(range) = change.range {
                    info!("reload: partial source sync");
                    let lrange = lsp_types::Range {
                        start: lsp_types::Position {
                            line: range.start.line,
                            character: range.start.character,
                        },
                        end: lsp_types::Position {
                            line: range.end.line,
                            character: range.end.character,
                        },
                    };
                    let span = range_to_byte_span(&state.files, *id, &lrange).unwrap_or_default();
                    let range = (span.start)..(span.end);
                    source.replace_range(range, &change.text);
                }
            }
            state.files.update(*id, source);
            *id
        } else {
            panic!("attempted to reload source that does not exist");
        }
    }
    fn save_source(
        state: &mut State,
        document: &TextDocumentIdentifier,
        text: &Option<String>,
    ) -> FileId {
        if let Some(id) = state.sources.get(&document.uri) {
            let mut source = state.files.source(*id).to_owned();
            if let Some(ref text) = text {
                info!("save: full source sync");
                source = text.to_owned();
            }
            state.files.update(*id, source);
            *id
        } else {
            panic!("attempted to save source that does not exist");
        }
    }

    fn get_diagnostics(state: &State, _uri: &Url, id: FileId) -> Vec<Diagnostic> {
        let source = state.files.source(id);
        let (_, errors) = compile(source);
        errors
            .into_iter()
            .map(|err| match err {
                slop::Error::ParseError(slop::ParseError::UnexpectedToken(token, position)) => {
                    let range = byte_span_to_range(&state.files, id, position).unwrap();
                    Diagnostic {
                        range: convert_range(range),
                        severity: Some(DiagnosticSeverity::Error),
                        code: None,
                        source: None,
                        message: format!("unexpected token: {token}"),
                        related_information: None,
                        tags: None,
                    }
                }
                slop::Error::ParseError(slop::ParseError::UnexpectedEOF) => {
                    let range =
                        byte_span_to_range(&state.files, id, source.len() - 1..source.len())
                            .unwrap();
                    Diagnostic {
                        range: convert_range(range),
                        severity: Some(DiagnosticSeverity::Error),
                        code: None,
                        source: None,
                        message: "unexpected end of file".to_string(),
                        related_information: None,
                        tags: None,
                    }
                }
                slop::Error::CompilationError(slop::CompilationError::MissingOperand(position)) => {
                    let range = byte_span_to_range(&state.files, id, position).unwrap();
                    Diagnostic {
                        range: convert_range(range),
                        severity: Some(DiagnosticSeverity::Error),
                        code: None,
                        source: None,
                        message: "missing operand".to_string(),
                        related_information: None,
                        tags: None,
                    }
                }
                slop::Error::CompilationError(slop::CompilationError::UnusedOperands(
                    count,
                    position,
                )) => {
                    let range = byte_span_to_range(&state.files, id, position).unwrap();
                    Diagnostic {
                        range: convert_range(range),
                        severity: Some(DiagnosticSeverity::Error),
                        code: None,
                        source: None,
                        message: format!("found {count} unused operands"),
                        related_information: None,
                        tags: None,
                    }
                }
            })
            .collect()
    }

    fn convert_range(r: lsp_types::Range) -> Range {
        Range {
            start: Position {
                line: r.start.line,
                character: r.start.character,
            },
            end: Position {
                line: r.end.line,
                character: r.end.character,
            },
        }
    }
}
