use std::collections::HashMap;

use codespan::{FileId, Files};
use codespan_lsp::{make_lsp_diagnostic, range_to_byte_span};
use jsonrpc_core::Result as RpcResult;
use log::info;
use ast::Recipe;
use std::sync::Mutex;
use tower_lsp::lsp_types::*;
use tower_lsp::{Client, LanguageServer};

#[derive(Debug)]
struct State {
    sources: HashMap<Url, FileId>,
    files: Files<String>,
}

#[derive(Debug)]
pub struct Slop {
    state: Mutex<State>,
}

impl Slop {
    pub fn new() -> Self {
        Slop {
            state: Mutex::new(State {
                sources: HashMap::new(),
                files: Files::new(),
            }),
        }
    }
}

#[tower_lsp::async_trait]
impl LanguageServer for Slop {
    fn initialize(&self, _: &Client, _: InitializeParams) -> RpcResult<InitializeResult> {
        Ok(InitializeResult {
            server_info: None,
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncCapability::Kind(
                    TextDocumentSyncKind::Incremental,
                )),
                completion_provider: Some(CompletionOptions {
                    resolve_provider: Some(true),
                    trigger_characters: Some(vec![".".to_string()]),
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                }),
                signature_help_provider: Some(SignatureHelpOptions {
                    trigger_characters: None,
                    retrigger_characters: None,
                    work_done_progress_options: WorkDoneProgressOptions::default(),
                }),
                hover_provider: Some(true),
                document_formatting_provider: Some(true),
                document_highlight_provider: Some(true),
                document_symbol_provider: Some(true),
                workspace_symbol_provider: Some(true),
                definition_provider: Some(true),
                ..ServerCapabilities::default()
            },
        })
    }

    async fn shutdown(&self) -> RpcResult<()> {
        Ok(())
    }

    async fn did_open(&self, client: &Client, params: DidOpenTextDocumentParams) {
        let mut state = self.state.lock().await;
        let id = get_or_insert_source(&mut state, &params.text_document);
        let diags = get_diagnostics(&state, &params.text_document.uri, id);
        client.publish_diagnostics(params.text_document.uri, diags, None);
    }

    async fn did_change(&self, client: &Client, params: DidChangeTextDocumentParams) {
        let mut state = self.state.lock().await;
        let id = reload_source(&mut state, &params.text_document, params.content_changes);
        let diags = get_diagnostics(&state, &params.text_document.uri, id);
        client.publish_diagnostics(params.text_document.uri, diags, None);
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
                let span = range_to_byte_span(&state.files, *id, &range).unwrap_or_default();
                let range = (span.start().to_usize())..(span.end().to_usize());
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
    match source.parse::<SourceFile>() {
        Ok(expr) => {
            info!("parsed expression: {}", expr);
            Vec::new()
        }
        Err(err) => {
            info!("expression has errors: {}", err);
            err.to_diagnostics(id)
                .into_iter()
                .map(|d| make_lsp_diagnostic(&state.files, None, d, |_| Ok(uri.clone())))
                .collect::<Result<Vec<_>, _>>()
                .unwrap()
        }
    }
}

