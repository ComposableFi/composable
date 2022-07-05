{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane.url = "github:ipetkov/crane";
    crane.inputs.nixpkgs.follows = "nixpkgs";
  };

  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
        rust-nightly = pkgs.rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          });
        rust = pkgs.pkgsBuildHost.rust-bin.stable.latest.default;
        craneLib = (crane.mkLib pkgs).overrideToolchain rust;
      in with pkgs; {
        packages.default = craneLib.buildPackage {
          src = let
            customFilter = name: type:
              !(type == "directory" && (baseNameOf name == "nix"
                || baseNameOf name == "nix-crate"));
          in lib.cleanSourceWith {
            filter = lib.cleanSourceFilter;
            src = lib.cleanSourceWith {
              filter =
                nix-gitignore.gitignoreFilterPure customFilter [ ../.gitignore ]
                ../.;
              src = ../.;
            };
          };
          buildInputs = [ openssl ];
          nativeBuildInputs = [ rust rust-nightly clang pkg-config ]
            ++ lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
              Security
              SystemConfiguration
            ]);
          cargoExtraArgs = "-p composable-node";
          # Required as we are sandboxed, forward a --offline flag to cargo operations.
          CARGO_NET_OFFLINE = "1";
          # Avoid building those runtimes.
          SKIP_POLKADOT_RUNTIME_WASM_BUILD = "1";
          SKIP_KUSAMA_RUNTIME_WASM_BUILD = "1";
          SKIP_ROCOCO_RUNTIME_WASM_BUILD = "1";
          SKIP_STATEMINE_RUNTIME_WASM_BUILD = "1";
          LD_LIBRARY_PATH = lib.strings.makeLibraryPath [
            stdenv.cc.cc.lib
            llvmPackages.libclang.lib
          ];
          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
          PROTOC = "${protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        };
      });
}
