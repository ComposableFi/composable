{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, ... }: {
      _module.args.systemCommonRust = rec {

        mkRustSrc = path:
          pkgs.lib.cleanSourceWith {
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
                  || type == "regular"
                  && pkgs.lib.strings.hasSuffix ".lock" name;
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
              [ ../.gitignore ] path;
              src = path;
            };
          };

        rustSrc = mkRustSrc ./.;

        darwin-deps = pkgs.lib.optional pkgs.stdenv.isDarwin (with pkgs;
          with darwin.apple_sdk.frameworks; [
            Security
            SystemConfiguration
          ]);

        # Common env required to build the node
        common-attrs = subnix.subattrs // {
          src = rustSrc;
          buildInputs = with pkgs; [ openssl zstd ];
          nativeBuildInputs = with pkgs;
            [ clang openssl pkg-config ] ++ darwin-deps;
          doCheck = false;
          cargoCheckCommand = "true";
          # Don't build any wasm as we do it ourselves
          SKIP_WASM_BUILD = "1";
        };

        # TODO: refactor as mkOverride common-attrs
        common-test-deps-attrs = subnix.subattrs // {
          src = rustSrc;
          buildInputs = with pkgs; [ openssl zstd ];
          nativeBuildInputs = with pkgs;
            [ clang openssl pkg-config ] ++ darwin-deps;
          doCheck = true;
          SKIP_WASM_BUILD = "1";
        };

        common-bench-attrs = common-attrs // {
          cargoExtraArgs = "--features=builtin-wasm,runtime-benchmarks";
        };
      };

      packages = rec {
        common-deps =
          crane.nightly.buildDepsOnly (systemCommonRust.common-attrs // { });
        common-deps-nightly =
          crane.nightly.buildDepsOnly (systemCommonRust.common-attrs // { });
        common-bench-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-bench-attrs // { });
        common-test-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-test-deps-attrs // { });
      };

    };
}
