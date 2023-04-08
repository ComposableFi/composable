{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {

      all-deps = pkgs.linkFarmFromDrvs "all-deps" (with self'.packages; [
        acala-node
        bifrost-node
        polkadot-node-dep
        polkadot-node-on-parity-kusama
        polkadot-node-on-parity-polkadot
        statemine-node
        subwasm
        zombienet
      ]);

      all-docs = pkgs.linkFarmFromDrvs "all-docs"
        (with self'.packages; [ docs-server docs-static ]);
      all-misc = pkgs.linkFarmFromDrvs "all-misc" (with self'.packages; [
        cargo-fmt-check
        hadolint-check
        nixfmt-check
        deadnix-check
        prettier-check
        spell-check
        taplo-check
      ]);

      all = pkgs.linkFarmFromDrvs "all-ci-packages" (with self'.packages; [
        benchmarks-check
        cargo-clippy-check
        cargo-deny-check
        check-composable-benchmarks-ci
        check-picasso-benchmarks-ci
        check-picasso-integration-tests
        check-picasso-benchmarks-ci
        check-picasso-integration-tests
        cmc-api
        cmc-api-image
        composable-bench-node
        composable-node
        devnet-centauri
        devnet-picasso
        devnet-picasso-image
        devnet-initialize-script-picasso-persistent
        devnet-integration-tests
        devnet-picasso-complete
        unit-tests
        hyperspace-composable-rococo-picasso-rococo
        hyperspace-composable-rococo-picasso-rococo-image
      ]);

      all-frontend = pkgs.linkFarmFromDrvs "all-frontend"
        (with self'.packages; [ frontend-static ]);

      docker-images-to-push = pkgs.linkFarmFromDrvs "docker-images-to-push"
        (with self'.packages; [
          cmc-api-image
          hyperspace-composable-rococo-picasso-rococo-image
          devnet-picasso-image
        ]);
    };
  };
}
