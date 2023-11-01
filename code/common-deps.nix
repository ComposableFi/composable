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
        common-attrs = subnix.subattrs // {
          src = rustSrc;
          buildInputs = with pkgs; [ openssl zstd zlib.dev ];
          nativeBuildInputs = with pkgs;
            [ clang openssl pkg-config ] ++ darwin-deps;
          doCheck = false;
          cargoCheckCommand = "true";
          # Don't build any wasm as we do it ourselves
          SKIP_WASM_BUILD = "1";
          NIX_BUILD_FLAKE = "true";
        };

        common-test-deps-attrs = subnix.subattrs // {
          src = rustSrc;
          buildInputs = with pkgs; [ openssl zstd ];
          nativeBuildInputs = with pkgs;
            [ clang openssl pkg-config ] ++ darwin-deps;
          doCheck = true;
          SKIP_WASM_BUILD = "1";
        };

        common-std-bench-attrs = common-attrs // {
          cargoExtraArgs = "--features=builtin-wasm,runtime-benchmarks";
        };
        common-wasm-bench-attrs = common-attrs // {
          cargoExtraArgs = "--features=runtime-benchmarks";
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
