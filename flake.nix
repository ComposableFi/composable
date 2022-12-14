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
    let darwinFilter = import ./darwin-filter.nix { lib = nixpkgs.lib; };
    in darwinFilter (flake-parts.lib.mkFlake { inherit inputs; } {
      imports = [
        ./code/services/cmc-api/cmc-api.nix
        ./code/benchmarks.nix
        ./code/common-deps.nix
        ./code/composable-nodes.nix
        ./code/integration-tests/local-integration-tests/flake-module.nix
        ./code/simnode-tests.nix
        ./code/integration-tests/runtime-tests/runtime-tests.nix
        ./code/runtimes.nix
        ./code/xcvm/xcvm-contracts.nix
        ./dev-shells.nix
        ./devnet-tools.nix
        ./devnets.nix
        ./docker.nix
        ./code/xcvm/cosmos/flake-module.nix
        ./docs/docs.nix
        ./fmt.nix
        ./frontend/frontend.nix
        ./nixops-config.nix
        ./price-feed.nix
        ./release.nix
        ./rust.nix
        ./subwasm.nix
        ./scripts/zombienet/flake-module.nix
        ./.nix/cargo/flake-module.nix
        ./code/utils/composable-subxt/subxt.nix
        ./code/xcvm/cosmos/junod.nix
        ./code/xcvm/cosmos/gex.nix
        ./code/xcvm/cosmos/wasmswap.nix
        ./parachains/acala.nix
        ./parachains/statemine.nix
        ./parachains/polkadot.nix
        ./parachains/polkadot-launch.nix
      ];
      systems =
        [ "x86_64-linux" "aarch64-linux" "aarch64-darwin" "x86_64-darwin" ];
      perSystem = { config, self', inputs', pkgs, system, crane, ... }: {
        _module.args.pkgs = import self.inputs.nixpkgs {
          inherit system;
          overlays = with self.inputs; [
            self.overlays.default
            npm-buildpackage.overlays.default
            rust-overlay.overlays.default
          ];
        };
        packages = {
          # TODO: remove this from here
          default = self'.packages.zombienet-rococo-local-dali-dev;
          devnet-dali = self'.packages.zombienet-rococo-local-dali-dev;
          # NOTE: Do not add packages here directly, instead, put them in flake-parts.
        };
      };
      flake = {
        # NOTE: These will bue put in a part soon.
        overlays = {
          default = let
            mkDevnetProgram = { pkgs }:
              name: spec:
              pkgs.writeShellApplication {
                inherit name;
                runtimeInputs =
                  [ pkgs.arion pkgs.docker pkgs.coreutils pkgs.bash ];
                text = ''
                  arion --prebuilt-file ${
                    pkgs.arion.build spec
                  } up --build --force-recreate -V --always-recreate-deps --remove-orphans
                '';
              };
          in inputs.nixpkgs.lib.composeManyExtensions [
            inputs.arion-src.overlays.default
            (final: _prev: {
              composable = {
                mkDevnetProgram = final.callPackage mkDevnetProgram { };
              };
            })
          ];
        };
      };
    });
}
