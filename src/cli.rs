//! This module is responsible for command line arguments deserialization and [`Config`] building.

use crate::colors::CompletionsMode;
use crate::colors::NamedColors;

use argh::FromArgs;

/// Hex color language server
#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help", "help"))]
struct Args {
    /// enable color variables completions
    #[argh(switch, short = 'a')]
    variable_completions: bool,

    /// mode of named colors completions, can be one of: full, none, upper, lower. default: upper
    #[argh(option, short = 'c', default = r#""upper".to_string()"#)]
    named_completions_mode: String,

    /// color collection used for completions, can be one of: css, colorhexa. default: colorhexa
    #[argh(option, short = 'o', default = r#""colorhexa".to_string()"#)]
    color_collection: String,

    /// version
    #[argh(switch, short = 'V')]
    version: bool,
}

/// Configuration struct based on CLI args.
pub struct Config {
    pub variable_completions: bool,

    /// CSS named colors completions mode.
    pub named_completions_mode: CompletionsMode,

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
            variable_completions: args.variable_completions,
            named_completions_mode: match args.named_completions_mode.to_lowercase().as_str() {
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
