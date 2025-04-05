mod cli;

use cli::Config;
use uwu_colors::Backend;
use uwu_colors::colors;

use dashmap::DashMap;
use fancy_regex::Regex;
use tower_lsp::LspService;
use tower_lsp::Server;

const COLOR_REGEX: &str =
    r#"(["'])\#([0-9a-fA-F]{8}|[0-9a-fA-F]{6}|[0-9a-fA-F]{4}|[0-9a-fA-F]{3})\1"#;

#[tokio::main]
async fn main() {
    let config = Config::new();

    if config.version {
        println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

        return;
    }

    let stdin = tokio::io::stdin();
    let stdout = tokio::io::stdout();

    let documents = DashMap::new();
    let color_regex = Regex::new(COLOR_REGEX).unwrap();
    let completions =
        colors::named_colors_completions(&config.completions_mode, &config.color_collection);

    let (service, socket) = LspService::new(|client| Backend {
        client,
        documents,
        color_regex,
        completions,
    });

    Server::new(stdin, stdout, socket).serve(service).await;
}
