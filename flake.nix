{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    fenix = {
      url = "github:nix-community/fenix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    nixpkgs,
    fenix,
  } @ inputs: let
    system = "x86_64-linux";
    overlays = [(fenix.overlays.default)];
    pkgs = import nixpkgs {inherit system overlays;};
    lib = pkgs.lib;

    rustToolchain = pkgs.fenix.stable.toolchain;

    icedDeps = with pkgs; [
      expat
      fontconfig
      freetype
      freetype.dev
      libGL
      pkg-config
      libx11
      libxcursor
      libxi
      libxrandr
      wayland
      libxkbcommon
      openssl
    ];
  in {
    devShells.${system}.default = pkgs.mkShell {
      buildInputs = [rustToolchain] ++ icedDeps;

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
