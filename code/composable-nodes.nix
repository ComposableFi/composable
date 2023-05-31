{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , devnetTools, ... }:

    let
      rust-src-template = root:
        pkgs.lib.cleanSourceWith {
          filter = pkgs.lib.cleanSourceFilter;
          src = pkgs.lib.cleanSourceWith {
            filter = let
              isProto = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".proto" name;
              isJSON = name: type:
                type == "regular" && pkgs.lib.strings.hasSuffix ".json" name;
              isREADME = name: type:
                type == "regular"
                && pkgs.lib.strings.hasSuffix "README.md" name;
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
            src = root;
          };
        };

      rustSrc = rust-src-template ./.;
      toDockerImage = { ... }@drv:
        (pkgs.dockerTools.buildImage {
          name = drv.name or drv.pname or "image";
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ drv pkgs.glibc ] ++ devnetTools.withBaseContainerTools;
            pathsToLink = [ "/bin" ];
          };
          config = {
            Entrypoint =
              [ "${pkgs.lib.getBin drv}/bin/${pkgs.lib.getName drv}" ];
            Env =
              [ "SSL_CERT_FILE=${pkgs.cacert}/etc/ssl/certs/ca-bundle.crt" ];
          };
        });

      makeComposableNode = picasso-runtime: composable-runtime:
        crane.nightly.buildPackage (systemCommonRust.common-attrs // rec {
          name = "composable";
          cargoArtifacts = self'.packages.common-deps;
          cargoBuildCommand = "cargo build --release --package ${name}";
          cargoExtraArgs = "--features=builtin-wasm";
          PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
          COMPOSABLE_RUNTIME =
            "${composable-runtime}/lib/runtime.optimized.wasm";
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
        ccw = crane.nightly.buildPackage (systemCommonRust.common-attrs // rec {
          name = "ccw";
          cargoBuildCommand = "cargo build --release --package ${name}";
          meta = { mainProgram = name; };
          src = rust-src-template ./parachain/frame/cosmwasm/cli;
        });

        composable-node-image = toDockerImage composable-node;
        composable-node = makeComposableNode self'.packages.picasso-runtime
          self'.packages.composable-runtime;
        composable-testfast-node =
          makeComposableNode self'.packages.picasso-testfast-runtime
          self'.packages.composable-testfast-runtime;

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
