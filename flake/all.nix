{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {
      all = pkgs.linkFarmFromDrvs "all-ci-packages" (with self'.packages;
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
    };
  };
}
