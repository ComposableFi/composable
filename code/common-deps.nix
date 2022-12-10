{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
      _module.args.systemCommonRust = rec {

        rustSrc = let
          directoryBlacklist = [ "runtime-tests" ];
          fileBlacklist = [
            # does not makes sense to black list,
            # if we changed some version of tooling(seldom), we want to rebuild
            # so if we changed version of tooling, nix itself will detect invalidation and rebuild
            # "flake.lock"
          ];
        in pkgs.lib.cleanSourceWith {
          filter = pkgs.lib.cleanSourceFilter;
          src = pkgs.lib.cleanSourceWith {
            filter = let
              isBlacklisted = name: type:
                let
                  blacklist = if type == "directory" then
                    directoryBlacklist
                  else if type == "regular" then
                    fileBlacklist
                  else
                    [ ]; # symlink, unknown
                in builtins.elem (baseNameOf name) blacklist;
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
            in pkgs.nix-gitignore.gitignoreFilterPure customFilter
            [ ../.gitignore ] ./.;
            src = ./.;
          };
        };
        # TODO: cosmwasm attrs
        substrate-attrs = {
          LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
            pkgs.stdenv.cc.cc.lib
            pkgs.llvmPackages.libclang.lib
          ];
          LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          PROTOC = "${pkgs.protobuf}/bin/protoc";
          ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
        };

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

        # TODO: refactor as mkOverride common-attrs
        common-test-deps-attrs = substrate-attrs // {
          src = rustSrc;
          buildInputs = with pkgs; [ openssl zstd ];
          nativeBuildInputs = with pkgs;
            [ clang openssl pkg-config ] ++ pkgs.lib.optional stdenv.isDarwin
            (with darwin.apple_sdk.frameworks; [
              Security
              SystemConfiguration
            ]);
          doCheck = true;
          SKIP_WASM_BUILD = "1";
        };

        common-bench-attrs = common-attrs // {
          cargoExtraArgs = "--features=builtin-wasm,runtime-benchmarks";
        };

      };
      # Add the npm-buildpackage overlay to the perSystem's pkgs
      packages = rec {
        common-deps =
          crane.nightly.buildDepsOnly (systemCommonRust.common-attrs // { });
        common-deps-nightly =
          crane.nightly.buildDepsOnly (systemCommonRust.common-attrs // { });
        common-bench-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-bench-attrs // { });
        common-test-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-test-deps-attrs // { });

        wasm-optimizer = crane.stable.buildPackage
          (systemCommonRust.common-attrs // {
            cargoCheckCommand = "true";
            pname = "wasm-optimizer";
            cargoArtifacts = common-deps;
            cargoBuildCommand =
              "cargo build --release --package wasm-optimizer";
            version = "0.1.0";
            # NOTE: we copy more then needed, but tht is simpler to setup, we depend on substrate for sure so
          });
      };

    };
}
