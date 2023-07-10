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
        runtimeInputs = with pkgs; [ curl dasel nodejs coreutils ];
        name = "devnet-integration-tests";
        text = ''
          # shellcheck disable=SC2069
          ( ${
            pkgs.lib.meta.getExe self'.packages.default
          } 2>&1 & ) | tee devnet-picasso.log &
          wait_for_log () {
           until test -f "$1"; do
              sleep 1 
              printf "============================== waiting =========================="
              echo "$2"
            done
          }

          wait_for_log "devnet-picasso.log" "waiting network start"
          TIMEOUT=300
          COMMAND="( tail --follow --lines=0  devnet-picasso.log & ) | grep picasso | grep --max-count=1 \"Network launched 🚀🚀\""
          set +o errexit
          timeout $TIMEOUT bash -c "$COMMAND"
          START_RESULT="$?"
          set -o errexit
          if [[ $START_RESULT -ne 0 ]] ; then 
            printf "failed to start devnet within %s with exit code %s" "$TIMEOUT" "$START_RESULT"
            exit $START_RESULT
          fi

          cd code/integration-tests/runtime-tests || exit
          npm install -q
          # shellcheck disable=SC2069
          export ENDPOINT=127.0.0.1:9988 ENDPOINT_RELAYCHAIN=127.0.0.1:9944 && npm run test_basic 2>&1>runtime-tests.log &
          RUNTIME_TESTS_PID=$!
          wait_for_log "runtime-tests.log" "waiting tests start"
          tail --follow runtime-tests.log &
          ( tail --follow --lines=0 runtime-tests.log & ) | ( grep --max-count=5 "API-WS: disconnected from" >stop.log & )
          ( while : ; do if test "$( wc --lines stop.log | cut --delimiter " " --fields 1 )" -gt 4; then kill -s SIGKILL $RUNTIME_TESTS_PID && echo "Failed" && exit 42; fi; sleep 1; done ) &
          wait $RUNTIME_TESTS_PID
          exit $?
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
