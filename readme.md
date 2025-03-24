# uwu-colors

a dead simple language server that i made for helix because of https://github.com/helix-editor/helix/pull/12308

![first screenshot demonstrating usage in a TOML file inside helix](https://i.imgur.com/SCBt8I7.png)

![second screenshot demonstrating usage in a nix file inside helix](https://i.imgur.com/SOfDmr2.png)

currently, it only supports hex color strings like `"#ABC"`, `"#AABBCC"`, `"#AABBCCDD"`. alpha channel doesn't seem to be supported by helix tho

you can use it as a flake

1. add it to your system's inputs
2. overlay it as `inputs.uwu-colors.packages.${pkgs.system}.default`
3. add it to helix language server configuration with command `"${pkgs.uwu-colors}/bin/uwu_colors"`
4. add it to your languages of needs
