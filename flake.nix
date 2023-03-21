{
  description = "Composable Finance";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # remove me when the `nixops_unstable` works again on the latest unstable
    nixpkgs-working-nixops.url =
      "github:NixOS/nixpkgs/34c5293a71ffdb2fe054eb5288adc1882c1eb0b1/";
    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    npm-buildpackage.url = "github:serokell/nix-npm-buildpackage";
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    arion-src = {
      url = "github:hercules-ci/arion";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    helix.url = "github:helix-editor/helix";
    bundlers = {
      url = "github:NixOS/bundlers";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    nix-std.url = "github:chessai/nix-std";
    devenv.url = "github:cachix/devenv";
    zombienet = {
      url =
        "github:dzmitry-lahoda-forks/zombienet/5122c59a33c9ec3eab60e3b6d5020732836f8c95";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-substituters = [ "https://composable-community.cachix.org/" ];
    extra-trusted-public-keys = [
      "composable-community.cachix.org-1:GG4xJNpXJ+J97I8EyJ4qI5tRTAJ4i7h+NK2Z32I8sK8="
    ];
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        # External `inputs` that the authors did not nixify themselves
        ./inputs/AcalaNetwork/acala.nix
        ./inputs/bifrost-finance/bifrost/flake-module.nix
        ./inputs/centauri/flake-module.nix
        ./inputs/chevdor/subwasm.nix
        ./inputs/cosmos/cosmwasm.nix
        ./inputs/cosmos/gex.nix
        ./inputs/CosmosContracts/juno.nix
        ./inputs/CosmWasm/wasmvm.nix
        ./inputs/paritytech/polkadot.nix
        ./inputs/paritytech/statemine.nix
        ./inputs/paritytech/substrate.nix
        ./inputs/paritytech/zombienet/flake-module.nix
        ./inputs/Wasmswap/wasmswap-contracts.nix

        # The things we use within flake parts to build packages, apps, devShells, and devnets.
        ./tools/cargo-tools.nix # _module.args.cargoTools
        ./tools/devnet-tools.nix # _module.args.devnetTools
        ./tools/pkgs.nix # _module.args.pkgs
        ./tools/rust.nix # _module.args.rust

        # our own packages
        ./code/benchmarks.nix
        ./code/common-deps.nix
        ./code/composable-nodes.nix
        ./code/integration-tests/local-integration-tests/flake-module.nix
        ./code/integration-tests/runtime-tests/runtime-tests.nix
        ./code/runtimes.nix
        ./code/services/cmc-api/cmc-api.nix
        ./code/utils/price-feed/price-feed.nix
        ./code/xcvm/xcvm-contracts.nix
        ./docs/docs.nix
        ./frontend/frontend.nix

        ./devnets/all.nix

        # Everything that is not an input, tool, package, or devnet, but still part of the final flake
        ./flake/all.nix
        ./flake/check.nix
        ./flake/dev-shells.nix
        ./flake/docker.nix
        ./flake/fmt.nix
        ./flake/help.nix
        ./flake/nixops-config.nix
        ./flake/overlays.nix
        ./flake/release.nix
        ./flake/subxt.nix
        ./flake/zombienet.nix
      ];
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
    };
}
