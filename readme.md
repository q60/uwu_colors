# uwu-colors

a simple language server that i made mainly for helix because of https://github.com/helix-editor/helix/pull/12308

![first screenshot demonstrating usage in a nix file inside helix](https://i.imgur.com/h1R35Gq.png)

![second screenshot demonstrating help message](https://i.imgur.com/6nM046j.png)


## features

`uwu-colors` sends `textDocument/documentColor` request on hex color strings like `"#ABC"`, `"#abcd"`, `'#AaBbCc'`, `'#AABBCCDD'`.

it also provides named color completions.


### completions

named color completions are enabled by default - uppercase hex colors from https://www.colorhexa.com/color-names


#### options

`--completions-mode`:
- `upper` completes with uppercase hex strings
- `lower` - with lowercase strings
- `full` - both with lowercase strings and uppercase strings using uppercase names
- `none` disables completions

`--color-collection`:
- `colorhexa` - named colors from ColorHexa
- `css` - named CSS colors


## installation

you can use it as a flake

1. add it to your system's inputs
2. overlay it as `inputs.uwu-colors.packages.${pkgs.system}.default`
3. add it to helix language server configuration with command `"${pkgs.uwu-colors}/bin/uwu_colors"`
4. add it to your languages of needs
