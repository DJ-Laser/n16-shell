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
      shellHook = ''
        export RUST_SRC_PATH =${rustToolchain}/lib/rustlib/src/rust/src
        export LD_LIBRARY_PATH=${lib.makeLibraryPath icedDeps}:$LD_LIBRARY_PATH
      '';
    };

    packages.${system}.n16-shell = pkgs.rustPlatform.buildRustPackage rec {
      pname = "n16-shell";
      version = "0.1.0";
      src = ./.;

      cargoLock = {
        lockFile = ./Cargo.lock;
      };

      buildInputs = icedDeps;
      postFixup = ''
        patchelf --add-rpath ${lib.makeLibraryPath icedDeps} $out/bin/${pname}
      '';
    };

    defaultPackage.${system} = self.packages.${system}.n16-shell;

    overlays.default = final: prev: {
      n16-shell = self.packages.${system}.n16-shell;
    };
  };
}
