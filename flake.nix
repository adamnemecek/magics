rec {
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
    let
      cargoToml = with builtins; fromTOML (readFile ./Cargo.toml);
      inherit (cargoToml.workspace.package) version;
      name = "magics";
    in
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import inputs.nixpkgs { inherit system overlays; };
        rustToolchain = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        inherit (pkgs) lib;

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
        dev-deps = with pkgs; [
          taplo # TOML formatter and LSP
          bacon
          just
          typos # spell checker
        ];
      in
      {
        apps.default = {
          type = "app";
          program = pkgs.lib.getExe self.packages.${system}.default;
        };

        devShells.default = pkgs.mkShell rec {
          inherit (self.packages.${system}.default) nativeBuildInputs;
          buildInputs = self.packages.${system}.default.buildInputs ++ [
            rustToolchain
          ];
          packages = cargo-subcommands ++ dev-deps;

          LD_LIBRARY_PATH = lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
        };

        formatter.${system} = pkgs.nixpkgs-rfc-style;

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = name;
          inherit version;

          nativeBuildInputs = with pkgs; [
            pkg-config
            alsa-lib
            openssl
            zlib
            mold # A Modern Linker
            clang # For linking
            wayland
            egl-wayland
            libxkbcommon
          ];
          buildInputs =
            with pkgs;
            [
              freetype
              fontconfig
              vulkan-loader
              blas
              # openblas
              # openssl
              # lapack
              gcc
              gfortran
              graphviz
              libxkbcommon
            ]
            ++ lib.optionals pkgs.stdenv.isLinux (
              with pkgs;
              [
                (lib.getLib alsa-lib)
                xorg.libX11
                xorg.libXcursor
                xorg.libXi
                xorg.libXrandr
                # wayland
                (lib.getLib wayland)
                (lib.getLib udev)
                (lib.getLib libxkbcommon)
                udev
                egl-wayland
              ]
            );

          src = ./.;

          cargoLock = {
            lockFile = ./Cargo.lock;
            allowBuiltinFetchGit = true;
          };

          meta = {
            inherit description;
            homepage = cargoToml.workspace.package.repository;
            license = lib.licenses.mit;
            mainProgram = name;
          };
        };

        # overlays.default = final: prev: { ${name} = self.packages.${system}.default; };
        overlays.default = {
          ${name} = self.packages.${system}.default;
        };
      }
    );
}
