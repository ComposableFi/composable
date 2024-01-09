{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {

      all-deps = pkgs.linkFarmFromDrvs "all-deps" (with self'.packages; [
        all-deps-shell
        kusama-runtime-on-parity
        polkadot-parachain
        polkadot-runtime-on-parity
        rococo-runtime-from-dep
        common-deps
        common-deps-nightly
        common-std-bench-deps
        common-wasm-bench-deps
        common-test-deps
      ]);

      all-checks = pkgs.linkFarmFromDrvs "all-checks" (with pkgs;
        with self'.packages; [
          # no-std benchmark build broken because of composavble-ibc deps, like pallet-ibc
          # benchmarks-check
          # check-composable-benchmarks-ci
          # check-picasso-benchmarks-ci
          all-outputs
          cargo-clippy-check
          cargo-deny-check
          cargo-fmt-check
          cargo-no-std-core-check
          cargo-no-std-cosmwasm
          cargo-no-std-xcm-ibc
          composable-bench-node
          deadnix-check
          hyperspace-composable-rococo-picasso-rococo
          mantis-e2e
          picasso-testfast-runtime
        ]);

      all-outputs = pkgs.linkFarmFromDrvs "all-outputs" (with pkgs;
        with self'.packages; [
          all-deps-shell
          docs-server
          docs-static
          cmc-api
          cmc-api-image
          composable-node
          composable-testfast-node
          composable-testfast-runtime
          devnet-image
          devnet-picasso
          devnet-picasso-image
          devnet-xc-fresh
          devnet-xc-image
          devnet-cosmos
          hyperspace-composable-rococo-picasso-rococo
        ]);

      all = pkgs.linkFarmFromDrvs "all" (with pkgs;
        with self'.packages; [
          all-deps-shell
          all-checks
          all-outputs
        ]);
    };
  };
}
