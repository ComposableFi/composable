{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {

      # System-specific lib to be used across flake parts
      _module.args.crane = rec {

        # Crane lib instantiated with current nixpkgs
        # Crane pinned to stable Rust
        lib = self.inputs.crane.mkLib pkgs;

        stable = lib.overrideToolchain self'.packages.rust-stable;

        # Crane pinned to nightly Rust
        nightly = lib.overrideToolchain self'.packages.rust-nightly;
      };

      packages = {

        cargo-llvm-cov = pkgs.rustPlatform.buildRustPackage rec {
          pname = "cargo-llvm-cov";
          version = "0.3.3";
          src = pkgs.fetchFromGitHub {
            owner = "andor0";
            repo = pname;
            rev = "v${version}";
            sha256 = "sha256-e2MQWOCIj0GKeyOI6OfLnXkxUWbu85eX4Smc/A6eY2w";
          };
          cargoSha256 = "sha256-1fxqIQr8hol2QEKz8IZfndIsSTjP2ACdnBpwyjG4UT0=";
          doCheck = false;
          meta = {
            description =
              "Cargo subcommand to easily use LLVM source-based code coverage";
            homepage = "https://github.com/taiki-e/cargo-llvm-cov";
            license = "Apache-2.0 OR MIT";
          };
        };

        rust-stable = pkgs.rust-bin.stable.latest.default;
        rust-nightly =
          pkgs.rust-bin.fromRustupToolchainFile ../rust-toolchain.toml;

        cargo-fmt-check = crane.nightly.cargoFmt (systemCommonRust.common-attrs
          // {
            cargoArtifacts = self'.packages.common-deps-nightly;
            cargoExtraArgs = "--all --check --verbose";
          });

        cargo-clippy-check = crane.nightly.cargoClippy
          (systemCommonRust.common-attrs // {
            cargoArtifacts = self'.packages.common-deps-nightly;
            cargoClippyExtraArgs = "--all-targets --tests -- -D warnings -A deprecated";
          });

        cargo-deny-check = crane.nightly.mkCargoDerivation {
          buildInputs = with pkgs; [ cargo-deny ];
          src = ../code;
          cargoArtifacts = self'.packages.common-deps;
          buildPhaseCargoCommand =
            "cargo-deny --manifest-path ./parachain/frame/composable-support/Cargo.toml check bans";
        };

        cargo-udeps-check = crane.nightly.cargoBuild
          (systemCommonRust.common-attrs // {
            DALI_RUNTIME =
              "${self'.packages.dali-runtime}/lib/runtime.optimized.wasm";
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
              "cargo test --workspace --release --locked --verbose --exclude local-integration-tests";
            installPhase = "mkdir -p $out";
          });

        unit-tests-with-coverage = crane.nightly.cargoBuild
          (systemCommonRust.common-attrs // {
            pnameSuffix = "-tests-with-coverage";
            buildInputs = [ self'.packages.cargo-llvm-cov ];
            cargoArtifacts = self'.packages.common-deps-nightly;
            # NOTE: do not add --features=runtime-benchmarks because it force multi ED to be 0 because of dependencies
            # NOTE: in order to run benchmarks as tests, just make `any(test, feature = "runtime-benchmarks")
            buildPhase = "cargo llvm-cov";
            cargoExtraArgs =
              "--workspace --release --locked --verbose --lcov --output-path lcov.info";
            installPhase = ''
              mkdir -p $out/lcov
              mv lcov.info $out/lcov
            '';
          });

      };
    };
}
