{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
      # Add the npm-buildpackage overlay to the perSystem's pkgs
      packages = rec {
        composable-node = crane.nightly.buildPackage
          (systemCommonRust.common-attrs // {
            name = "composable";
            cargoArtifacts = self'.packages.common-deps;
            cargoBuildCommand =
              "cargo build --release --package composable --features=builtin-wasm";
            DALI_RUNTIME =
              "${self'.packages.dali-runtime}/lib/runtime.optimized.wasm";
            PICASSO_RUNTIME =
              "${self'.packages.picasso-runtime}/lib/runtime.optimized.wasm";
            COMPOSABLE_RUNTIME =
              "${self'.packages.composable-runtime}/lib/runtime.optimized.wasm";
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/composable $out/bin/composable
            '';
            meta = { mainProgram = "composable"; };
          });

        composable-node-release = crane.nightly.buildPackage
          (systemCommonRust.common-attrs // {
            name = "composable";
            cargoArtifacts = systemCommonRust.common-deps;
            cargoBuildCommand = "cargo build --release --package composable";
            SUBSTRATE_CLI_GIT_COMMIT_HASH = if self ? rev then
              self.rev
            else
              builtins.abort "Cannot build the release node in a dirty repo.";
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/composable $out/bin/composable
            '';
            meta = { mainProgram = "composable"; };
          });

        composable-bench-node = crane.nightly.cargoBuild
          (systemCommonRust.common-bench-attrs // {
            name = "composable";
            cargoArtifacts = self'.packages.common-bench-deps;
            cargoBuildCommand = "cargo build --release --package composable";
            DALI_RUNTIME =
              "${self'.packages.dali-bench-runtime}/lib/runtime.optimized.wasm";
            PICASSO_RUNTIME =
              "${self'.packages.picasso-bench-runtime}/lib/runtime.optimized.wasm";
            COMPOSABLE_RUNTIME =
              "${self'.packages.composable-bench-runtime}/lib/runtime.optimized.wasm";
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/composable $out/bin/composable
            '';
            meta = { mainProgram = "composable"; };
          });
      };
    };
}
