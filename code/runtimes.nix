{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      rustSrc = pkgs.lib.cleanSourceWith {
        filter = pkgs.lib.cleanSourceFilter;
        src = pkgs.lib.cleanSourceWith {
          filter = let
            isProto = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".proto" name;
            isJSON = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".json" name;
            isREADME = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix "README.md" name;
            isDir = name: type: type == "directory";
            isCargo = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".toml" name
              || type == "regular" && pkgs.lib.strings.hasSuffix ".lock" name;
            isRust = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".rs" name;
            customFilter = name: type:
              builtins.any (fun: fun name type) [
                isCargo
                isRust
                isDir
                isREADME
                isJSON
                isProto
              ];
          in pkgs.nix-gitignore.gitignoreFilterPure customFilter
          [ ../.gitignore ] ./.;
          src = ./.;
        };
      };
      # Build a wasm runtime, unoptimized
      mkRuntime = name: features:
        crane.nightly.buildPackage (systemCommonRust.common-attrs // {
          pname = "${name}-runtime";
          src = rustSrc;

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
          nativeBuildInputs = [ pkgs.binaryen ];
          installPhase = ''
            mkdir --parents $out/lib
            # https://github.com/paritytech/substrate/blob/30cb4d10b3118d1b3aa5b2ae7fa8429b2c4f28de/utils/wasm-builder/src/wasm_project.rs#L694
            wasm-opt ${runtime}/lib/${name}_runtime.wasm -o $out/lib/runtime.optimized.wasm -Os --strip-dwarf --debuginfo --mvp-features            
            ${self'.packages.subwasm}/bin/subwasm compress $out/lib/runtime.optimized.wasm $out/lib/runtime.optimized.wasm
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
        rococo-wasm-runtime = pkgs.stdenv.mkDerivation {
          name = "rococo-wasm-runtime";
          dontUnpack = true;
          src = pkgs.fetchurl {
            url =
              "https://github.com/paritytech/polkadot/releases/download/v0.9.33/rococo_runtime-v9330.compact.compressed.wasm";
            hash = "sha256-lvjPqQVEdu/5EeZE2NMAROO2ypCeV6QENFHhNYf9SCI=";
          };
          installPhase = ''
            mkdir -p $out/lib
            cp $src $out/lib/rococo_runtime.compact.compressed.wasm
          '';
        };
      };

    };
}
