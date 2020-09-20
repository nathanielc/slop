use std::collections::HashMap;

use codespan::{FileId, Files};
use codespan_lsp::{byte_span_to_range, range_to_byte_span};
use lsp_types;
use tokio::sync::Mutex;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

use lalrpop_util::ParseError;

use slop::{parse, Error};

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
            client: client,
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
                    TextDocumentSyncKind::Incremental,
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
                //document_formatting_provider: Some(true),
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
                source = change.text;
            } else if let Some(range) = change.range {
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

fn get_diagnostics(state: &State, uri: &Url, id: FileId) -> Vec<Diagnostic> {
    let source = state.files.source(id);
    match parse(source) {
        Ok(expr) => Vec::new(),
        Err(err) => match err {
            ParseError::InvalidToken { location } => {
                let span = location..(location + 1);
                let range = byte_span_to_range(&state.files, id, span).unwrap();
                return vec![Diagnostic {
                    range: convert_range(range),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: None,
                    message: format!("{}", &err),
                    related_information: None,
                    tags: None,
                }];
            }
            ParseError::UnrecognizedEOF {
                location,
                ref expected,
            } => {
                let span = location..(location + 1);
                let range = byte_span_to_range(&state.files, id, span).unwrap();
                return vec![Diagnostic {
                    range: convert_range(range),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: None,
                    message: format!("{}", &err),
                    related_information: None,
                    tags: None,
                }];
            }
            ParseError::UnrecognizedToken {
                ref token,
                ref expected,
            } => {
                let span = token.0..token.2;
                let range = byte_span_to_range(&state.files, id, span).unwrap();
                return vec![Diagnostic {
                    range: convert_range(range),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: None,
                    message: format!("{}", &err),
                    related_information: None,
                    tags: None,
                }];
            }
            ParseError::ExtraToken { ref token } => {
                let span = token.0..token.2;
                let range = byte_span_to_range(&state.files, id, span).unwrap();
                return vec![Diagnostic {
                    range: convert_range(range),
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: None,
                    message: format!("{}", &err),
                    related_information: None,
                    tags: None,
                }];
            }
            ParseError::User { error } => {
                return vec![Diagnostic {
                    range: Range {
                        start: Position {
                            line: 1,
                            character: 1,
                        },
                        end: Position {
                            line: 1,
                            character: 1,
                        },
                    },
                    severity: Some(DiagnosticSeverity::Error),
                    code: None,
                    source: None,
                    message: format!("{}", &error),
                    related_information: None,
                    tags: None,
                }];
            }
        },
    }
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
