{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {

      all-deps = pkgs.linkFarmFromDrvs "all-deps" (with self'.packages; [
        acala-node
        bifrost-node
        polkadot-node-from-dep
        rococo-runtime-from-dep
        statemine-node
        subwasm
        zombienet
      ]);

      all-testnet-deps = pkgs.linkFarmFromDrvs "all-testnet-deps"
        (with self'.packages; [
          polkadot-node-on-parity-rococo
          polkadot-node-on-parity-westend
          polkadot-runtime-on-parity
          rococo-runtime-on-parity
          westend-runtime-on-parity
        ]);

      all-production-deps = pkgs.linkFarmFromDrvs "all-production-deps"
        (with self'.packages; [
          kusama-runtime-on-parity
          polkadot-node-on-parity-kusama
          polkadot-node-on-parity-polkadot
          polkadot-runtime-on-parity
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
        benchmarks-check
      ]);

      all-platforms = pkgs.linkFarmFromDrvs "all-platforms"
        (with self'.packages; [
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
          hyperspace-composable-rococo-picasso-rococo
          hyperspace-composable-rococo-picasso-rococo-image
        ]);

      all-ci-packages = pkgs.linkFarmFromDrvs "all-ci-packages"
        (with self'.packages; [
          all-platforms
          cargo-clippy-check
          cargo-deny-check
          check-picasso-integration-tests
          unit-tests
        ]);

      all-frontend = pkgs.linkFarmFromDrvs "all-frontend"
        (with self'.packages; [ frontend-static ]);

      docker-images-to-push = pkgs.linkFarmFromDrvs "docker-images-to-push"
        (with self'.packages; [
          cmc-api-image
          hyperspace-composable-rococo-picasso-rococo-image
          hyperspace-composable-polkadot-picasso-kusama-image
          devnet-picasso-image
        ]);
    };
  };
}
