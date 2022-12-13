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
              ((isCargo name type) || (isRust name type) || (isDir name type)
                || (isREADME name type) || (isJSON name type)
                || (isProto name type));
          in pkgs.nix-gitignore.gitignoreFilterPure customFilter
          [ ../.gitignore ] ./.;
          src = ./.;
        };
      };
    in {
      packages = {
        simnode-tests = crane.nightly.cargoBuild (systemCommonRust.common-attrs
          // {
            pnameSuffix = "-simnode";
            src = rustSrc;
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
