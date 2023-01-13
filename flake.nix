# We use https://flake.parts/ in order split this flake into multiple parts.
{
  description = "Composable Finance";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
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
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }:
    let darwinFilter = import ./flake/darwin-filter.nix { lib = nixpkgs.lib; };
    in darwinFilter (flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        # External `inputs` that the authors did not nixify themselves
        ./inputs/AcalaNetwork/acala.nix
        ./inputs/chevdor/subwasm.nix
        ./inputs/cosmos/cosmwasm.nix
        ./inputs/cosmos/gex.nix
        ./inputs/CosmosContracts/juno.nix
        ./inputs/CosmWasm/wasmvm.nix
        ./inputs/paritytech/statemine.nix
        ./inputs/paritytech/polkadot.nix
        ./inputs/paritytech/polkadot-launch.nix
        ./inputs/paritytech/zombienet.nix
        ./inputs/Wasmswap/wasmswap-contracts.nix

        # The things we use within flake parts to build packages, apps, devShells, and devnets. 
        ./tools/pkgs.nix # _module.args.pkgs
        ./tools/devnet-tools.nix # _module.args.devnetTools
        ./tools/rust.nix # _module.args.rust
        ./tools/cargo-tools.nix # _module.args.cargoTools

        # our own packages
        ./code/services/cmc-api/cmc-api.nix
        ./code/benchmarks.nix
        ./code/common-deps.nix
        ./code/composable-nodes.nix
        ./code/integration-tests/local-integration-tests/flake-module.nix
        ./code/simnode-tests.nix
        ./code/integration-tests/runtime-tests/runtime-tests.nix
        ./code/runtimes.nix
        ./code/xcvm/xcvm-contracts.nix
        ./code/utils/composable-subxt/subxt.nix
        ./code/utils/price-feed/price-feed.nix
        ./docs/docs.nix
        ./frontend/frontend.nix

        # our devnets
        # TODO: Split into multiple files
        ./devnets/all.nix

        # Everything that is not an input, tool, package, or devnet, but still part of the final flake
        ./flake/check.nix
        ./flake/dev-shells.nix
        ./flake/docker.nix
        ./flake/fmt.nix
        ./flake/nixops-config.nix
        ./flake/overlays.nix
        ./flake/release.nix
        ./inputs/bifrost-finance/bifrost/flake-module.nix
      ];
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
    });
}
