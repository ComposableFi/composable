{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , cargoTools, ... }:
    let
      rustSrc = cargoTools.mkRustSrc ./.;
      # https://github.com/paritytech/polkadot-sdk/issues/1755  
      rust = (self.inputs.crane.mkLib pkgs).overrideToolchain
        (pkgs.rust-bin.nightly."2023-03-09".default.override {
          targets = [ "wasm32-unknown-unknown" ];
        });
      # Build a wasm runtime, unoptimized
      mkRuntime = name: features: cargoArtifacts:
        rust.buildPackage (systemCommonRust.common-attrs // {
          pname = "${name}-runtime";
          src = rustSrc;
          inherit cargoArtifacts;
          cargoBuildCommand =
            "cargo build --release --package ${name}-runtime-wasm --target wasm32-unknown-unknown"
            + pkgs.lib.strings.optionalString (features != "")
            (" --features=${features}");
          # From parity-tech/polkdot-sdk/wasm-builder
          RUSTFLAGS =
            "-C target-cpu=mvp -C target-feature=-sign-ext -C link-arg=--export-table -Clink-arg=--export=__heap_base -C link-arg=--import-memory";
        });

      # Derive an optimized wasm runtime from a prebuilt one, garbage collection + compression
      mkOptimizedRuntime = { name, features ? "", tag ? "prod"
        , common-deps ? self'.packages.common-deps-nightly }:
        let runtime = mkRuntime name features common-deps;
        in pkgs.stdenv.mkDerivation {
          name = "${runtime.name}-${tag}-optimized";
          phases = [ "installPhase" ];
          nativeBuildInputs = [ pkgs.binaryen ];
          installPhase = ''
            mkdir --parents $out/lib
            # https://github.com/paritytech/substrate/blob/30cb4d10b3118d1b3aa5b2ae7fa8429b2c4f28de/utils/wasm-builder/src/wasm_project.rs#L694
            wasm-opt ${runtime}/lib/${name}_runtime.wasm -o $out/lib/runtime.optimized.wasm -Os --strip-dwarf --debuginfo --mvp-features            
            ${pkgs.subwasm}/bin/subwasm compress $out/lib/runtime.optimized.wasm $out/lib/runtime.optimized.wasm
          '';
        };

    in {
      # Add the npm-buildpackage overlay to the perSystem's pkgs
      packages = rec {
        picasso-runtime = mkOptimizedRuntime {
          name = "picasso";
          features = "";
        };

        picasso-runtime-dev = pkgs.stdenv.mkDerivation ({
          name = "picasso-runtime-dev";
          dontUnpack = true;
          buildInputs = with self'.packages; with pkgs; [ subwasm subxt ];
          patchPhase = "";
          dontStrip = true;
          installPhase = ''
            mkdir --parents $out/lib
            mkdir --parents $out/docs
            mkdir --parents $out/include
            subwasm metadata ${picasso-runtime}/lib/runtime.optimized.wasm --format json > $out/lib/picasso-runtime.json
            subwasm metadata ${picasso-runtime}/lib/runtime.optimized.wasm --format scale > $out/lib/picasso-runtime.scale
            subwasm metadata ${picasso-runtime}/lib/runtime.optimized.wasm --format human > $out/docs/picasso-runtime.txt
            subxt codegen --file $out/lib/picasso-runtime.scale > $out/include/picasso_runtime.rs
          '';
        });

        composable-runtime-dev = pkgs.stdenv.mkDerivation ({
          name = "composable-runtime-dev";
          dontUnpack = true;
          buildInputs = with self'.packages; with pkgs; [ subwasm subxt ];
          patchPhase = "";
          dontStrip = true;
          installPhase = ''
            mkdir --parents $out/lib
            mkdir --parents $out/docs
            mkdir --parents $out/include
            subwasm metadata ${composable-runtime}/lib/runtime.optimized.wasm --format json > $out/lib/composable-runtime.json
            subwasm metadata ${composable-runtime}/lib/runtime.optimized.wasm --format scale > $out/lib/composable-runtime.scale
            subwasm metadata ${composable-runtime}/lib/runtime.optimized.wasm --format human > $out/docs/composable-runtime.txt
            subxt codegen --file $out/lib/composable-runtime.scale > $out/include/picasso_runtime.rs
          '';
        });

        picasso-testfast-runtime = mkOptimizedRuntime {
          name = "picasso";
          features = "testnet,fastnet";
        };
        composable-testfast-runtime = mkOptimizedRuntime {
          name = "composable";
          features = "testnet,fastnet";
        };
        composable-runtime = mkOptimizedRuntime {
          name = "composable";
          features = "";
        };
        # asd
        picasso-bench-runtime = mkOptimizedRuntime {
          name = "picasso";
          tag = "bench";
          features = "runtime-benchmarks";
          common-deps = self'.packages.common-wasm-bench-deps;
        };
        composable-bench-runtime = mkOptimizedRuntime {
          tag = "bench";
          name = "composable";
          features = "runtime-benchmarks";
          common-deps = self'.packages.common-wasm-bench-deps;
        };
      };

    };
}
