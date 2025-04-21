rec {
  description =
    "Simulating Multi-agent Path Planning in Complex environments using Gaussian Belief Propagation and Global Path Finding";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    let
      cargoToml = with builtins; fromTOML (readFile ./Cargo.toml);
      inherit (cargoToml.workspace.package) version;
      name = "magics";
    in flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };
        rustToolchain =
          pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
        inherit (pkgs) lib;

        cargo-subcommands = with pkgs; [
          cargo-watch
          cargo-flamegraph
          cargo-release
        ];
        dev-deps = with pkgs; [
          taplo # TOML formatter and LSP
          bacon
          just
          typos # spell checker
        ];
      in {
        apps.default = {
          type = "app";
          program = pkgs.lib.getExe self.packages.${system}.default;
          # description = "foo";
        };

        # checks.default = self.packages.${system}.default.override { cargoBuildType = "debug"; };

        devShells.default = pkgs.mkShell rec {
          inherit (self.packages.${system}.default)
            nativeBuildInputs buildInputs;
          packages = cargo-subcommands ++ dev-deps;

          LD_LIBRARY_PATH =
            lib.makeLibraryPath (buildInputs ++ nativeBuildInputs);
        };

        packages.default = pkgs.rustPlatform.buildRustPackage {
          pname = name;
          inherit version;

          nativeBuildInputs = with pkgs;
            [
              pkg-config
              openssl
              zlib
              mold # A Modern Linker
              clang # For linking
              libxkbcommon
              rustToolchain
            ] ++ (lib.optionals pkgs.stdenv.isLinux
              (with pkgs; [ wayland egl-wayland alsa-lib ]));
          buildInputs = with pkgs;
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
            ] ++ lib.optionals pkgs.stdenv.isLinux (with pkgs; [
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
              (lib.getLib vulkan-loader)
              # (lib.getLib egl-wayland)
            ]);

          src = ./.;

          rustPlatform = rustToolchain;

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
        overlays.default = { ${name} = self.packages.${system}.default; };
      });
}
