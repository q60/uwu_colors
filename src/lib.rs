#![doc = include_str!("../readme.md")]

pub mod cli;
pub mod colors;

use dashmap::DashMap;
use fancy_regex::Regex;
use tower_lsp::Client;
use tower_lsp::LanguageServer;
use tower_lsp::jsonrpc::Error;
use tower_lsp::jsonrpc::Result;

#[allow(clippy::wildcard_imports, reason = "there is a ton of types")]
use tower_lsp::lsp_types::*;

/// Backend structure.
#[derive(Debug)]
pub struct Backend {
    /// LSP client.
    pub client: Client,

    /// Contains all opened documents (files).
    pub documents: DashMap<Url, String>,

    /// Regex to find colors in files.
    pub color_regex: Regex,

    /// CSS named colors completions
    pub completions: Option<CompletionResponse>,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    /// Initialize request handler. Initializes the language server with parameters.
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncKind::FULL.into()),
                color_provider: Some(ColorProviderCapability::Simple(true)),
                completion_provider: Some(CompletionOptions {
                    trigger_characters: Some(vec!["#".to_string()]),
                    ..CompletionOptions::default()
                }),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    /// Initialized notification.
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized")
            .await;
    }

    /// Shutdown request handler.
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    /// Inserts a newly opened file into the `documents` field.
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.documents.insert(uri, text);
    }

    /// Updates a file on its change.
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let docs = &self.documents;

        if let Some(mut text) = docs.get_mut(&uri) {
            text.clone_from(&params.content_changes[0].text);
        }
    }

    /// Named colors completions handler.
    async fn completion(&self, params: CompletionParams) -> Result<Option<CompletionResponse>> {
        if let Some(completions) = &self.completions {
            let Position {
                line: line_pos,
                character: char_pos,
            } = params.text_document_position.position;

            // early return on lines starting with `#`
            if char_pos <= 1 {
                return Ok(None);
            }

            let Some(context) = params.context else {
                return Ok(None);
            };

            if Some("#".to_string()) == context.trigger_character {
                let indented_hash_regex = Regex::new(r"^\s+#").expect("perfectly valid regex");

                let uri = params.text_document_position.text_document.uri;

                let Some(document) = &self.documents.get(&uri) else {
                    return Ok(None);
                };

                let line = document.lines().collect::<Vec<&str>>()[line_pos as usize];

                if !indented_hash_regex
                    .is_match(line)
                    .expect("perfectly valid regex")
                {
                    return Ok(Some(completions.clone()));
                }
            }
        }

        Ok(None)
    }

    /// Document color request handler.
    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        // current file's URL
        let uri = params.text_document.uri;
        // all the opened files
        let docs = &self.documents;

        let colors: Vec<ColorInformation> = docs
            .get(&uri)
            .ok_or_else(|| Error::invalid_params("document not found"))?
            .lines()
            .enumerate()
            .flat_map(|(line_num, line)| {
                colors::colors_in_line_iter(&self.color_regex, line_num, line)
            })
            .collect();

        Ok(colors)
    }
}
