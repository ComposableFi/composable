{
  description = "Composable Finance";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    nixpkgs-latest.url =
      "github:NixOS/nixpkgs/0135b7a556ee60144b143b071724fa44348a188e";
    process-compose-flake = {
      url = "github:Platonic-Systems/process-compose-flake";
    };

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
        "github:dzmitry-lahoda-forks/zombienet/6d0b4cc3cca26e250f160d1979accc7e7318d347";
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
    cosmos 
    = {
      url =
        "github:dzmitry-lahoda-forks/cosmos.nix/16f6aaf252d36505ab3333f9e82389b5f6c78a39";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    centauri-src.flake = false;
    centauri-src.url =
      "github:ComposableFi/centauri/fd0a4911d86a531513547bbcc1f98df0a276fa79";
  };

  nixConfig = {
    extra-substituters = [
      "https://cache.nixos.org"
      "https://nix-community.cachix.org"
      "https://nixpkgs-update.cachix.org"
      "https://devenv.cachix.org"
      "https://composable-community.cachix.org"
      "https://cosmos.cachix.org"
    ];
    extra-trusted-public-keys = [
      "composable-community.cachix.org-1:GG4xJNpXJ+J97I8EyJ4qI5tRTAJ4i7h+NK2Z32I8sK8="
      "devenv.cachix.org-1:w1cLUi8dv3hnoSPGAuibQv+f9TZLr6cv/Hm9XgU50cw="
      "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
      "nixpkgs-update.cachix.org-1:6y6Z2JdoL3APdu6/+Iy8eZX2ajf09e4EE9SnxSML1W8="
      "cache.nixos.org-1:6NCHdD59X431o0gWypbMrAURkbJ16ZPMQFGspcDShjY="
      "cosmos.cachix.org-1:T5U9yg6u2kM48qAOXHO/ayhO8IWFnv0LOhNcq0yKuR8="
    ];
  };

  outputs = inputs@{ self, nixpkgs, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        inputs.process-compose-flake.flakeModule
        ./code/benchmarks.nix
        ./code/common-deps.nix
        ./code/composable-nodes.nix
        ./code/integration-tests/local-integration-tests/flake-module.nix
        ./code/integration-tests/runtime-tests/runtime-tests.nix
        ./code/runtimes.nix
        ./code/services/cmc-api/cmc-api.nix
        ./code/utils/price-feed/price-feed.nix
        ./code/xcvm/flake-module.nix
        ./docs/docs.nix
        ./flake/all.nix
        ./flake/cargo-tools.nix
        ./flake/check.nix
        ./flake/darwin-configurations.nix
        ./flake/dev-shells.nix
        ./flake/docker.nix
        ./flake/fmt.nix
        ./flake/ibc.nix
        ./flake/osmosis.nix
        ./flake/hermes.nix
        ./flake/devnet.nix
        ./flake/home-configurations.nix
        ./flake/live.nix
        ./flake/overlays.nix
        ./flake/process-compose.nix
        ./flake/release.nix
        ./flake/subxt.nix
        ./flake/zombienet.nix
        ./inputs/AcalaNetwork/acala.nix
        ./inputs/bifrost-finance/bifrost/flake-module.nix
        ./inputs/ComposableFi/centauri/flake-module.nix
        ./inputs/chevdor/subwasm.nix
        ./inputs/CosmosContracts/juno.nix
        ./inputs/notional-labs/composable-centauri/flake-module.nix
        ./inputs/osmosis-labs/beaker/flake-module.nix
        ./inputs/paritytech/cumulus.nix
        ./inputs/paritytech/polkadot.nix
        ./inputs/paritytech/substrate.nix
        ./inputs/paritytech/subxt.nix
        ./inputs/paritytech/zombienet/flake-module.nix
        ./inputs/Wasmswap/wasmswap-contracts.nix
        ./tools/devnet-tools.nix
        ./tools/pkgs.nix
        ./tools/rust.nix
      ];
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
    };
}
