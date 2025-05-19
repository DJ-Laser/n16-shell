{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    rust-overlay.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = {
    self,
    nixpkgs,
    rust-overlay,
  } @ inputs: let
    system = "x86_64-linux";
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
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = with pkgs; [rustToolchain alejandra] ++ icedDeps;

      RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/src";
      LD_LIBRARY_PATH = lib.makeLibraryPath icedDeps;
    };

    packages.${system}.n16-shell = pkgs.callPackage ./nix/n16-shell.nix {};

    defaultPackage.${system} = self.packages.${system}.n16-shell;

    overlays.default = final: prev: {
      n16-shell = self.packages.${system}.n16-shell;
    };
  };
}
