{
  description = "uwu_colors";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
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
  }: let
    lib = nixpkgs.lib;
    default-nix = system:
      import ./default.nix {
        inherit nixpkgs lib system;
      };
  in
    {
      overlays.default = final: prev: {
        uwu-colors = self.packages.${final.stdenv.hostPlatform.system}.default;
      };
    }
    // lib.mapAttrs (_: lib.genAttrs ["x86_64-linux" "aarch64-linux" "aarch64-darwin"]) {
      packages = system: (default-nix system).packages;
      apps = system: {
        default = {
          type = "app";
          program = lib.getExe self.packages.${system}.default;
        };
      };
    };
}
