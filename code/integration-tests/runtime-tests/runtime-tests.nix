{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {
      devnet-integration-tests = pkgs.writeShellApplication {
        runtimeInputs = with pkgs;
          with self'.packages; [
            curl
            dasel
            nodejs
            coreutils
            process-compose
            centaurid
            osmosisd
          ];
        name = "devnet-integration-tests";
        text = ''
          # shellcheck disable=SC2069
          ( ${
            pkgs.lib.meta.getExe self'.packages.devnet-xc-fresh-background
          } 2>&1 & ) | tee devnet-xc.log &

          process-compose-stop() {
            for i in $(process-compose process list)
            do
              process-compose process stop "$i"
            done
          }

          TRIES=0
          START_RESULT=1
          while test $TRIES -le 64; do
            set +o errexit
            curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://127.0.0.1:9988
            START_RESULT=$?
            echo "picasso $START_RESULT"
            if test $START_RESULT -eq 0; then
              curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://127.0.0.1:29988
              START_RESULT=$?
              echo "composable $START_RESULT"
            fi
            set -o errexit            
            if test $START_RESULT -eq 0; then
              process-compose-stop
              break
            fi
            ((TRIES=TRIES+1))
            sleep 8
          done

          # here nodes are up and running, binaries in path, npm is here too

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
