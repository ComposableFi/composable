{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {
      all-ci-packages = pkgs.linkFarmFromDrvs "all-ci-packages"
        (with self'.packages; [
          cargo-fmt-check
          cargo-clippy-check
          cargo-deny-check
          cargo-udeps-check
          taplo-check
          prettier-check
          nixfmt-check
          deadnix-check
          spell-check
          docs-static # TODO(cor): deployment
          frontend-static
          hadolint-check
          benchmarks-check
          unit-tests
          composable-node
          composable-bench-node
          polkadot-node
          statemine-node
          bifrost-node
          acala-node
          dali-subxt-client
          zombienet
          devnet-initialize-script-picasso-persistent
          devnet-dali-complete
          devnet-picasso-complete
          devnet-dali
          check-dali-benchmarks-ci
          check-picasso-benchmarks-ci
          check-composable-benchmarks-ci
          cmc-api
          cmc-api-image # TODO(cor): needs to be pushed to docker
          devnet-container # errored in ci, our runners need the `kvm` feature in order to build this

          # unsure about these
          check-dali-integration-tests
          check-picasso-integration-tests
          devnet-integration-tests

          # TODO(cor): filter these out on arm, but build them on x64
          # disabled because this is not properly nixified
          # devnet-centauri 
          hyperspace-dali
          hyperspace-dali-image
          # bridge-devnet-dali-container 
        ]);

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
