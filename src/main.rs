use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

use regex::Regex;
use tower_lsp::Client;
use tower_lsp::LanguageServer;
use tower_lsp::LspService;
use tower_lsp::Server;
use tower_lsp::jsonrpc::Error;
use tower_lsp::jsonrpc::Result;
use tower_lsp::lsp_types::*;

#[derive(Debug)]
struct Backend {
    client: Client,
    documents: Arc<Mutex<HashMap<Url, String>>>,
    color_regex: Regex,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    async fn initialize(&self, _: InitializeParams) -> Result<InitializeResult> {
        Ok(InitializeResult {
            capabilities: ServerCapabilities {
                text_document_sync: Some(TextDocumentSyncKind::FULL.into()),
                color_provider: Some(ColorProviderCapability::Simple(true)),
                ..Default::default()
            },
            ..Default::default()
        })
    }

    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized")
            .await;
    }

    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.documents.lock().unwrap().insert(uri, text);
    }

    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let mut docs = self.documents.lock().unwrap();

        if let Some(text) = docs.get_mut(&uri) {
            text.clone_from(&params.content_changes[0].text);
        }
    }

    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        let uri = params.text_document.uri;
        let docs = self.documents.lock().unwrap();
        let text = docs
            .get(&uri)
            .ok_or_else(|| Error::invalid_params("document not found"))?;

        let mut colors = Vec::new();
        for (line_num, line) in text.lines().enumerate() {
            for color_match in self.color_regex.find_iter(line) {
                let (start, end) = (color_match.start(), color_match.end());

                let (color_start, color_end) = (
                    u32::try_from(line[..start].encode_utf16().count()).unwrap(),
                    u32::try_from(line[..end].encode_utf16().count()).unwrap(),
                );

                let line_num = u32::try_from(line_num).unwrap();

                let color_str = color_match.as_str();
                let color_str_length = color_str.len();

                let color = parse_color(&color_str[2..color_str_length], color_str_length - 3);

                colors.push(ColorInformation {
                    color,
                    range: Range {
                        start: Position::new(line_num, color_start),
                        end: Position::new(line_num, color_end),
                    },
                });
            }
        }

        Ok(colors)
    }
}

fn parse_color(hex: &str, length: usize) -> Color {
    let (red, green, blue, alpha) = match length {
        3 => (
            f32::from(u8::from_str_radix(&hex[0..1].repeat(2), 16).unwrap()) / 255f32,
            f32::from(u8::from_str_radix(&hex[1..2].repeat(2), 16).unwrap()) / 255f32,
            f32::from(u8::from_str_radix(&hex[2..3].repeat(2), 16).unwrap()) / 255f32,
            1f32,
        ),
        6 => (
            f32::from(u8::from_str_radix(&hex[0..2], 16).unwrap()) / 255f32,
            f32::from(u8::from_str_radix(&hex[2..4], 16).unwrap()) / 255f32,
            f32::from(u8::from_str_radix(&hex[4..6], 16).unwrap()) / 255f32,
            1f32,
        ),
        8 => (
            f32::from(u8::from_str_radix(&hex[0..2], 16).unwrap()) / 255f32,
            f32::from(u8::from_str_radix(&hex[2..4], 16).unwrap()) / 255f32,
            f32::from(u8::from_str_radix(&hex[4..6], 16).unwrap()) / 255f32,
            f32::from(u8::from_str_radix(&hex[6..8], 16).unwrap()) / 255f32,
        ),
        _ => (0f32, 0f32, 0f32, 0f32),
    };

    Color {
        red,
        green,
        blue,
        alpha,
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let documents = Arc::new(Mutex::new(HashMap::new()));
    let color_regex = Regex::new(r#""\#([0-9a-fA-F]{8}|[0-9a-fA-F]{6}|[0-9a-fA-F]{3})""#).unwrap();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents,
        color_regex,
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
