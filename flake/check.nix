{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    apps = {
      nix-flake-check = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "nix-flake-check";
          runtimeInputs = [  ];
          text = ''
            NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM=1
            export NIXPKGS_ALLOW_UNSUPPORTED_SYSTEM 
            #NIX_DEBUG_COMMAND="" && [[ ''${ACTIONS_RUNNER_DEBUG-"true"} = "true" ]] && NIX_DEBUG_COMMAND=' --print-build-logs --debug --show-trace --verbose'
            set -o pipefail -o errexit
            NIXPKGS_ALLOW_BROKEN=1 nix flake check --keep-going --no-build --allow-import-from-derivation --no-update-lock-file --accept-flake-config --fallback --print-build-logs "$NIX_DEBUG_COMMAND" --impure --option sandbox relaxed --impure 2>&1 | tee "nix.check.log"  || true
            set +o pipefail +o errexit
            echo "exited with(https://github.com/NixOS/nix/issues/7464) $?" 
            grep --invert-match  "error: path [']/nix/store/[a-zA-Z0-9]\+-[a-zA-Z0-9\.-]\+['] is not valid" < "nix.check.log" \
            | grep --invert-match  "error: cannot substitute path [']/nix/store/[a-zA-Z0-9]\+-[a-zA-Z0-9\.-]\+['] \- no write access to the Nix store" \
            | grep --invert-match '^error: some errors were encountered during the evaluation' > "filtered.nix.check.log"
            RESULT=$(grep -c 'error:' < "filtered.nix.check.log")
            echo "Got errors $RESULT"
            if [[ $RESULT != 0 ]]; then exit "$RESULT"; fi
          '';
        };
      };
    };
    packages = {
      check = let
        # The order of these checks can still be improved.
        # In general, the shorter the check takes, the higher up it should be.
        checks = [
          "nixfmt-check"
          "deadnix-check"
          "taplo-check"
          "hadolint-check"
          "spell-check"
          "docs-static"
          "devnet-initialize-script-picasso-persistent"
          "common-deps"
          "common-test-deps"
          "cargo-fmt-check"
          "cargo-clippy-check"
          "cargo-deny-check"
          "cargo-udeps-check"
          "benchmarks-check"
          "unit-tests"
          "benchmarks-once-picasso"
          "benchmarks-once-composable"
          "prettier-check"
          "frontend-static"
          "check-picasso-integration-tests"
          "composable-node"
          "composable-bench-node"
          "polkadot-node-on-parity-kusama"
          "statemine-node"
          "bifrost-node"
          "acala-node"
          "simnode-tests"
          "simnode-tests-picasso"
          "simnode-tests-composable"
          "cmc-api"
          "cmc-api-image"
          "zombienet"
          "price-feed"
          "devnet-integration-tests"
        ];
        toCommand = check: ''
                  echo "🧐Checking ${check}..."
          				nix build .\#${check} --no-warn-dirty
                  if [ $? -eq 1 ]; then 
                    echo "❌Check ${check} FAILED"
                  else 
                    printf "\033[1A" # Remove the Checking... line                   
                    echo -e "\r\e[K✅Check ${check} PASSED"
                  fi
          			'';
        script = pkgs.lib.concatMapStrings toCommand checks;
      in pkgs.writeShellScriptBin "check" script;
    };
  };
}
