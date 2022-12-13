{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:

    let
      makeComposableNode = f:
        crane.nightly.buildPackage (f (systemCommonRust.common-attrs // {
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
        }));
    in {
      # Add the npm-buildpackage overlay to the perSystem's pkgs
      packages = rec {

        composable-node = makeComposableNode (node: node);

        composable-node-release = makeComposableNode (node:
          node // {
            SUBSTRATE_CLI_GIT_COMMIT_HASH = if self ? rev then
              self.rev
            else
              builtins.abort "Cannot build the release node in a dirty repo.";
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
