{
  system ? builtins.currentSystem,
  nixpkgs ? <nixpkgs>,
  lib ? import "${nixpkgs}/lib",
  pkgs ? import nixpkgs {inherit system;},
  ...
}:
lib.fix (self: {
  inherit system nixpkgs;

  packages = {
    default = self.packages.uwu_colors;
    uwu_colors = pkgs.callPackage ./package.nix {};
  };
})
