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
        ./docs/docs.nix
        ./subsquid/subsquid.nix
        ./code/common-deps.nix
        ./code/runtimes.nix
        ./code/composable-nodes.nix
        ./code/integration-tests/runtime-tests/runtime-tests.nix
      ];
      systems = [ "x86_64-linux" "aarch64-linux" ];
      perSystem = { config, self', inputs', pkgs, system, ... }: {
        # Per-system attributes can be defined here. The self' and inputs'
        # module parameters provide easy access to attributes of the same
        # system.

        # Equivalent to  inputs'.nixpkgs.legacyPackages.hello;
        # packages.default = pkgs.hello;

        # Add the npm-buildpackage overlay to the perSystem's pkgs
        _module.args.pkgs = import self.inputs.nixpkgs {
          inherit system;
          overlays = with self.inputs; [
            npm-buildpackage.overlays.default
            rust-overlay.overlays.default
          ];
        };

        # System-specific lib to be used accross flake parts
        _module.args.systemLib = rec {

          # TODO: Find a way to define these in flake parts
          rust-stable = pkgs.rust-bin.stable.latest.default;
          rust-nightly =
            pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;
          # Crane lib instantiated with current nixpkgs
          crane-lib = self.inputs.crane.mkLib pkgs;

          # Crane pinned to stable Rust
          crane-stable = crane-lib.overrideToolchain rust-stable;

          # Crane pinned to nightly Rust
          crane-nightly = crane-lib.overrideToolchain rust-nightly;
        };
      };
      flake = {
        # The usual flake attributes can be defined here, including system-
        # agnostic ones like nixosModule and system-enumerating ones, although
        # those are more easily expressed in perSystem.

      };
    };
}
