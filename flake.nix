{
  description = "Simulating Multi-agent Path Planning in Complex environments using Gaussian Belief Propagation and Global Path Finding";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs =
    {
      self,
      nixpkgs,
      rust-overlay,
      flake-utils,
      ...
    }@inputs:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

        bevy-deps = with pkgs; [
          udev
          alsa-lib
          vulkan-loader
          xorg.libX11
          xorg.libXcursor
          xorg.libXi
          xorg.libXrandr
          libxkbcommon
          wayland
          egl-wayland
          freetype
          fontconfig
        ];
        cargo-subcommands = with pkgs; [
          cargo-bloat
          cargo-expand
          cargo-outdated
          cargo-show-asm
          cargo-make
          cargo-modules
          cargo-nextest
          cargo-rr
          cargo-udeps
          cargo-watch
          cargo-wizard
          cargo-pgo
          cargo-flamegraph
          cargo-license
          cargo-release
          # cargo-tree

          #   # cargo-profiler
          #   # cargo-feature
        ];
        rust-deps =
          with pkgs;
          [
            # rustup
            taplo # TOML formatter and LSP
            bacon
            mold # A Modern Linker
            clang # For linking
            gdb # debugger
            # lldb # debugger
            # rr # time-traveling debugger
            ra-multiplex
            graphviz
            blas
            # openblas
            openssl
            # lapack
            gcc
            gfortran
            zlib
          ]
          ++ cargo-subcommands;
        dev-deps = with pkgs; [
          just
          typos # spell checker
        ];
      in
      with pkgs;
      {
        formatter.${system} = pkgs.alejandra;
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = [ pkgs.pkg-config ];
          buildInputs =
            [
              rustToolchain
            ]
            ++ bevy-deps
            ++ rust-deps
            ++ dev-deps;

          LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;
        };
      }
    );
}
