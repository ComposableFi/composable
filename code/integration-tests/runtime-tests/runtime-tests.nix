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
            pkgs.lib.meta.getExe self'.packages.default
          } 2>&1 & ) | tee devnet-picasso.log &
          wait_for_log () {
           until test -f "$1"; do
              sleep 1 
              echo "$2"
            done
          }

          wait_for_log "devnet-picasso.log" "waiting network start"
          TIMEOUT=300
          COMMAND="( tail --follow --lines=0  devnet-picasso.log & ) | grep --max-count=1 \"Network launched 🚀🚀\""
          set +o errexit
          timeout $TIMEOUT bash -c "$COMMAND"
          START_RESULT="$?"
          set -o errexit
          if [[ $START_RESULT -ne 0 ]] ; then 
            printf "failed to start devnet within %s with exit code %s" "$TIMEOUT" "$START_RESULT"
            exit $START_RESULT
          fi
          exit $START_RESULT
        '';
    };
    apps = {
      devnet-integration-tests = self.inputs.flake-utils.lib.mkApp {
        drv = self'.packages.devnet-integration-tests;
      };
    };
  };
}
