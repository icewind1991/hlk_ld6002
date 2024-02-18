{
  inputs = {
    nixpkgs.url = "nixpkgs/nixos-23.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = (import nixpkgs) {
          inherit system;
        };
        dependencies = with pkgs; [pkg-config udev];
        tools = with pkgs; [rustc cargo bacon cargo-edit cargo-outdated clippy cargo-audit];
      in rec {
        devShell = pkgs.mkShell {
          nativeBuildInputs = dependencies ++ tools;
        };
      }
    );
}
