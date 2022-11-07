{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      run-simnode-tests = chain:
        pkgs.writeShellScriptBin "run-simnode-tests-${chain}" ''
          ${self'.packages.simnode-tests}/bin/simnode-tests --chain=${chain} \
          --base-path=/tmp/db/var/lib/composable-data/ \
          --pruning=archive \
          --execution=wasm
        '';
    in {
      packages = {
        simnode-tests = crane.nightly.cargoBuild (systemCommonRust.common-attrs
          // {
            pnameSuffix = "-simnode";
            cargoArtifacts = self'.packages.common-deps;
            cargoBuildCommand =
              "cargo build --release --package simnode-tests --features=builtin-wasm";
            DALI_RUNTIME =
              "${self'.packages.dali-runtime}/lib/runtime.optimized.wasm";
            PICASSO_RUNTIME =
              "${self'.packages.picasso-runtime}/lib/runtime.optimized.wasm";
            COMPOSABLE_RUNTIME =
              "${self'.packages.composable-runtime}/lib/runtime.optimized.wasm";
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/simnode-tests $out/bin/simnode-tests
            '';
            meta = { mainProgram = "simnode-tests"; };
          });
      };

      apps = {
        simnode-tests-composable = self.inputs.flake-utils.lib.mkApp {
          drv = run-simnode-tests "composable";
        };
        simnode-tests-picasso = self.inputs.flake-utils.lib.mkApp {
          drv = run-simnode-tests "picasso";
        };
        simnode-tests-dali-rococo = self.inputs.flake-utils.lib.mkApp {
          drv = run-simnode-tests "dali-rococo";
        };

      };
    };
}
