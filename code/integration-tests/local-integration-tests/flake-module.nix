{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , cargoTools, ... }: {
      packages = let
        checkIntegrationTests = { chainName }:
          crane.nightly.cargoBuild (systemCommonRust.common-attrs // {
            src = cargoTools.rustSrc;
            pname = "${chainName}-local-integration-tests";
            doInstallCargoArtifacts = false;
            cargoArtifacts = self'.packages.common-test-deps;
            cargoBuildCommand = "cargo test --package local-integration-tests";
            cargoExtraArgs =
              "--features=local-integration-tests,${chainName},std --no-default-features --verbose";
          });
      in {
        check-dali-integration-tests =
          checkIntegrationTests { chainName = "dali"; };
        check-picasso-integration-tests =
          checkIntegrationTests { chainName = "picasso"; };

        local-integration-tests =
          pkgs.writeShellScriptBin "local-integration-tests" ''
            cd code
            RUST_BACKTRACE=full \
            SKIP_WASM_BUILD=1 \
            RUST_LOG=trace,bdd=trace,parity-db=warn,trie=warn,runtime=trace,substrate-relay=trace,bridge=trace,xcmp=trace,xcm=trace \
            cargo +nightly test --package local-integration-tests --features=local-integration-tests,picasso,  --no-default-features -- --nocapture --test-threads=1
          '';
      };

      apps = {
        local-integration-tests = self.inputs.flake-utils.lib.mkApp {
          drv = self'.packages.local-integration-tests;
        };
      };

    };
}
