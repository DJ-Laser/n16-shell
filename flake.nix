{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    flake-utils,
    rust-overlay,
  } @ inputs:
    flake-utils.lib.eachDefaultSystem
    (
      system: let
        overlays = [(import rust-overlay)];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
          extensions = ["rust-src"];
        };
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [rustToolchain alejandra wasm-pack node2nix nodejs];
          shellHook = ''
            export RUST_SRC_PATH =${rustToolchain}/lib/rustlib/src/rust/src
          '';
        };
      }
    );
}