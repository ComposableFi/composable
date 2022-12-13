{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
      _module.args.systemCommonRust = rec {

        rustSrc = pkgs.lib.cleanSourceWith {
          filter = pkgs.lib.cleanSourceFilter;
          src = pkgs.lib.cleanSourceWith {
            filter = let
              isProto = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".proto" name;
              isJSON = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".json" name;
              isREADME = name: type:
                type == "regular"
                && pkgs.lib.strings.hasSuffix "README.md" name;
              isDir = name: type: type == "directory";
              isCargo = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".toml" name
                || type == "regular" && pkgs.lib.strings.hasSuffix ".lock" name;
              isRust = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".rs" name;
              customFilter = name: type:
                ((isCargo name type) || (isRust name type) || (isDir name type)
                  || (isREADME name type) || (isJSON name type)
                  || (isProto name type));
            in pkgs.nix-gitignore.gitignoreFilterPure customFilter
            [ ../.gitignore ] ./.;
            src = ./.;
          };
        };
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
