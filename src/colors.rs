//! Module with helper functions related to hex colors parsing.

mod named_colors;

pub use named_colors::NamedColors;

use crate::Color;
use crate::ColorInformation;
use crate::CompletionItem;
use crate::CompletionItemKind;
use crate::CompletionResponse;
use crate::Position;
use crate::Range;
use crate::Regex;

/// Named colors completions mode.
pub enum CompletionsMode {
    /// both the uppercase hex strings and the lowercase ones.
    Full,

    /// no completions.
    None,

    /// only the uppercase hex colors after completion.
    Uppercase,

    /// only the lowercase hex colors after completion.
    Lowercase,
}

/// Completion builder function.
///
/// Builds a [`CompletionResponse`] of a [`Vec`] of CSS named colors.
pub fn named_colors_completions(
    mode: &CompletionsMode,
    colors: &NamedColors,
) -> Option<CompletionResponse> {
    if matches!(mode, CompletionsMode::None) {
        return None;
    }

    let colors = colors.get();

    let completions: Vec<CompletionItem> =
        colors
            .iter()
            .fold(Vec::new(), |mut acc, (name, hex)| match mode {
                CompletionsMode::Full => {
                    let lowercase_colors = ((*name).to_string(), (*hex).to_string());

                    acc.push(completion_item(lowercase_colors.0, lowercase_colors.1));

                    if hex.matches(char::is_alphabetic).count() > 0 {
                        let uppercase_colors = (name.to_uppercase(), hex.to_uppercase());

                        acc.push(completion_item(uppercase_colors.0, uppercase_colors.1));
                    }

                    acc
                }
                CompletionsMode::Uppercase => {
                    let uppercase_colors = ((*name).to_string(), hex.to_uppercase());

                    acc.push(completion_item(uppercase_colors.0, uppercase_colors.1));

                    acc
                }
                CompletionsMode::Lowercase => {
                    let lowercase_colors = ((*name).to_string(), (*hex).to_string());

                    acc.push(completion_item(lowercase_colors.0, lowercase_colors.1));

                    acc
                }
                CompletionsMode::None => acc,
            });

    Some(CompletionResponse::Array(completions))
}

/// [`CompletionItem`] helper function.
///
/// Builds a [`CompletionItem`].
fn completion_item(color_name: String, color_hex: String) -> CompletionItem {
    CompletionItem {
        kind: Some(CompletionItemKind::COLOR),
        detail: Some(format!(
            "\"#{}\" ({}) color",
            color_hex,
            color_name.to_lowercase()
        )),
        sort_text: Some(color_name.to_lowercase()),
        insert_text: Some(color_hex),
        label: color_name,
        ..CompletionItem::default()
    }
}

/// Color searching helper function.
///
/// Searches for color matches in a `line` using `regex`, returns a [`ColorInformation`].
pub fn colors_in_line_iter(
    regex: &Regex,
    line_number: usize,
    line: &str,
) -> impl Iterator<Item = ColorInformation> {
    regex
        .captures_iter(line)
        .filter(Result::is_ok)
        .map(move |captures| {
            let captures = captures.expect("perfectly valid regex");
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
            let line_number = u32::try_from(line_number).unwrap();

            // parsing string slices of hex colors to the Color structure
            let color = parse_color(color_match.as_str()).unwrap();

            // start tells the LSP client that the color starts *here* on *this* line
            // end tells the client that the color ends *here* on *this* line
            ColorInformation {
                color,
                range: Range {
                    start: Position::new(line_number, color_start),
                    end: Position::new(line_number, color_end),
                },
            }
        })
}

