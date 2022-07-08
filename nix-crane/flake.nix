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
      in with pkgs;
      let
        rust-stable = rust-bin.stable.latest.default;
        rust-nightly = rust-bin.selectLatestNightlyWith (toolchain:
          toolchain.default.override {
            extensions = [ "rust-src" ];
            targets = [ "wasm32-unknown-unknown" ];
          });
        crane-lib = crane.mkLib pkgs;
        # Crane pinned to stable Rust
        crane-stable = crane-lib.overrideToolchain rust-stable;
        # Crane pinned to nightly Rust
        crane-nightly = crane-lib.overrideToolchain rust-nightly;
        # Wasm optimizer, used to replicate build.rs behavior in an explicit fashion
        wasm-optimizer = crane-stable.buildPackage {
          cargoCheckCommand = "true";
          src = let customFilter = name: type: true;
          in lib.cleanSourceWith {
            filter = lib.cleanSourceFilter;
            src = lib.cleanSourceWith {
              filter =
                nix-gitignore.gitignoreFilterPure customFilter [ ../.gitignore ]
                ../utils/wasm-optimizer;
              src = ../utils/wasm-optimizer;
            };
          };
        };
        # Common env required to build the node
        common-args = {
          cargoCheckCommand = "true";
          doCheck = false;
          src = let
            customFilter = name: type:
              !(type == "directory" && (baseNameOf name == "nix"
                || baseNameOf name == "nix-crane"));
          in lib.cleanSourceWith {
            filter = lib.cleanSourceFilter;
            src = lib.cleanSourceWith {
              filter =
                nix-gitignore.gitignoreFilterPure customFilter [ ../.gitignore ]
                ../.;
              src = ../.;
            };
          };
          buildInputs = [ openssl zstd ];
          nativeBuildInputs = [ clang pkg-config ]
            ++ lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
              Security
              SystemConfiguration
            ]);
          # Don't build any wasm as we do it ourselves
          SKIP_WASM_BUILD = "1";
          LD_LIBRARY_PATH = lib.strings.makeLibraryPath [
            stdenv.cc.cc.lib
            llvmPackages.libclang.lib
          ];
          LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
          PROTOC = "${protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${rocksdb}/lib";
        };
        common-deps = crane-stable.buildDepsOnly (common-args // { });
        # Build a wasm runtime, unoptimized
        mk-runtime = name:
          let file-name = "${name}_runtime.wasm";
          in crane-nightly.buildPackage (common-args // {
            pname = "${name}-runtime";
            cargoBuildCommand =
              "cargo build --release -p ${name}-runtime-wasm --target wasm32-unknown-unknown";
            RUSTFLAGS =
              "-Clink-arg=--export=__heap_base -Clink-arg=--import-memory";
          });
        # Optimize a pre-built wasm runtime
        mk-optimized-runtime = name:
          let runtime = mk-runtime name;
          in stdenv.mkDerivation {
            name = "${runtime.name}-optimized";
            phases = [ "installPhase" ];
            installPhase = ''
              mkdir -p $out/lib
              ${wasm-optimizer}/bin/wasm-optimizer \
                --input ${runtime}/lib/${name}_runtime.wasm \
                --output $out/lib/runtime.optimized.wasm
            '';
          };
        mk-package = name:
          crane-stable.buildPackage (common-args // {
            pname = name;
            cargoArtifacts = common-deps;
            cargoBuildCommand = "cargo build --release -p ${name}";
          });
        mk-node = runtimes:
          crane-stable.buildPackage (common-args // {
            pname = "composable-node";
            cargoArtifacts = common-deps;
            cargoBuildCommand = "cargo build --release -p composable";
            DALI_RUNTIME = "${runtimes.dali}/lib/runtime.optimized.wasm";
            PICASSO_RUNTIME = "${runtimes.picasso}/lib/runtime.optimized.wasm";
            COMPOSABLE_RUNTIME =
              "${runtimes.composable}/lib/runtime.optimized.wasm";
          });
      in rec {
        packages.wasm-optimizer = wasm-optimizer;
        packages.runtimes = {
          dali = mk-optimized-runtime "dali";
          picasso = mk-optimized-runtime "picasso";
          composable = mk-optimized-runtime "composable";
        };
        packages.price-feed = mk-package "price-feed";
        packages.composable-node = mk-node packages.runtimes;
        packages.default = packages.composable-node;
        devShell = mkShell {
          buildInputs = with packages; [
            rust-stable
            wasm-optimizer
            composable-node
          ];
        };
      });
}
