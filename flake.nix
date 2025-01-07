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
        pkgs = import nixpkgs {inherit system overlays;};
        lib = pkgs.lib;

        rustToolchain = pkgs.pkgsBuildHost.rust-bin.stable.latest.default.override {
          extensions = ["rust-src"];
        };

        icedDeps = with pkgs; [
          expat
          fontconfig
          freetype
          freetype.dev
          libGL
          pkg-config
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          wayland
          libxkbcommon
        ];
      in {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [rustToolchain alejandra] ++ icedDeps;
          shellHook = ''
            export RUST_SRC_PATH =${rustToolchain}/lib/rustlib/src/rust/src
            export LD_LIBRARY_PATH=${lib.makeLibraryPath icedDeps}:$LD_LIBRARY_PATH
          '';
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = "icylauncher";
          version = "0.1.0";

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
          };
        };
      }
    );
}
