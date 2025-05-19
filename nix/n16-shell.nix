{
  lib,
  fetchFromGitHub,
  rustPlatform,
  expat,
  fontconfig,
  freetype,
  libGL,
  pkg-config,
  xorg,
  wayland,
  libxkbcommon,
}: let
  icedDeps = [
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
in
  rustPlatform.buildRustPackage rec {
    pname = "n16-shell";
    version = "0.1.1";
    src = ../.;

    cargoLock = {
      lockFile = ../Cargo.lock;
    };

    buildInputs = icedDeps;

    preBuild = ''
      export N16_COMPLETION_OUT_DIR=$out/share/bash-completion/completions
      mkdir -p $N16_COMPLETION_OUT_DIR
    '';

    postFixup = ''
      patchelf --add-rpath ${lib.makeLibraryPath icedDeps} $out/bin/n16-daemon
    '';

    meta = {
      mainProgram = "n16";
      platforms = lib.platforms.linux;
    };
  }
