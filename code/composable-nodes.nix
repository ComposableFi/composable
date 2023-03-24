{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:

    let
      rustSrc = pkgs.lib.cleanSourceWith {
        filter = pkgs.lib.cleanSourceFilter;
        src = pkgs.lib.cleanSourceWith {
          filter = let
            isProto = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".proto" name;
            isJSON = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".json" name;
            isREADME = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix "README.md" name;
            isDir = name: type: type == "directory";
            isCargo = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".toml" name
              || type == "regular" && pkgs.lib.strings.hasSuffix ".lock" name;
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
          [ ../.gitignore ] ./.;
          src = ./.;
        };
      };

      makeComposableNode = f:
        crane.nightly.buildPackage (f (systemCommonRust.common-attrs // {
          name = "composable";
          cargoArtifacts = self'.packages.common-deps;
          cargoBuildCommand =
            "cargo build --release --package composable --features=builtin-wasm,composable";
          DALI_RUNTIME =
            "${self'.packages.dali-runtime}/lib/runtime.optimized.wasm";
          PICASSO_RUNTIME =
            "${self'.packages.picasso-runtime}/lib/runtime.optimized.wasm";
          COMPOSABLE_RUNTIME =
            "${self'.packages.composable-runtime}/lib/runtime.optimized.wasm";
          installPhaseCommand = ''
            mkdir -p $out/bin
            cp target/release/composable $out/bin/composable
          '';
          meta = { mainProgram = "composable"; };
        }));
    in {
      packages = rec {

        composable-node = makeComposableNode (node: node);

        composable-node-dali = makeComposableNode (node:
          node // {
            PICASSO_RUNTIME = node.DALI_RUNTIME;
            COMPOSABLE_RUNTIME = node.DALI_RUNTIME;
          });

        composable-node-picasso = makeComposableNode (node:
          node // {
            PICASSO_RUNTIME = node.PICASSO_RUNTIME;
            COMPOSABLE_RUNTIME = node.COMPOSABLE_RUNTIME;
          });

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
            installPhaseCommand = ''
              mkdir -p $out/bin
              cp target/release/composable $out/bin/composable
            '';
            meta = { mainProgram = "composable"; };
          });
      };
    };
}
