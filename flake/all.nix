{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {
      all-docs = pkgs.linkFarmFromDrvs "all-ci-packages" (with self'.packages; [
        docs-server
        docs-static
      ]);

      all = pkgs.linkFarmFromDrvs "all-ci-packages" (with self'.packages; [
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
        devnet-centauri
        devnet-dali
        devnet-dali-complete
        devnet-dali-image
        devnet-initialize-script-picasso-persistent
        devnet-integration-tests
        devnet-picasso-complete
        frontend-static
        hadolint-check
        hyperspace-dali
        hyperspace-dali-image
        nixfmt-check
        polkadot-node
        prettier-check
        spell-check
        statemine-node
        subwasm
        taplo-check
        unit-tests
      ]);

      docker-images-to-push = pkgs.linkFarmFromDrvs "docker-images-to-push"
        (with self'.packages; [
          cmc-api-image
          devnet-dali-image
          hyperspace-dali-image
        ]);
    };
  };
}