/// Color parsing function.
///
/// It parses a string slice and returns a [`Color`].
///
/// # Exceptions
///
/// Returns a [`None`] when the input string length is not in [3, 4, 6, 8].
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
        3 | 4 => {
            let (r, g, b, a) = if str_length == 3 {
                ((hex & 0xF00) >> 8, (hex & 0x0F0) >> 4, hex & 0x00F, 0xF)
            } else {
                (
                    (hex & 0xF000) >> 12,
                    (hex & 0x0F00) >> 8,
                    (hex & 0x00F0) >> 4,
                    (hex & 0x000F),
                )
            };

            #[expect(
                clippy::cast_possible_truncation,
                reason = "r, g, b values are always less than 0x10 which is less than u8::MAX"
            )]
            Some(Color {
                red: f32::from(r as u8) / 15f32,
                green: f32::from(g as u8) / 15f32,
                blue: f32::from(b as u8) / 15f32,
                alpha: f32::from(a as u8) / 15f32,
            })
        }
        6 | 8 => {
            let (r, g, b, a) = if str_length == 6 {
                let [_, r, g, b] = hex.to_be_bytes();

                (r, g, b, 0xFF)
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

#[cfg(test)]
mod parse_color {
    use super::*;

    #[test]
    fn success() {
        let cases = [
            ("369", (0.2, 0.4, 0.6, 1.0)),
            ("369C", (0.2, 0.4, 0.6, 0.8)),
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
    fn fail_with_incorrect_length() {
        assert_eq!(None, parse_color("abcdefe"));
    }

    #[test]
    #[should_panic]
    fn panic_cant_parse_hex() {
        parse_color("arstgm");
    }

    #[test]
    #[should_panic]
    fn panic_str_too_long() {
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

#[cfg(test)]
mod colors_in_line_iter {
    use super::*;

    const REGEX: &str =
        r#"(["'])\#([0-9a-fA-F]{8}|[0-9a-fA-F]{6}|[0-9a-fA-F]{4}|[0-9a-fA-F]{3})\1"#;

    #[test]
    fn success() {
        let text = "color1 = \"#9AB8DE\" # some comment\ncolor2 = '#369'\ncolor3 = \"#336699CC\"\ncolor4 = \"#693C\"";

        let colors = [
            color_information((0, 9), (0, 18), (0.6039216, 0.72156864, 0.87058824, 1.0)),
            color_information((1, 9), (1, 15), (0.2, 0.4, 0.6, 1.0)),
            color_information((2, 9), (2, 20), (0.2, 0.4, 0.6, 0.8)),
            color_information((3, 9), (3, 16), (0.4, 0.6, 0.2, 0.8)),
        ];

        assert_eq!(colors, &find_colors(text)[..]);
    }

    #[test]
    fn no_colors() {
        let text = "some random text talking about colors like #ABCDEF, #FFF,\n#E03C31,\n#007AA5\n#0A45\nalso wrong strings like \"#FF0000'";

        let colors: [ColorInformation; 0] = [];

        assert_eq!(colors, &find_colors(text)[..]);
    }

    #[test]
    fn one_line_multiple_colors() {
        let text = "color1 = \"#9AB8DE\", '#369'";

        let colors = [
            color_information((0, 9), (0, 18), (0.6039216, 0.72156864, 0.87058824, 1.0)),
            color_information((0, 20), (0, 26), (0.2, 0.4, 0.6, 1.0)),
        ];

        assert_eq!(colors, &find_colors(text)[..]);
    }

    fn color_information(
        start: (u32, u32),
        end: (u32, u32),
        (red, green, blue, alpha): (f32, f32, f32, f32),
    ) -> ColorInformation {
        ColorInformation {
            range: Range {
                start: Position {
                    line: start.0,
                    character: start.1,
                },
                end: Position {
                    line: end.0,
                    character: end.1,
                },
            },
            color: Color {
                red,
                green,
                blue,
                alpha,
            },
        }
    }

    fn find_colors(text: &str) -> Vec<ColorInformation> {
        let regex = Regex::new(REGEX).unwrap();

        text.lines()
            .enumerate()
            .flat_map(|(line_num, line)| colors_in_line_iter(&regex, line_num, &line))
            .collect()
    }
}

#[cfg(test)]
mod color_completions {
    use super::*;

    #[test]
    fn named_color_completions_noop() {
        assert_eq!(None, named_colors_completions(&CompletionsMode::None));
    }

    #[test]
    fn named_color_completions_uppercase() {
        if let Some(CompletionResponse::Array(completion_items_vec)) =
            named_colors_completions(&CompletionsMode::Uppercase)
        {
            assert_eq!(false, completion_items_vec.is_empty());
        }
    }

    #[test]
    fn named_color_completions_lowercase() {
        if let Some(CompletionResponse::Array(completion_items_vec)) =
            named_colors_completions(&CompletionsMode::Lowercase)
        {
            assert_eq!(false, completion_items_vec.is_empty());
        }
    }

    #[test]
    fn named_color_completions_full() {
        if let Some(CompletionResponse::Array(completion_items_vec)) =
            named_colors_completions(&CompletionsMode::Full)
        {
            assert_eq!(false, completion_items_vec.is_empty());
        }
    }
}
