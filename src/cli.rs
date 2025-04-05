//! This module is responsible for command line arguments deserialization and [`Config`] building.

use crate::colors::CompletionsMode;
use crate::colors::NamedColors;

use argh::FromArgs;

/// Hex color language server
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help", "help"))]
struct Args {
    /// mode of css named colors completions, can be one of: full, none, upper, lower. default: full
    #[argh(option, short = 'c', default = r#""upper".to_string()"#)]
    completions_mode: String,

    /// color collection used for completions, can be one of: css, colorhexa. default: css
    #[argh(option, short = 'o', default = r#""colorhexa".to_string()"#)]
    color_collection: String,

    /// version
    #[argh(switch)]
    version: bool,
}

/// Configuration struct based on CLI args.
pub struct Config {
    /// CSS named colors completions mode.
    pub completions_mode: CompletionsMode,

    /// ColorHexa colors by name.
    pub color_collection: NamedColors,

    /// version flag.
    pub version: bool,
}

impl Config {
    /// Creates a new [`Config`] from CLI args.
    pub fn new() -> Self {
        let args: Args = argh::from_env();

        Self {
            completions_mode: match args.completions_mode.to_lowercase().as_str() {
                "none" => CompletionsMode::None,
                "upper" => CompletionsMode::Uppercase,
                "lower" => CompletionsMode::Lowercase,
                _ => CompletionsMode::Full,
            },
            color_collection: match args.color_collection.to_lowercase().as_str() {
                "colorhexa" => NamedColors::ColorHexa,
                _ => NamedColors::Css,
            },
            version: args.version,
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}
