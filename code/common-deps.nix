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
          RUSTC_STAGE = 1;
        };

        common-test-deps-attrs = subnix.subenv // {
          src = rustSrc;
          doCheck = true;
          SKIP_WASM_BUILD = "1";
          RUSTC_STAGE = 1;
        };

        common-std-bench-attrs = common-attrs // {
          cargoExtraArgs = "--features=builtin-wasm,runtime-benchmarks";
          SKIP_WASM_BUILD = "1";
          RUSTC_STAGE = 1;
        };
        common-wasm-bench-attrs = common-attrs // {
          cargoExtraArgs = "--features=runtime-benchmarks";
          SKIP_WASM_BUILD = "1";
          RUSTC_STAGE = 1;
        };
      };

      packages = rec {
        common-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-attrs // { SKIP_WASM_BUILD = "1"; });
        common-deps-nightly = crane.nightly.buildDepsOnly
          (systemCommonRust.common-attrs // { SKIP_WASM_BUILD = "1"; });
        common-std-bench-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-std-bench-attrs // {
            SKIP_WASM_BUILD = "1";
          });
        common-wasm-bench-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-wasm-bench-attrs // {
            SKIP_WASM_BUILD = "1";
          });
        common-test-deps = crane.nightly.buildDepsOnly
          (systemCommonRust.common-test-deps-attrs // {
            SKIP_WASM_BUILD = "1";
          });
      };

    };
}
