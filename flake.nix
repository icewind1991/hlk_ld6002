{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
    rust-overlay.inputs.flake-utils.follows = "flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        overlays = [
          (import rust-overlay)
        ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };
        toolchain = pkgs.rust-bin.stable.latest.default;
        dependencies = with pkgs; [pkg-config udev];
        tools = with pkgs; [toolchain bacon cargo-edit cargo-outdated cargo-audit];
      in rec {
        devShell = pkgs.mkShell {
          nativeBuildInputs = dependencies ++ tools;
        };
      }
    );
}
