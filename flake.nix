{
  description = "uwu_colors";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    utils.url = "github:numtide/flake-utils";
  };

  nixConfig = {
    extra-substituters = [
      "https://kira.cachix.org/"
    ];

    extra-trusted-public-keys = [
      "kira.cachix.org-1:THBrq/BplPxOJnWnxCBMOeP03ReON+FUYZpiDTnZqwA="
    ];
  };

  outputs = {
    self,
    nixpkgs,
    utils,
  }:
    {
      overlays.default = final: prev: {
        uwu-colors = self.packages.${final.system}.default;
      };
    }
    // utils.lib.eachDefaultSystem
    (system: let
      pkgs = import nixpkgs {inherit system;};
    in {
      packages = {
        default = pkgs.rustPlatform.buildRustPackage {
          name = "uwu_colors";

          nativeBuildInputs = [
            pkgs.clippy
          ];

          cargoLock.lockFile = ./Cargo.lock;
          src = pkgs.lib.cleanSource ./.;
        };
      };

      apps.default = utils.lib.mkApp {drv = self.packages.${system}.default;};
    });
}
