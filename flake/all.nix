{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {
      all-docs = pkgs.linkFarmFromDrvs "all-docs"
        (with self'.packages; [ docs-server docs-static ]);
      all-misc = pkgs.linkFarmFromDrvs "all-misc" (with self'.packages; [
        cargo-fmt-check
        cargo-udeps-check
        hadolint-check
        nixfmt-check
        deadnix-check
        prettier-check
        spell-check
        taplo-check
      ]);

      all = pkgs.linkFarmFromDrvs "all-ci-packages" (with self'.packages; [
        acala-node
        benchmarks-check
        bifrost-node
        cargo-clippy-check
        cargo-deny-check
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
        devnet-centauri
        devnet-dali
        devnet-dali-complete
        devnet-dali-image
        devnet-initialize-script-picasso-persistent
        devnet-integration-tests
        devnet-picasso-complete
        frontend-static
        hyperspace-dali
        hyperspace-dali-image
        polkadot-node-on-parity-kusama
        statemine-node
        subwasm
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
