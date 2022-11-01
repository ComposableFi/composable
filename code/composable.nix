{ self, ... }:
{
  perSystem = { config, self', inputs', pkgs, system, systemLib, ... }:
    let
      rustSrc =
        let
          directoryBlacklist = [ "runtime-tests" ];
          fileBlacklist = [
            # does not makes sense to black list,
            # if we changed some version of tooling(seldom), we want to rebuild
            # so if we changed version of tooling, nix itself will detect invalidation and rebuild
            # "flake.lock"
          ];
        in
        pkgs.lib.cleanSourceWith {
          filter = pkgs.lib.cleanSourceFilter;
          src = pkgs.lib.cleanSourceWith {
            filter =
              let
                isBlacklisted = name: type:
                  let
                    blacklist =
                      if type == "directory" then
                        directoryBlacklist
                      else if type == "regular" then
                        fileBlacklist
                      else
                        [ ]; # symlink, unknown
                  in
                  builtins.elem (baseNameOf name) blacklist;
                isImageFile = name: type:
                  type == "regular" && pkgs.lib.strings.hasSuffix ".png" name;
                isPlantUmlFile = name: type:
                  type == "regular"
                  && pkgs.lib.strings.hasSuffix ".plantuml" name;
                isNixFile = name: type:
                  type == "regular" && pkgs.lib.strings.hasSuffix ".nix" name;
                customFilter = name: type:
                  !((isBlacklisted name type) || (isImageFile name type)
                    || (isPlantUmlFile name type)
                    # assumption that nix is final builder,
                    # so there would no be sandwich like  .*.nix <- build.rs <- *.nix
                    # and if *.nix changed, nix itself will detect only relevant cache invalidations
                    || (isNixFile name type));
              in
              pkgs.nix-gitignore.gitignoreFilterPure customFilter
                [ ../.gitignore ] ./.;
            src = ./.;
          };
        };

      common-deps = systemLib.crane-nightly.buildDepsOnly (common-attrs // { });
      common-deps-nightly = systemLib.crane-nightly.buildDepsOnly (common-attrs // { });
      common-bench-attrs = common-attrs // { cargoExtraArgs = "--features=builtin-wasm,runtime-benchmarks"; };
      common-bench-deps = systemLib.crane-nightly.buildDepsOnly (common-bench-attrs // { });


      substrate-attrs = {
        LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
          pkgs.stdenv.cc.cc.lib
          pkgs.llvmPackages.libclang.lib
        ];
        LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
        PROTOC = "${pkgs.protobuf}/bin/protoc";
        ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
      };

      wasm-optimizer = systemLib.crane-stable.buildPackage (common-attrs // {
        cargoCheckCommand = "true";
        pname = "wasm-optimizer";
        cargoArtifacts = common-deps;
        cargoBuildCommand =
          "cargo build --release --package wasm-optimizer";
        version = "0.1.0";
        # NOTE: we copy more then needed, but tht is simpler to setup, we depend on substrate for sure so
      });


      # Common env required to build the node
      common-attrs = substrate-attrs // {
        src = rustSrc;
        buildInputs = with pkgs; [ openssl zstd ];
        nativeBuildInputs = with pkgs;
          [ clang openssl pkg-config ] ++ pkgs.lib.optional stdenv.isDarwin
            (with darwin.apple_sdk.frameworks; [
              Security
              SystemConfiguration
            ]);
        doCheck = false;
        cargoCheckCommand = "true";
        # Don't build any wasm as we do it ourselves
        SKIP_WASM_BUILD = "1";
      };

      # Build a wasm runtime, unoptimized
      mkRuntime = name: features:
        systemLib.crane-nightly.buildPackage (common-attrs // {
          pname = "${name}-runtime";
          cargoArtifacts = common-deps-nightly;
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
        in
        pkgs.stdenv.mkDerivation {
          name = "${runtime.name}-optimized";
          phases = [ "installPhase" ];
          installPhase = ''
            mkdir -p $out/lib
            ${wasm-optimizer}/bin/wasm-optimizer \
            --input ${runtime}/lib/${name}_runtime.wasm \
            --output $out/lib/runtime.optimized.wasm
          '';
        };

    in
    {
      # Add the npm-buildpackage overlay to the perSystem's pkgs
      packages = rec {
        inherit wasm-optimizer;
        inherit common-deps;
        inherit common-deps-nightly;
        inherit common-bench-deps;

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
        composable-node = systemLib.crane-nightly.buildPackage (common-attrs // {
          name = "composable";
          cargoArtifacts = common-deps;
          cargoBuildCommand =
            "cargo build --release --package composable --features=builtin-wasm";
          DALI_RUNTIME = "${dali-runtime}/lib/runtime.optimized.wasm";
          PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
          COMPOSABLE_RUNTIME =
            "${composable-runtime}/lib/runtime.optimized.wasm";
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/composable $out/bin/composable
          '';
          meta = { mainProgram = "composable"; };
        });

        composable-node-release = systemLib.crane-nightly.buildPackage (common-attrs
          // {
          name = "composable";
          cargoArtifacts = common-deps;
          cargoBuildCommand = "cargo build --release --package composable";
          SUBSTRATE_CLI_GIT_COMMIT_HASH =
            if self ? rev then
              self.rev
            else
              builtins.abort "Cannot build the release node in a dirty repo.";
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/composable $out/bin/composable
          '';
          meta = { mainProgram = "composable"; };
        });

        composable-bench-node = systemLib.crane-nightly.cargoBuild (common-bench-attrs
          // {
          name = "composable";
          cargoArtifacts = common-bench-deps;
          cargoBuildCommand = "cargo build --release --package composable";
          DALI_RUNTIME = "${dali-bench-runtime}/lib/runtime.optimized.wasm";
          PICASSO_RUNTIME =
            "${picasso-bench-runtime}/lib/runtime.optimized.wasm";
          COMPOSABLE_RUNTIME =
            "${composable-bench-runtime}/lib/runtime.optimized.wasm";
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/composable $out/bin/composable
          '';
          meta = { mainProgram = "composable"; };
        });


      };

    };
}
