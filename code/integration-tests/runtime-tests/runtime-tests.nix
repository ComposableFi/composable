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

      devnet-integration-tests =
        pkgs.writeShellScriptBin "devnet-integration-tests" ''
          ( ${
            pkgs.lib.meta.getExe self'.packages.devnet-dali
          } 2>&1 & ) | tee devnet-dali.log &
          wait_for_log () {
           until test -f "$1"; do
              sleep 1 
              echo "$2"
            done
          }

          wait_for_log "devnet-dali.log" "waiting network start"
          TIMEOUT=240
          COMMAND="( tail --follow --lines=0  devnet-dali.log & ) | grep --max-count=1 \"Network launched 🚀🚀\""
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
          export ENDPOINT=127.0.0.1:9988 ENDPOINT_RELAYCHAIN=127.0.0.1:9944 && npm run test_short 2>&1>runtime-tests.log &
          RUNTIME_TESTS_PID=$!
          wait_for_log "runtime-tests.log" "waiting tests start"
          tail --follow runtime-tests.log &
          ( tail --follow --lines=0 runtime-tests.log & ) | ( grep --max-count=5 "API-WS: disconnected from" >stop.log & )
          ( while : ; do if test $( wc --lines stop.log | cut --delimiter " " --fields 1 ) -gt 4; then kill -s SIGKILL $RUNTIME_TESTS_PID && echo "Failed" && exit 42; fi; sleep 1; done ) &
          wait $RUNTIME_TESTS_PID
          exit $?
        '';
    };
    apps = {
      devnet-integration-tests = self.inputs.flake-utils.lib.mkApp {
        drv = self'.packages.devnet-integration-tests;
      };
    };
  };
}
