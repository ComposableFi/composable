{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      # Build a wasm runtime, unoptimized
      mkRuntime = name: features:
        crane.nightly.buildPackage (systemCommonRust.common-attrs // {
          pname = "${name}-runtime";
          cargoArtifacts = self'.packages.common-deps-nightly;
          cargoBuildCommand =
            "cargo build --release -p ${name}-runtime-wasm --target wasm32-unknown-unknown"
            + pkgs.lib.strings.optionalString (features != "")
            (" --features=${features}");
          # From parity/wasm-builder
          RUSTFLAGS =
            "-Clink-arg=--export=__heap_base -Clink-arg=--import-memory";
        });

      # Derive an optimized wasm runtime from a prebuilt one, garbage collection + compression
      mkOptimizedRuntime = { name, features ? "" }:
        let runtime = mkRuntime name features;
        in pkgs.stdenv.mkDerivation {
          name = "${runtime.name}-optimized";
          phases = [ "installPhase" ];
          installPhase = ''
            mkdir -p $out/lib
            ${self'.packages.wasm-optimizer}/bin/wasm-optimizer \
            --input ${runtime}/lib/${name}_runtime.wasm \
            --output $out/lib/runtime.optimized.wasm
          '';
        };

    in {
      # Add the npm-buildpackage overlay to the perSystem's pkgs
      packages = rec {
        dali-runtime = mkOptimizedRuntime {
          name = "dali";
          features = "";
        };
        picasso-runtime = mkOptimizedRuntime {
          name = "picasso";
          features = "";
        };
        composable-runtime = mkOptimizedRuntime {
          name = "composable";
          features = "";
        };
        dali-bench-runtime = mkOptimizedRuntime {
          name = "dali";
          features = "runtime-benchmarks";
        };
        picasso-bench-runtime = mkOptimizedRuntime {
          name = "picasso";
          features = "runtime-benchmarks";
        };
        composable-bench-runtime = mkOptimizedRuntime {
          name = "composable";
          features = "runtime-benchmarks";
        };

      };

    };
}
