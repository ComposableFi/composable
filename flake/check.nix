{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    apps = {
      nix-flake-check = let basicShellIsolation = [ pkgs.nix ];
      in {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "nix-flake-check";
          runtimeInputs = basicShellIsolation;
          text = ''
            NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM=1
            export NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM 
            NIX_DEBUG_ARGS=""
            if [[ ''${ACTIONS_RUNNER_DEBUG-"false"} = "true" ]]; then
              NIX_DEBUG_ARGS=' --print-build-logs --debug --show-trace --verbose'
            fi
            # some nix bug, works fine locally
            # shellcheck disable=SC2086
            # nix flake show --allow-import-from-derivation --fallback --keep-failed --no-write-lock-file --accept-flake-config --no-update-lock-file --system "${system}" $NIX_DEBUG_ARGS

            set -o pipefail -o errexit
            # shellcheck disable=SC2086
            NIXPKGS_ALLOW_BROKEN=1 nix flake check --keep-going --no-build --allow-import-from-derivation --accept-flake-config --no-update-lock-file --accept-flake-config --system "${system}" --fallback $NIX_DEBUG_ARGS --impure --option sandbox relaxed 2>&1 | tee "nix.check.log"  || true
            set +o pipefail +o errexit
            echo "exited with(https://github.com/NixOS/nix/issues/7464) $?" 
            grep --invert-match  "error: path [']/nix/store/[a-zA-Z0-9]\+-[a-zA-Z0-9\.-]\+['] is not valid" < "nix.check.log" |
            grep --invert-match  "error: cannot substitute path [']/nix/store/[a-zA-Z0-9]\+-[a-zA-Z0-9\.-]\+['] \- no write access to the Nix store" |
            grep --invert-match '^error: some errors were encountered during the evaluation' |
            grep --invert-match "error: a \'aarch64-darwin\' with features" > "filtered.nix.check.log"
            RESULT=$(grep -c 'error:' < "filtered.nix.check.log")
            echo "Got errors $RESULT"
            if [[ $RESULT != 0 ]]; then exit "$RESULT"; fi
          '';
        };
      };
      run-in-docker = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "run-in-docker";
          runtimeInputs = [ ];
          text = ''
            docker run --rm --volume /var/run/docker.sock:/var/run/docker.sock --volume nix:/nix -it nixos/nix bash -c "nix run composable#''${1-} --print-build-logs --extra-experimental-features nix-command --extra-experimental-features flakes --option sandbox relaxed --show-trace --accept-flake-config" 
          '';
        };
      };
    };
  };
}
