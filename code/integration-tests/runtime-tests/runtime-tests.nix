{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {
      runtime-tests = pkgs.stdenv.mkDerivation {
        name = "runtime-tests";
        src =
          builtins.filterSource (path: _type: baseNameOf path != "node_modules")
          ./.;
        dontUnpack = true;
        installPhase = ''
          mkdir $out/
          cp -r $src/. $out/
        '';
      };

      prettier-check = pkgs.stdenv.mkDerivation {
        name = "prettier-check";
        dontUnpack = true;
        buildInputs = [ pkgs.nodePackages.prettier runtime-tests ];
        installPhase = ''
          mkdir $out
          prettier \
          --config="${runtime-tests}/.prettierrc" \
          --ignore-path="${runtime-tests}/.prettierignore" \
          --check \
          --loglevel=debug \
          ${runtime-tests}
        '';
      };

      devnet-integration-tests = pkgs.writeShellApplication {
        runtimeInputs = with pkgs; [
          curl
          dasel
          nodejs
          coreutils
          process-compose
        ];
        name = "devnet-integration-tests";
        text = ''
          # shellcheck disable=SC2069
          ( ${
            pkgs.lib.meta.getExe self'.packages.default
          } 2>&1 & ) | tee devnet-xc.log &

          process-compose-stop() {
            for i in $(process-compose process list); do process-compose process stop "$i"; done
          }

          TRIES=0
          START_RESULT=1
          while test $TRIES -le 30; do
            set +o errexit
            curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://127.0.0.1:32201
            START_RESULT=$?
            set -o errexit
            if test $START_RESULT -eq 0; then
              process-compose-stop
              break
            fi
            ((TRIES=TRIES+1))
            sleep 3
          done
          process-compose-stop
          exit $START_RESULT              
        '';
      };
    };
    apps = {
      devnet-integration-tests = self.inputs.flake-utils.lib.mkApp {
        drv = self'.packages.devnet-integration-tests;
      };
    };
  };
}
