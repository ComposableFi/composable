{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, devnetTools, cargoTools, ... }:

    let
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
          SKIP_WASM_BUILD = "1";
          cargoArtifacts = self'.packages.common-deps;
          cargoBuildCommand = "cargo build --release --package ${name}";
          cargoExtraArgs = "--features=builtin-wasm";
          CW_CVM_OUTPOST_WASM_PATH = "${
              self.inputs.cvm.packages."${system}".cw-cvm-outpost
            }/lib/cw_cvm_outpost.wasm";
          CW_CVM_EXECUTOR_WASM_PATH = "${
              self.inputs.cvm.packages."${system}".cw-cvm-executor
            }/lib/cw_cvm_executor.wasm";
          CW_20_BASE_WASM_PATH = "${
              self.inputs.cosmos.packages.${system}.cw20-base
            }/lib/cw20_base.wasm";
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
      ccw-src = cargoTools.mkRustSrc ./parachain/frame/cosmwasm/cli;
    in {
      packages = rec {
        ccw-patch = pkgs.stdenv.mkDerivation rec {
          name = "ccw-patch";
          pname = "${name}";
          buildInputs = [ self'.packages.picasso-runtime-dev ];
          src = ccw-src;
          patchPhase = "true";
          installPhase = ''
            mkdir --parents $out
            set +e
            diff --unified $src/src/substrate/subxt_api.rs ${self'.packages.picasso-runtime-dev}/include/picasso_runtime.rs > $out/picasso_runtime.rs.patch
            if [[ $? -ne 1 ]] ; then
              echo "Failed diff"              
            fi                          
            set -e 
          '';
          dontFixup = true;
          dontStrip = true;
        };

        ccw-patched-src = pkgs.stdenv.mkDerivation rec {
          name = "ccw-patched-src";
          pname = "${name}";
          src = ccw-src;
          buildInputs = with pkgs; [ git ];
          patchFlags = "--strip=0";
          patchPhase = "true";

          installPhase = ''
            mkdir --parents $out
            cp --recursive --no-preserve=mode,ownership $src/. $out/

            cd $out/src/substrate
            patch subxt_api.rs ${self'.packages.ccw-patch}/picasso_runtime.rs.patch
          '';
          dontFixup = true;
          dontStrip = true;
        };

        ccw = crane.nightly.buildPackage (subnix.subenv // rec {
          name = "ccw";
          pname = name;
          cargoBuildCommand = "cargo build --release --package ${name}";
          meta = { mainProgram = name; };
          src = ccw-patched-src;
        });

        composable-node-image = toDockerImage composable-node;
        composable-node = makeComposableNode self'.packages.picasso-runtime
          self'.packages.composable-runtime;
        default = composable-node;

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
