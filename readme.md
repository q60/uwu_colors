# uwu-colors

[![Crates.io Version](https://img.shields.io/crates/v/uwu_colors?style=for-the-badge)](https://crates.io/crates/uwu_colors)

a simple language server that i made mainly for helix because of https://github.com/helix-editor/helix/pull/12308

![first screenshot demonstrating usage in a nix file inside helix](https://i.imgur.com/h1R35Gq.png)

![second screenshot demonstrating variable completions](https://i.imgur.com/xfUVAVB.png)

![third screenshot demonstrating help message](https://i.imgur.com/lrPDcDR.png)


## features

`uwu-colors` sends `textDocument/documentColor` request on hex color strings like `"#ABC"`, `"#abcd"`, `'#AaBbCc'`, `'#AABBCCDD'`

it also provides various completions


### completions

named color completions are enabled by default - uppercase hex colors from https://www.colorhexa.com/color-names

variable completions can be enabled using a flag


#### options and flags

`--named-completions-mode`:
- `upper` completes with uppercase hex strings
- `lower` - with lowercase strings
- `full` - both with lowercase strings and uppercase strings using uppercase names
- `none` disables completions

`--color-collection`:
- `colorhexa` - named colors from ColorHexa
- `css` - named CSS colors

`--variable-completions` - enables variable completions like on the second screenshot


## installation

### you can use it as a flake

1. add it to your system's inputs
2. overlay it
  - as `inputs.uwu-colors.overlays.default`
  - *or* using packages overlay as `inputs.uwu-colors.packages.${pkgs.system}.default`
3. add it to helix language server configuration with command `"${pkgs.uwu-colors}/bin/uwu_colors"`
4. add it to your languages of needs


### from crates.io

preferred way on a system w/o nix - `cargo install uwu_colors`


### binary from releases

grab an x86_64 binary from github [releases](https://github.com/q60/uwu_colors/releases)


### packaging status

[![repology packaging status](https://repology.org/badge/vertical-allrepos/uwu-colors.svg)](https://repology.org/project/uwu-colors/versions)
