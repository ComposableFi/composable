{
  description = "Composable Finance systems, tools and releases";
  inputs = {
    bundlers = {
      url = "github:NixOS/bundlers";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, npm-buildpackage
    , arion-src, home-manager, helix, bundlers }:
    let
      # https://cloud.google.com/iam/docs/creating-managing-service-account-keys
      # or just use GOOGLE_APPLICATION_CREDENTIALS env as path to file
      service-account-credential-key-file-input = builtins.fromJSON
        (builtins.readFile (builtins.getEnv "GOOGLE_APPLICATION_CREDENTIALS"));

      eachSystemOutputs = flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            allowUnsupportedSystem = true; # we do not trigger this on mac
            config = {
              permittedInsecurePackages = [
                "openjdk-headless-16+36"
                "openjdk-headless-15.0.1-ga"
                "openjdk-headless-14.0.2-ga"
                "openjdk-headless-13.0.2-ga"
              ];
            };
          };

          benchmarks-run-once = chainspec:
            pkgs.writeShellScriptBin "run-benchmarks-once" ''
              ${composable-bench-node}/bin/composable benchmark pallet \
              --chain="${chainspec}" \
              --execution=wasm \
              --wasm-execution=compiled \
              --wasm-instantiation-strategy=legacy-instance-reuse \
              --pallet="*" \
              --extrinsic="*" \
              --steps=1 \
              --repeat=1
            '';

          generate-benchmarks = { chain, steps, repeat }:
            pkgs.writeShellScriptBin "generate-benchmarks" ''
              ${composable-bench-node}/bin/composable benchmark pallet \
              --chain="${chain}-dev" \
              --execution=wasm \
              --wasm-execution=compiled \
              --wasm-instantiation-strategy=legacy-instance-reuse \
              --pallet="*" \
              --extrinsic="*" \
              --steps=${builtins.toString steps} \
              --repeat=${builtins.toString repeat} \
              --output=code/parachain/runtime/${chain}/src/weights
            '';
          mk-xcvm-contract = name:
            crane-nightly.buildPackage (common-attrs // {
              pnameSuffix = name;
              cargoBuildCommand =
                "cargo build --target wasm32-unknown-unknown --profile cosmwasm-contracts -p ${name}";
              RUSTFLAGS = "-C link-arg=-s";
            });

        in rec {
          packages = rec {
            inherit simnode-tests;
            generated-release-body = let
              subwasm-call = runtime:
                builtins.readFile (pkgs.runCommand "subwasm-info" { } ''
                  ${subwasm}/bin/subwasm info ${runtime}/lib/runtime.optimized.wasm | tail -n+2 | head -c -1 > $out
                '');
              flake-url =
                "github:ComposableFi/composable/v${composable-node.version}";
            in pkgs.writeTextFile {
              name = "release.txt";
              text = ''
                ## Runtimes
                ### Dali
                ```
                ${subwasm-call dali-runtime}```
                ### Picasso
                ```
                ${subwasm-call picasso-runtime}
                ```
                ### Composable
                ```
                ${subwasm-call composable-runtime}
                ```
                ## Nix
                ```bash
                # Generate the Wasm runtimes
                nix build ${flake-url}#dali-runtime
                nix build ${flake-url}#picasso-runtime
                nix build ${flake-url}#composable-runtime

                # Run the Composable node (release mode) alone
                nix run ${flake-url}#composable-node-release

                # Spin up a local devnet
                nix run ${flake-url}#devnet

                # Spin up a local XCVM devnet
                nix run ${flake-url}#devnet-xcvm

                # Show all possible apps, shells and packages
                nix flake show ${flake-url} --allow-import-from-derivation
                ```
              '';
            };

            generate-release-artifacts = pkgs.writeShellApplication {
              name = "generate-release-artifacts";
              text = let
                make-bundle = type: package:
                  bundlers.bundlers."${system}"."${type}" package;
                subwasm-version = runtime:
                  builtins.readFile (pkgs.runCommand "subwasm-version" { } ''
                    ${subwasm}/bin/subwasm version ${runtime}/lib/runtime.optimized.wasm | grep specifications | cut -d ":" -f2 | cut -d " " -f3 | head -c -1 > $out
                  '');
              in ''
                mkdir -p release-artifacts/to-upload/

                # Generate release body
                cp ${generated-release-body} release-artifacts/release.txt

                # Generate wasm runtimes
                cp ${dali-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/dali_runtime_${
                  subwasm-version dali-runtime
                }.wasm
                cp ${picasso-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/picasso_runtime_${
                  subwasm-version picasso-runtime
                }.wasm
                cp ${composable-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/composable_runtime_${
                  subwasm-version composable-runtime
                }.wasm

                # Generate packaged binaries
                # RPM Name convention: https://docs.oracle.com/en/database/oracle/oracle-database/19/ladbi/rpm-packages-naming-convention.html
                cp ${
                  make-bundle "toRPM" composable-node-release
                }/*.rpm release-artifacts/to-upload/composable-node-${composable-node-release.version}-1.x86_64.rpm
                # DEB Name convention: https://askubuntu.com/questions/330018/what-is-the-standard-for-naming-deb-file-name
                cp ${
                  make-bundle "toDEB" composable-node-release
                }/*.deb release-artifacts/to-upload/composable-node_${composable-node-release.version}-1_amd64.deb
                cp ${
                  make-bundle "toDockerImage" composable-node-release
                } release-artifacts/composable-docker-image

                # Checksum everything
                cd release-artifacts/to-upload
                sha256sum ./* > checksums.txt
              '';
            };
            xcvm-contract-asset-registry =
              mk-xcvm-contract "xcvm-asset-registry";
            xcvm-contract-router = mk-xcvm-contract "xcvm-router";
            xcvm-contract-interpreter = mk-xcvm-contract "xcvm-interpreter";
            # TODO: inherit and provide script to run all stuff


            check-dali-dev-benchmarks = benchmarks-run-once "dali-dev";
            check-picasso-dev-benchmarks = benchmarks-run-once "picasso-dev";
            check-composable-dev-benchmarks =
              benchmarks-run-once "composable-dev";
           cargo-llvm-cov = pkgs.rustPlatform.buildRustPackage rec {
              pname = "cargo-llvm-cov";
              version = "0.3.3";
              src = pkgs.fetchFromGitHub {
                owner = "andor0";
                repo = pname;
                rev = "v${version}";
                sha256 = "sha256-e2MQWOCIj0GKeyOI6OfLnXkxUWbu85eX4Smc/A6eY2w";
              };
              cargoSha256 =
                "sha256-1fxqIQr8hol2QEKz8IZfndIsSTjP2ACdnBpwyjG4UT0=";
              doCheck = false;
              meta = {
                description =
                  "Cargo subcommand to easily use LLVM source-based code coverage";
                homepage = "https://github.com/taiki-e/cargo-llvm-cov";
                license = "Apache-2.0 OR MIT";
              };
            };
            default = packages.composable-node;
          };


          apps = let
            makeApp = p: {
              type = "app";
              program = pkgs.lib.meta.getExe p;
            };
          in rec {
            # TODO: move list of chains out of here and do fold
            benchmarks-once-composable = flake-utils.lib.mkApp {
              drv = benchmarks-run-once "composable-dev";
            };
            benchmarks-once-dali =
              flake-utils.lib.mkApp { drv = benchmarks-run-once "dali-dev"; };
            benchmarks-once-picasso = flake-utils.lib.mkApp {
              drv = benchmarks-run-once "picasso-dev";
            };
            benchmarks-generate-dali = flake-utils.lib.mkApp {
              drv = generate-benchmarks {
                chain = "dali";
                steps = 50;
                repeat = 10;
              };sNixFile = name: type:
                    type == "regular" && pkgs.lib.strings.hasSuffix ".nix" name;
                  customFilter = name: type:
                    !((isBlacklisted name type) || (isImageFile name type)
                      || (isPlantUmlFile name type)
                      # assumption that nix is final builder,
                      # so there would no be sandwich like  .*.nix <- build.rs <- *.nix
                      # and if *.nix changed, nix itself will detect only relevant cache invalidations
                      || (isNixFile name type));
                in
                pkgs.nix-gitignore.gitignoreFilterPure customFilter
                  [ ../.gitigno
            };
            benchmarks-generate-picasso = flake-utils.lib.mkApp {
              drv = generate-benchmarks {
                chain = "picasso";
                steps = 50;
                repeat = 10;
              };
            };
            benchmarks-generate-composable = flake-utils.lib.mkApp {
              drv = generate-benchmarks {
                chain = "composable";
                steps = 50;
                repeat = 10;
              };
            };
            benchmarks-generate-quick-dali = flake-utils.lib.mkApp {
              drv = generate-benchmarks {
                chain = "dali";
                steps = 2;
                repeat = 2;
              };
            };
            benchmarks-generate-quick-picasso = flake-utils.lib.mkApp {
              drv = generate-benchmarks {
                chain = "picasso";
                steps = 2;
                repeat = 2;
              };
            };
            benchmarks-generate-quick-composable = flake-utils.lib.mkApp {
              drv = generate-benchmarks {
                chain = "composable";
                steps = 2;
                repeat = 2;
              };
            };sNixFile = name: type:
                    type == "regular" && pkgs.lib.strings.hasSuffix ".nix" name;
                  customFilter = name: type:
                    !((isBlacklisted name type) || (isImageFile name type)
                      || (isPlantUmlFile name type)
                      # assumption that nix is final builder,
                      # so there would no be sandwich like  .*.nix <- build.rs <- *.nix
                      # and if *.nix changed, nix itself will detect only relevant cache invalidations
                      || (isNixFile name type));
                in
                pkgs.nix-gitignore.gitignoreFilterPure customFilter
                  [ ../.gitigno
          };
        });
    in eachSystemOutputs // {};
}
