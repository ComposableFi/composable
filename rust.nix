{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {

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
      rust-nightly = pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

      cargo-fmt-check = crane.nightly.cargoFmt (systemCommonRust.common-attrs // {
        cargoArtifacts = self'.packages.common-deps-nightly;
        cargoExtraArgs = "--all --check --verbose";
      });


      cargo-clippy-check = crane.nightly.cargoBuild (systemCommonRust.common-attrs // {
        cargoArtifacts = self'.packages.common-deps-nightly;
        cargoBuildCommand = "cargo clippy";
        cargoExtraArgs = "--all-targets --tests -- -D warnings";
      });

      cargo-deny-check = crane.nightly.cargoBuild (systemCommonRust.common-attrs // {
        buildInputs = with pkgs; [ cargo-deny ];
        cargoArtifacts = self'.packages.common-deps;
        cargoBuildCommand = "cargo deny";
        cargoExtraArgs =
          "--manifest-path ./parachain/frame/composable-support/Cargo.toml check ban";
      });

      cargo-udeps-check = crane.nightly.cargoBuild (systemCommonRust.common-attrs // {
        DALI_RUNTIME = "${self'.packages.dali-runtime}/lib/runtime.optimized.wasm";
        PICASSO_RUNTIME = "${self'.packages.picasso-runtime}/lib/runtime.optimized.wasm";
        COMPOSABLE_RUNTIME =
          "${self'.packages.composable-runtime}/lib/runtime.optimized.wasm";
        buildInputs = with pkgs; [
          cargo-udeps
          expat
          freetype
          fontconfig
          openssl
        ];
        cargoArtifacts = self'.packages.common-deps-nightly;
        cargoBuildCommand = "cargo udeps";
        cargoExtraArgs =
          "--workspace --exclude local-integration-tests --all-features";
      });

      benchmarks-check = crane.nightly.cargoBuild (systemCommonRust.common-attrs // {
        cargoArtifacts = self'.packages.common-deps-nightly;
        cargoBuildCommand = "cargo check";
        cargoExtraArgs = "--benches --all --features runtime-benchmarks";
      });

    };
  };
}
