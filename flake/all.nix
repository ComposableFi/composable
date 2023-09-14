{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {

      all-deps = pkgs.linkFarmFromDrvs "all-deps" (with self'.packages; [
        acala-node
        bifrost-node
        polkadot-node-from-dep
        rococo-runtime-from-dep
        polkadot-parachain
        subwasm
        zombienet
        subxt
        bech32cli
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
        nixfmt-check
        deadnix-check
        taplo-check
        cargo-deny-check
        cargo-no-std-core-check
        cargo-no-std-cosmwasm
        cargo-no-std-xcm-ibc
      ]);

      all-benchmarks = pkgs.linkFarmFromDrvs "all-benchmarks"
        (with self'.packages; [
          check-composable-benchmarks-ci
          check-picasso-benchmarks-ci
          composable-bench-node
          benchmarks-check
        ]);

      all-rust-qa-packages = pkgs.linkFarmFromDrvs "all-rust-qa-packages"
        (with self'.packages; [ cargo-clippy-check unit-tests ]);

      all-production = pkgs.linkFarmFromDrvs "all-production"
        (with self'.packages; [ livenet-composable ]);

      all-darwin = pkgs.linkFarmFromDrvs "all-darwin"
        (with self'.packages; [ devnet-picasso ccw ]);

      all-run-packages = pkgs.linkFarmFromDrvs "all-run-packages"
        (with self'.packages; [
          ccw
          cmc-api
          cmc-api-image
          composable-node
          composable-testfast-node
          composable-testfast-runtime
          devnet-integration-tests
          devnet-picasso
          devnet-picasso-image
          devnet-xc-image
          devnet-xc-fresh
          hyperspace-composable-rococo-picasso-rococo
          hyperspace-composable-rococo-picasso-rococo-image
          picasso-testfast-runtime
        ]);

      all-ci-packages = pkgs.linkFarmFromDrvs "all-ci-packages"
        (with self'.packages; [ all-run-packages ]);

      docker-images-to-push = pkgs.linkFarmFromDrvs "docker-images-to-push"
        (with self'.packages; [
          cmc-api-image
          hyperspace-composable-rococo-picasso-rococo-image
          hyperspace-composable-polkadot-picasso-kusama-image
          devnet-picasso-image
          devnet-image
          devnet-xc-image
        ]);
    };
  };
}
