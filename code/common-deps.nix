{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, cargoTools, ... }: {
      _module.args.systemCommonRust = rec {

        mkRustSrc = cargoTools.mkRustSrc;
        rustSrc = cargoTools.mkRustSrc ./.;

        darwin-deps = pkgs.lib.optional pkgs.stdenv.isDarwin (with pkgs;
          with darwin.apple_sdk.frameworks; [
            Security
            SystemConfiguration
          ]);

        # Common env required to build the node
        common-attrs = subnix.subenv // {
          src = rustSrc;
          cargoCheckCommand = "true";
          NIX_BUILD_FLAKE = "true";
        };

        common-test-deps-attrs = subnix.subenv // {
          src = rustSrc;
          doCheck = true;
          SKIP_WASM_BUILD = "1";
        };

        common-std-bench-attrs = common-attrs // {
          cargoExtraArgs = "--features=builtin-wasm,runtime-benchmarks";
          SKIP_WASM_BUILD = "1";
        };
        common-wasm-bench-attrs = common-attrs // {
          cargoExtraArgs = "--features=runtime-benchmarks";
          SKIP_WASM_BUILD = "1";
        };
      };

      packages = rec {
        common-deps =
          crane.nightly.buildDepsOnly (systemCommonRust.common-attrs // { });
        common-deps-nightly =
          crane.nightly.buildDepsOnly (systemCommonRust.common-attrs // { });
        common-std-bench-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-std-bench-attrs // { });
        common-wasm-bench-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-wasm-bench-attrs // { });
        common-test-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-test-deps-attrs // { });
      };

    };
}
