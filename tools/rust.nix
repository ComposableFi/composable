{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      rust-toolchain =
        pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;
      cargo-no-std-check = pname:
        crane.nightly.cargoBuild (systemCommonRust.common-attrs // {
          cargoArtifacts = self'.packages.common-deps-nightly;
          buildPhase = ''
            cargo check --no-default-features --target wasm32-unknown-unknown --package ${pname} 
            cargo clippy --package ${pname} -- --deny warnings --allow deprecated
          '';
          installPhase = "mkdir --parents $out";
        });

    in {
      _module.args.crane = rec {
        lib = self.inputs.crane.mkLib pkgs;
        stable = lib.overrideToolchain self'.packages.rust-stable;
        nightly = lib.overrideToolchain rust-toolchain;
      };

      packages = {
        rust-stable = pkgs.rust-bin.stable.latest.default;
        rust-nightly = rust-toolchain;

        cargo-fmt-check = crane.nightly.cargoFmt (systemCommonRust.common-attrs
          // {
            cargoArtifacts = self'.packages.common-deps-nightly;
            cargoExtraArgs = "--all --check --verbose";
          });

        cargo-clippy-check = crane.nightly.cargoClippy
          (systemCommonRust.common-attrs // {
            cargoArtifacts = self'.packages.common-deps-nightly;
            cargoClippyExtraArgs =
              "--all-targets --tests -- --deny warnings --allow deprecated";
          });

        cargo-deny-check = crane.nightly.mkCargoDerivation {
          buildInputs = with pkgs; [ cargo-deny ];
          src = ../code;
          cargoArtifacts = self'.packages.common-deps;
          buildPhaseCargoCommand =
            "cargo-deny --manifest-path ./parachain/frame/composable-support/Cargo.toml check bans";
        };

        cargo-no-std-core-check = cargo-no-std-check "composable-traits";
        cargo-no-std-cosmwasm = cargo-no-std-check "pallet-cosmwasm";
        cargo-no-std-xcm-ibc = cargo-no-std-check "pallet-multihop-xcm-ibc";

        cargo-udeps-check = crane.nightly.cargoBuild
          (systemCommonRust.common-attrs // {
            PICASSO_RUNTIME =
              "${self'.packages.picasso-runtime}/lib/runtime.optimized.wasm";
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
            buildPhase = "cargo udeps";
            installPhase = "mkdir -p $out";
            cargoExtraArgs =
              "--workspace --exclude local-integration-tests --all-features";
          });

        benchmarks-check = crane.nightly.cargoBuild
          (systemCommonRust.common-attrs // {
            cargoArtifacts = self'.packages.common-deps-nightly;
            cargoBuildCommand = "cargo check";
            cargoExtraArgs = "--benches --all --features runtime-benchmarks";
          });

        unit-tests = crane.nightly.cargoBuild (systemCommonRust.common-attrs
          // {
            pnameSuffix = "-tests";
            doInstallCargoArtifacts = false;
            cargoArtifacts = self'.packages.common-test-deps;
            # NOTE: do not add --features=runtime-benchmarks because it force multi ED to be 0 because of dependencies
            # NOTE: in order to run benchmarks as tests, just make `any(test, feature = "runtime-benchmarks")
            buildPhase =
              "cargo test --workspace --release --locked --verbose --exclude local-integration-tests --exclude xc-tests";
            installPhase = "mkdir -p $out";
          });
      };
    };
}
