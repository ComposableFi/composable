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
      toDockerImage = package:
        self.inputs.bundlers.bundlers."${system}".toDockerImage package;
      makeComposableNode = picasso-runtime:
        crane.nightly.buildPackage (systemCommonRust.common-attrs // rec {
          name = "composable";
          cargoArtifacts = self'.packages.common-deps;
          cargoBuildCommand = "cargo build --release --package ${name}";
          cargoExtraArgs = "--features=builtin-wasm";
          PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
          COMPOSABLE_RUNTIME =
            "${self'.packages.composable-runtime}/lib/runtime.optimized.wasm";
          installPhaseCommand = ''
            mkdir -p $out/bin
            echo "built with ${cargoExtraArgs}"
            cp target/release/composable $out/bin/composable
          '';
          meta = { mainProgram = name; };
        } // {
          SUBSTRATE_CLI_GIT_COMMIT_HASH = if self ? rev then
            self.rev
          else
            builtins.trace "WARNING: no tracked"
            "0000000000000000000000000000000000000000";
        });
    in {
      packages = rec {

        composable-node-image = toDockerImage composable-node;
        composable-node = makeComposableNode self'.packages.picasso-runtime;
        composable-testfast-node =
          makeComposableNode self'.packages.picasso-testfast-runtime;

        composable-bench-node = crane.nightly.cargoBuild
          (systemCommonRust.common-std-bench-attrs // rec {
            name = "composable";
            cargoArtifacts = self'.packages.common-std-bench-deps;
            cargoBuildCommand = "cargo build --release --package ${name}";
            PICASSO_RUNTIME =
              "${self'.packages.picasso-bench-runtime}/lib/runtime.optimized.wasm";
            COMPOSABLE_RUNTIME =
              "${self'.packages.composable-bench-runtime}/lib/runtime.optimized.wasm";
            installPhaseCommand = ''
              mkdir -p $out/bin
              cp target/release/${name} $out/bin/${name}
            '';
            meta = { mainProgram = name; };
          });
      };
    };
}
