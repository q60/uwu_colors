#![doc = include_str!("../readme.md")]

use dashmap::DashMap;
use fancy_regex::Regex;
use tower_lsp::Client;
use tower_lsp::LanguageServer;
use tower_lsp::LspService;
use tower_lsp::Server;
use tower_lsp::jsonrpc::Error;
use tower_lsp::jsonrpc::Result;

#[allow(clippy::wildcard_imports, reason = "there is a ton of types")]
use tower_lsp::lsp_types::*;

/// Backend structure.
#[derive(Debug)]
struct Backend {
    /// LSP client.
    client: Client,

    /// Contains all opened documents (files).
    documents: DashMap<Url, String>,

    /// Regex to find colors in files.
    color_regex: Regex,
}

#[tower_lsp::async_trait]
impl LanguageServer for Backend {
    // Initialize request handler. Initializes the language server with parameters.
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

    // Initialized notification.
    async fn initialized(&self, _: InitializedParams) {
        self.client
            .log_message(MessageType::INFO, "server initialized")
            .await;
    }

    // Shutdown request handler.
    async fn shutdown(&self) -> Result<()> {
        Ok(())
    }

    // Inserts a newly opened file into the `documents` field.
    async fn did_open(&self, params: DidOpenTextDocumentParams) {
        let uri = params.text_document.uri;
        let text = params.text_document.text;

        self.documents.insert(uri, text);
    }

    // Updates a file on its change.
    async fn did_change(&self, params: DidChangeTextDocumentParams) {
        let uri = params.text_document.uri;
        let docs = &self.documents;

        if let Some(mut text) = docs.get_mut(&uri) {
            text.clone_from(&params.content_changes[0].text);
        }
    }

    // Document color request handler.
    async fn document_color(&self, params: DocumentColorParams) -> Result<Vec<ColorInformation>> {
        // current file's URL
        let uri = params.text_document.uri;
        // all the opened files
        let docs = &self.documents;
        // current file's text
        let text = docs
            .get(&uri)
            .ok_or_else(|| Error::invalid_params("document not found"))?;

        let mut colors = Vec::new();

        for (line_num, line) in text.lines().enumerate() {
            for captures in self.color_regex.captures_iter(line).map(|c| c.unwrap()) {
                // the first capture is a full matching string
                // the second capture holds quotation marks
                // the third one holds hex value
                let (full_hex, color_match) = (captures.get(0).unwrap(), captures.get(2).unwrap());
                // hex color offsets in bytes
                let (start, end) = (full_hex.start(), full_hex.end());

                // ColorInformation needs these offsets in characters, not in bytes
                let (color_start, color_end) = (
                    u32::try_from(line[..start].encode_utf16().count()).unwrap(),
                    u32::try_from(line[..end].encode_utf16().count()).unwrap(),
                );

                // current line number
                let line_num = u32::try_from(line_num).unwrap();

                // parsing string slices of hex colors to the Color structure
                let color = parse_color(color_match.as_str()).unwrap();

                // start tells the LSP client that the color starts *here* on *this* line
                // end tells the client that the color ends *here* on *this* line
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

/// Color parsing function.
///
/// It parses a string slice and returns a [`Color`].
///
/// # Exceptions
///
/// It returns a [`None`] when the input string length is not in [3, 6, 8].
///
/// # Panics
///
/// This function panics when the input string has non hexadecimal characters.
///
/// It will also panic if the string is bigger than 4 bytes.
fn parse_color(hex_str: &str) -> Option<Color> {
    let str_length = hex_str.len();
    let hex = u32::from_str_radix(hex_str, 16).unwrap();

    match str_length {
        3 => {
            let (r, g, b) = ((hex & 0xF00) >> 8, (hex & 0x0F0) >> 4, hex & 0x00F);

            #[expect(
                clippy::cast_possible_truncation,
                reason = "r, g, b values are always less than 0x10 which is less than u8::MAX"
            )]
            Some(Color {
                red: f32::from(r as u8) / 15f32,
                green: f32::from(g as u8) / 15f32,
                blue: f32::from(b as u8) / 15f32,
                alpha: 1f32,
            })
        }
        6 | 8 => {
            let (r, g, b, a) = if str_length == 6 {
                let [_, r, g, b] = hex.to_be_bytes();

                (r, g, b, 0xFFu8)
            } else {
                // [r, g, b, a]
                hex.to_be_bytes().into()
            };

            Some(Color {
                red: f32::from(r) / 255f32,
                green: f32::from(g) / 255f32,
                blue: f32::from(b) / 255f32,
                alpha: f32::from(a) / 255f32,
            })
        }
        _ => None,
    }
}

#[tokio::main]
async fn main() {
    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let documents = DashMap::new();
    let color_regex =
        Regex::new(r#"(["'])\#([0-9a-fA-F]{8}|[0-9a-fA-F]{6}|[0-9a-fA-F]{3})\1"#).unwrap();

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents,
        color_regex,
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_color_test() {
        let cases = [
            ("369", (0.2, 0.4, 0.6, 1.0)),
            ("336699", (0.2, 0.4, 0.6, 1.0)),
            ("336699CC", (0.2, 0.4, 0.6, 0.8)),
            ("1A4D80B3", (0.101960786, 0.3019608, 0.5019608, 0.7019608)),
            ("854D91B8", (0.52156866, 0.3019608, 0.5686275, 0.72156864)),
        ];

        for (hex, color) in cases {
            assert_eq!(Some(to_color(color)), parse_color(hex))
        }
    }

    #[test]
    fn parse_color_fail_incorrect_length() {
        assert_eq!(None, parse_color("abcdefe"));
    }

    #[test]
    #[should_panic]
    fn parse_color_panic_cant_parse_hex() {
        parse_color("arstgm");
    }

    #[test]
    #[should_panic]
    fn parse_color_panic_str_too_long() {
        parse_color("arstgmgtsra");
    }

    fn to_color(colors: (f32, f32, f32, f32)) -> Color {
        let (red, green, blue, alpha) = colors;

        Color {
            red,
            green,
            blue,
            alpha,
        }
    }
}
