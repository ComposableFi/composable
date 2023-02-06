{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {
      all = pkgs.linkFarmFromDrvs "all-ci-packages"
        (with self'.packages;
          [
            acala-node
            benchmarks-check
            bifrost-node
            cargo-clippy-check
            cargo-deny-check
            cargo-fmt-check
            cargo-udeps-check
            check-composable-benchmarks-ci
            check-dali-benchmarks-ci
            check-dali-integration-tests
            check-picasso-benchmarks-ci
            check-picasso-integration-tests
            cmc-api
            cmc-api-image
            composable-bench-node
            composable-node
            dali-subxt-client
            deadnix-check
            devnet-dali
            devnet-dali-complete
            devnet-dali-image
            devnet-initialize-script-picasso-persistent
            devnet-integration-tests
            devnet-picasso-complete
            docs-static
            frontend-static
            hadolint-check
            hyperspace-dali
            hyperspace-dali-image
            nixfmt-check
            polkadot-node
            prettier-check
            spell-check
            statemine-node
            taplo-check
            unit-tests
            zombienet
          ] ++ (if system == "x86_64-linux" then
            [ devnet-centauri ]
          else
            [ ])); # TODO: get this list from system-filter

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
          "benchmarks-once-dali"
          "benchmarks-once-picasso"
          "benchmarks-once-composable"
          "prettier-check"
          "frontend-static"
          "check-dali-integration-tests"
          "check-picasso-integration-tests"
          "composable-node"
          "composable-bench-node"
          "polkadot-node"
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
