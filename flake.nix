{
  description = "Composable Finance";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-parts.url = "github:hercules-ci/flake-parts";
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
    helix = {
      url = "github:helix-editor/helix";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = { self, flake-parts, ... }:
    flake-parts.lib.mkFlake { inherit self; } {
      imports = [
        # To import a flake module
        # 1. Add foo to inputs
        # 2. Add foo as a parameter to the outputs function
        # 3. Add here: foo.flakeModule
        ./fmt.nix
        ./docker.nix
        ./subwasm.nix
        ./dev-shells.nix
        ./docs/docs.nix
        ./subsquid/subsquid.nix
        ./code/common-deps.nix
        ./code/runtimes.nix
        ./code/composable-nodes.nix
        ./code/integration-tests/runtime-tests/runtime-tests.nix
      ];
      systems = [ "x86_64-linux" "aarch64-linux" ];
      perSystem = { config, self', inputs', pkgs, system, crane, ... }: {
        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.

        # Equivalent to  inputs'.nixpkgs.legacyPackages.hello;
        # packages.default = pkgs.hello;

        # Add the npm-buildpackage overlay to the perSystem's pkgs
        _module.args.pkgs = import self.inputs.nixpkgs {
          inherit system;
          overlays = with self.inputs; [
            self.overlays.default
            npm-buildpackage.overlays.default
            rust-overlay.overlays.default
          ];
        };

        # System-specific lib to be used accross flake parts
        _module.args.crane = rec {

          # Crane lib instantiated with current nixpkgs
          # Crane pinned to stable Rust
          lib = self.inputs.crane.mkLib pkgs;

          stable = lib.overrideToolchain self'.packages.rust-stable;

          # Crane pinned to nightly Rust
          nightly = lib.overrideToolchain self'.packages.rust-nightly;
        };

        packages = {
          rust-stable = pkgs.rust-bin.stable.latest.default;
          rust-nightly =
            pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          subxt = pkgs.callPackage ./code/utils/composable-subxt/subxt.nix { };
          junod = pkgs.callPackage ./code/xcvm/cosmos/junod.nix { };
          gex = pkgs.callPackage ./code/xcvm/cosmos/gex.nix { };
          wasmswap = pkgs.callPackage ./code/xcvm/cosmos/wasmswap.nix {
            crane = crane.nightly;
          };

          # NOTE: crane can't be used because of how it vendors deps, which is incompatible with some packages in polkadot, an issue must be raised to the repo
          acala-node = pkgs.callPackage ./.nix/acala-bin.nix {
            rust-overlay= self'.packages.rust-nightly;
          };

          polkadot-node = pkgs.callPackage ./.nix/polkadot/polkadot-bin.nix {
            rust-nightly = self'.packages.rust-nightly;
          };

          statemine-node = pkgs.callPackage ./.nix/statemine-bin.nix {
            rust-nightly = self'.packages.rust-nightly;
          };

          mmr-polkadot-node =
            pkgs.callPackage ./.nix/polkadot/mmr-polkadot-bin.nix {
              rust-nightly = self'.packages.rust-nightly;
            };

          polkadot-launch =
            pkgs.callPackage ./scripts/polkadot-launch/polkadot-launch.nix
              { };

        };
      };
      flake = {
        overlays = {
          default =
            let
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
            in
            self.inputs.nixpkgs.lib.composeManyExtensions [
              self.inputs.arion-src.overlay
              (final: _prev: {
                composable = {
                  mkDevnetProgram = final.callPackage mkDevnetProgram { };
                };
              })
            ];
        };
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.

      };
    };
}
