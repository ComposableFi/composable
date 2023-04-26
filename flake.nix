{
  description = "Composable Finance";
  inputs = {
    # basically this is old release locked, better not to updated as long as possible as it can break anything
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    # needed as some tools of old version on stable releases, so can start using new tooling until full switch to release
    nixpkgs-latest.url =
      "github:NixOS/nixpkgs/0135b7a556ee60144b143b071724fa44348a188e";
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
        "github:dzmitry-lahoda-forks/zombienet/4d2eff2fd5a165aceb1fd11b218482710bd35d77";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # for mac builder
    darwin = {
      url = "github:lnl7/nix-darwin/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-substituters = [
      "https://cache.nixos.org"
      "https://nix-community.cachix.org"
      "https://nixpkgs-update.cachix.org"
      "https://devenv.cachix.org"
      "https://composable-community.cachix.org"
    ];
    extra-trusted-public-keys = [
      "composable-community.cachix.org-1:GG4xJNpXJ+J97I8EyJ4qI5tRTAJ4i7h+NK2Z32I8sK8="
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "nixpkgs-update.cachix.org-1:6y6Z2JdoL3APdu6/+Iy8eZX2ajf09e4EE9SnxSML1W8="
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
    ];
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./code/benchmarks.nix
        ./code/common-deps.nix
        ./code/composable-nodes.nix
        ./code/integration-tests/local-integration-tests/flake-module.nix
        ./code/integration-tests/runtime-tests/runtime-tests.nix
        ./code/runtimes.nix
        ./code/services/cmc-api/cmc-api.nix
        ./code/utils/price-feed/price-feed.nix
        ./code/xcvm/xcvm-contracts.nix
        ./devnets/all.nix
        ./docs/docs.nix
        ./flake/all.nix
        ./flake/check.nix
        ./flake/dev-shells.nix
        ./flake/docker.nix
        ./flake/fmt.nix
        ./flake/help.nix
        ./flake/home-configurations.nix
        ./flake/darwin-configurations.nix
        ./flake/overlays.nix
        ./flake/release.nix
        ./flake/subxt.nix
        ./flake/live.nix
        ./flake/zombienet.nix
        ./frontend/frontend.nix
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
        ./tools/cargo-tools.nix # _module.args.cargoTools
        ./tools/devnet-tools.nix # _module.args.devnetTools
        ./tools/pkgs.nix # _module.args.pkgs
        ./tools/rust.nix # _module.args.rust
      ];
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
    };
}
