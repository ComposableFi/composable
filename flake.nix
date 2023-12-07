{
  description = "Composable Finance";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixpkgs-latest.url =
      "github:NixOS/nixpkgs/0135b7a556ee60144b143b071724fa44348a188e";
    process-compose-flake = {
      url = "github:Platonic-Systems/process-compose-flake";
    };

    sbt-derivation.url = "github:zaninime/sbt-derivation";
    # fixes wird behaviour when flake depends on this flake
    # it either fails to find overlay in sbt or fails to put nixpks into input
    sbt-derivation.inputs.nixpkgs.follows = "nixpkgs";
    sbt-derivation.inputs.flake-utils.follows = "flake-utils";

    process-compose = {
      url = "github:F1bonacc1/process-compose";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    flake-parts.url = "github:hercules-ci/flake-parts";
    flake-utils.url = "github:numtide/flake-utils";
    npm-buildpackage.url = "github:serokell/nix-npm-buildpackage";
    rust-overlay = { url = "github:oxalica/rust-overlay"; };
    crane = {
      url = "github:ipetkov/crane";
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
        "github:dzmitry-lahoda-forks/zombienet/a169bff1516c93114253ff6479956eeff66b0e2e";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    home-manager = {
      url = "github:nix-community/home-manager";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    darwin = {
      url = "github:lnl7/nix-darwin/master";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    cosmos = {
      url =
        "github:dzmitry-lahoda-forks/cosmos.nix/e398b4dc9fa8e44c9201d3285eb2818116c0b9d3";
      inputs.sbt-derivation.follows = "sbt-derivation";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
    };

    bech32cli = {
      url = "github:strangelove-ventures/bech32cli";
      inputs.nixpkgs.follows = "nixpkgs";
    };

    composable-ibc-relayer-src = {
      flake = false;
      url =
        "github:ComposableFi/composable-ibc/698146a5a66ce9e5e7a21633ef60e39fa1c8840e";
    };

    composable-ibc-light-client-src = {
      flake = false;
      url =
        "github:ComposableFi/composable-ibc/50eb36a8a1c4f67ae573ac447f6b1ba46f37791c";
    };

    composable-cosmos-src = {
      flake = false;
      url = "github:ComposableFi/composable-cosmos/devnet";
    };

    cvm = { url = "github:ComposableFi/cvm"; };

    networks = { url = "github:ComposableFi/networks"; };

    instrumental = {
      url =
        "github:InstrumentalFi/instrumental-contracts/61b3c81992178b7382308bfc3ecce04fff3de59c";
      inputs.cosmos.follows = "cosmos";
      inputs.crane.follows = "crane";
      inputs.flake-parts.follows = "flake-parts";
      inputs.flake-utils.follows = "flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.rust-overlay.follows = "rust-overlay";
    };

    eth-pos-devnet-src = {
      flake = false;
      url =
        "github:OffchainLabs/eth-pos-devnet/4f4c28e71fd39bc50788dc1b858c1cc6b983defb";
    };

    neutron-src.url = "github:neutron-org/neutron/v2.0.0";
    neutron-src.flake = false;

    ethereum = { url = "github:nix-community/ethereum.nix"; };

    polkadot = {
      url =
        "github:andresilva/polkadot.nix/30a616cb07b8f26b7bdb1b06d18440628bc6ecff";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  nixConfig = {
    extra-substituters = [
      "https://cache.nixos.org"
      "https://composable.cachix.org"
      "https://cosmos.cachix.org"
      "https://devenv.cachix.org"
      "https://nixpkgs-update.cachix.org"
    ];
    extra-trusted-public-keys = [
      "composable.cachix.org-1:J2TVJKH4U8xqYdN/0SpauoAxLuDYeheJtv22Vn3Hav8="
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "cosmos.cachix.org-1:T5U9yg6u2kM48qAOXHO/ayhO8IWFnv0LOhNcq0yKuR8="
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
      "nixpkgs-update.cachix.org-1:6y6Z2JdoL3APdu6/+Iy8eZX2ajf09e4EE9SnxSML1W8="
    ];
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.process-compose-flake.flakeModule
        ./code/benchmarks.nix
        ./code/common-deps.nix
        ./code/composable-nodes.nix
        ./code/integration-tests/runtime-tests/runtime-tests.nix
        ./code/runtimes.nix
        ./code/services/cmc-api/cmc-api.nix
        ./code/utils/price-feed/price-feed.nix
        ./docs/flake-module.nix
        ./flake/all.nix
        ./flake/bash.nix
        ./flake/cargo-tools.nix
        ./flake/check.nix
        ./flake/lightnet.nix
        ./flake/cosmos.nix
        ./flake/darwin-configurations.nix
        ./flake/dev-shells.nix
        ./flake/devnet-tools.nix
        ./flake/devnet.nix
        ./flake/ethereum.nix
        ./flake/fmt.nix
        ./flake/cosmos/hermes.nix
        ./flake/home-configurations.nix
        ./flake/ibc.nix
        ./flake/live.nix
        ./flake/cosmos/osmosis.nix
        ./flake/cosmos/cosmos-hub.nix
        ./flake/cosmos/neutron.nix
        ./flake/xapps.nix
        ./flake/xapps.nix
        ./flake/overlays.nix
        ./flake/devnets/flake-module.nix
        ./flake/release.nix
        ./flake/mantis.nix
        ./flake/rust.nix
        ./flake/subxt.nix
        ./flake/zombienet.nix
        ./inputs/AcalaNetwork/acala.nix
        ./inputs/bifrost-finance/bifrost/flake-module.nix
        ./inputs/ComposableFi/composable-ibc/flake-module.nix
        ./inputs/ComposableFi/composable-cosmos/flake-module.nix
        ./inputs/CosmWasm/flake-module.nix
        ./inputs/paritytech/cumulus.nix
        ./inputs/paritytech/polkadot.nix
        ./inputs/paritytech/substrate.nix
        ./inputs/paritytech/subxt.nix
        ./inputs/paritytech/zombienet/flake-module.nix
        ./inputs/Wasmswap/wasmswap-contracts.nix
        ./inputs/wynddao/flake-module.nix
      ];
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
    };
}
