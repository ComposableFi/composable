{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {

      all-deps = pkgs.linkFarmFromDrvs "all-deps" (with self'.packages; [
        acala-node
        bifrost-node
        rococo-runtime-from-dep
        kusama-runtime-on-parity
        polkadot-runtime-on-parity
        polkadot-node-from-dep
        polkadot-node-on-parity-kusama
        polkadot-node-on-parity-polkadot
        polkadot-node-on-parity-westend
        polkadot-node-on-parity-rococo
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

      all-benchmarks = pkgs.linkFarmFromDrvs "all-misc" (with self'.packages; [
        check-composable-benchmarks-ci
        check-picasso-benchmarks-ci
        composable-bench-node
      ]);

      all = pkgs.linkFarmFromDrvs "all-ci-packages" (with self'.packages; [
        benchmarks-check
        cargo-clippy-check
        cargo-deny-check
        check-picasso-integration-tests
        cmc-api
        cmc-api-image
        composable-node
        devnet-centauri
        composable-testfast-node
        picasso-testfast-runtime
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
