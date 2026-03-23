{
  rustPlatform,
  clippy,
  lib,
}:
rustPlatform.buildRustPackage {
  name = "uwu_colors";

  nativeBuildInputs = [
    clippy
  ];

  cargoLock.lockFile = ./Cargo.lock;
  src = lib.cleanSource ./.;

  meta = {
    description = "dead simple language server to colorize hex color strings via textDocument/documentColor";
    homepage = "https://codeberg.org/q60/uwu_colors";
    license = lib.licenses.unlicense;
    mainProgram = "uwu_colors";
  };
}
