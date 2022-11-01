{
  # see ./docs/nix.md for design guidelines of nix organization
  description = "Composable Finance systems, tools and releases";
  # when flake runs, ask for interactive answers first time
  # nixConfig.sandbox = "relaxed";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils = { url = "github:numtide/flake-utils"; };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
      inputs.flake-utils.follows = "flake-utils";
    };
    npm-buildpackage = {
      url = "github:serokell/nix-npm-buildpackage";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    home-manager = {
      url = "github:nix-community/home-manager";
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

      gce-to-nix = { project_id, client_email, private_key, ... }: {
        project = project_id;
        serviceAccount = client_email;
        accessKey = private_key;
      };

      gce-input = gce-to-nix service-account-credential-key-file-input;


      mk-devnet = { pkgs, lib, writeTextFile, writeShellApplication
        , useGlobalChainSpec ? true, polkadot-launch, composable-node
        , polkadot-node, chain-spec, network-config-path ?
          ./scripts/polkadot-launch/rococo-local-dali-dev.nix }:
        let
          original-config = (pkgs.callPackage network-config-path {
            polkadot-bin = polkadot-node;
            composable-bin = composable-node;
          }).result;

          patched-config = if useGlobalChainSpec then
            pkgs.lib.recursiveUpdate original-config {
              parachains = builtins.map
                (parachain: parachain // { chain = "${chain-spec}"; })
                original-config.parachains;
            }
          else
            original-config;

          config = pkgs.writeTextFile {
            name = "devnet-${chain-spec}-config.json";
            text = builtins.toJSON patched-config;
          };
        in {
          inherit chain-spec;
          parachain-nodes = builtins.concatMap (parachain: parachain.nodes)
            patched-config.parachains;
          relaychain-nodes = patched-config.relaychain.nodes;
          script = pkgs.writeShellApplication {
            name = "run-devnet-${chain-spec}";
            text = ''
              rm -rf /tmp/polkadot-launch
              ${polkadot-launch}/bin/polkadot-launch ${config} --verbose
            '';
          };
        };

      mk-bridge-devnet =
        { pkgs, packages, polkadot-launch, composable-node, polkadot-node }:
        (pkgs.callPackage mk-devnet {
          inherit pkgs;
          inherit (packages) polkadot-launch composable-node polkadot-node;
          chain-spec = "dali-dev";
          network-config-path =
            ./scripts/polkadot-launch/bridge-rococo-local-dali-dev.nix;
          useGlobalChainSpec = false;
        });

      mk-devnet-container = { pkgs, containerName, devNet, container-tools }:
        pkgs.lib.trace "Run Dali runtime on Composable node"
        pkgs.dockerTools.buildImage {
          name = containerName;
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ pkgs.curl pkgs.websocat ] ++ container-tools;
            pathsToLink = [ "/bin" ];
          };
          config = {
            Entrypoint = [ "${devNet}/bin/run-devnet-dali-dev" ];
            WorkingDir = "/home/polkadot-launch";
          };
          runAsRoot = ''
            mkdir -p /home/polkadot-launch /tmp
            chown 1000:1000 /home/polkadot-launch
            chmod 777 /tmp
          '';
        };

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

          # for containers which are intended for testing, debug and development (including running isolated runtime)
          docker-in-docker = with pkgs; [ docker docker-buildx docker-compose ];
          containers-tools-minimal = with pkgs; [
            acl
            direnv
            home-manager
            cachix
          ];
          container-tools = with pkgs;
            [
              bash
              bottom
              coreutils
              findutils
              gawk
              gnugrep
              less
              nettools
              nix
              procps
            ] ++ containers-tools-minimal;

         devcontainer-base-image =
            pkgs.callPackage ./.devcontainer/devcontainer-base-image.nix {
              inherit system;
            };

          # we reached limit of 125 for layers and build image cannot do non root ops, so split it
          devcontainer-root-image = pkgs.dockerTools.buildImage {
            name = "devcontainer-root-image";
            fromImage = devcontainer-base-image;
            contents = [ rust-nightly ] ++ containers-tools-minimal
              ++ docker-in-docker;
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

          mkFrontendStatic = { kusamaEndpoint, picassoEndpoint, karuraEndpoint
            , subsquidEndpoint }:
            npm-bp.buildYarnPackage {
              nativeBuildInputs = [ pkgs.pkg-config pkgs.vips pkgs.python3 ];
              src = ./frontend;

              # The filters exclude the storybooks for faster builds
              yarnBuildMore =
                "yarn export --filter=pablo --filter=picasso --filter=!picasso-storybook --filter=!pablo-storybook";

              # TODO: make these configurable
              preBuild = ''
                export SUBSQUID_URL="${subsquidEndpoint}";

                # Polkadot
                export SUBSTRATE_PROVIDER_URL_KUSAMA_2019="${picassoEndpoint}";
                export SUBSTRATE_PROVIDER_URL_KUSAMA="${kusamaEndpoint}";
                export SUBSTRATE_PROVIDER_URL_KARURA="${karuraEndpoint}";
              '';
              installPhase = ''
                mkdir -p $out
                mkdir $out/pablo
                mkdir $out/picasso
                cp -R ./apps/pablo/out/* $out/pablo
                cp -R ./apps/picasso/out/* $out/picasso
              '';
            };

          simnode-tests = crane-nightly.cargoBuild (common-attrs // {
            pnameSuffix = "-simnode";
            cargoArtifacts = common-deps;
            cargoBuildCommand =
              "cargo build --release --package simnode-tests --features=builtin-wasm";
            DALI_RUNTIME = "${dali-runtime}/lib/runtime.optimized.wasm";
            PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
            COMPOSABLE_RUNTIME =
              "${composable-runtime}/lib/runtime.optimized.wasm";
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/simnode-tests $out/bin/simnode-tests
            '';
            meta = { mainProgram = "simnode-tests"; };
          });

          run-simnode-tests = chain:
            pkgs.writeShellScriptBin "run-simnode-tests-${chain}" ''
              ${simnode-tests}/bin/simnode-tests --chain=${chain} \
              --base-path=/tmp/db/var/lib/composable-data/ \
              --pruning=archive \
              --execution=wasm
            '';

          mk-xcvm-contract = name:
            crane-nightly.buildPackage (common-attrs // {
              pnameSuffix = name;
              cargoBuildCommand =
                "cargo build --target wasm32-unknown-unknown --profile cosmwasm-contracts -p ${name}";
              RUSTFLAGS = "-C link-arg=-s";
            });

          subwasm = let
            src = pkgs.fetchFromGitHub {
              owner = "chevdor";
              repo = "subwasm";
              rev = "4d4d789326d65fc23820f70916bd6bd6f499bd0a";
              hash = "sha256-+/yqA6lP/5qyMxZupmaYBCRtbw2MFMBSgkmnxg261P8=";
            };
          in crane-stable.buildPackage {
            name = "subwasm";
            cargoArtifacts = crane-stable.buildDepsOnly {
              inherit src;
              doCheck = false;
              cargoTestCommand = "";
            };
            inherit src;
            doCheck = false;
            cargoTestCommand = "";
            meta = { mainProgram = "subwasm"; };
          };

          mkDevnetInitializeScript =
            { polkadotUrl, composableUrl, parachainIds }:
            let
              lease-period-prolongator = npm-bp.buildYarnPackage {
                nativeBuildInputs = [
                  pkgs.pkg-config
                  pkgs.python3
                  pkgs.nodePackages.node-gyp-build
                  pkgs.nodePackages.node-gyp
                  pkgs.nodePackages.typescript
                ];
                src = ./scripts/lease-period-prolongator;
                buildPhase = ''
                  yarn
                  ${pkgs.nodePackages.typescript}/bin/tsc
                '';
              };
              composablejs = npm-bp.buildYarnPackage {
                nativeBuildInputs = [
                  pkgs.pkg-config
                  pkgs.python3
                  pkgs.nodePackages.node-gyp-build
                  pkgs.nodePackages.node-gyp
                  pkgs.nodePackages.typescript
                ];
                src = ./composablejs;
                buildPhase = ''
                  yarn
                '';
              };
            in pkgs.writeShellApplication {
              name = "qa-state-initialize";
              runtimeInputs = [ pkgs.nodejs ];
              text = ''
                PARACHAIN_ENDPOINT=${composableUrl} ${pkgs.nodejs}/bin/npm run --prefix ${composablejs} start -w packages/devnet-setup
                ${builtins.concatStringsSep "\n" (builtins.map (parachainId:
                  "NODE_URL=${polkadotUrl} PARA_ID=${
                    toString parachainId
                  } ${pkgs.nodejs}/bin/node ${lease-period-prolongator}/dist/index.js")
                  parachainIds)}
              '';
            };

          frontend-static = mkFrontendStatic {
            subsquidEndpoint = "http://localhost:4350/graphql";
            picassoEndpoint = "ws://localhost:9988";
            kusamaEndpoint = "ws://localhost:9944";
            karuraEndpoint = "ws://localhost:9999";
          };

          frontend-static-persistent = mkFrontendStatic {
            subsquidEndpoint =
              "https://persistent.devnets.composablefinance.ninja/subsquid/graphql";
            picassoEndpoint =
              "wss://persistent.devnets.composablefinance.ninja/chain/dali";
            kusamaEndpoint =
              "wss://persistent.devnets.composablefinance.ninja/chain/rococo";
            karuraEndpoint =
              "wss://persistent.devnets.composablefinance.ninja/chain/karura";
          };

          frontend-static-picasso-persistent = mkFrontendStatic {
            subsquidEndpoint =
              "https://persistent.picasso.devnets.composablefinance.ninja/subsquid/graphql";
            picassoEndpoint =
              "wss://persistent.picasso.devnets.composablefinance.ninja/chain/picasso";
            kusamaEndpoint =
              "wss://persistent.picasso.devnets.composablefinance.ninja/chain/rococo";
            karuraEndpoint =
              "wss://persistent.picasso.devnets.composablefinance.ninja/chain/karura";
          };

          frontend-static-firebase = mkFrontendStatic {
            subsquidEndpoint =
              "https://persistent.devnets.composablefinance.ninja/subsquid/graphql";
            picassoEndpoint =
              "wss://persistent.devnets.composablefinance.ninja/chain/dali";
            kusamaEndpoint =
              "wss://persistent.devnets.composablefinance.ninja/chain/rococo";
            karuraEndpoint =
              "wss://persistent.devnets.composablefinance.ninja/chain/karura";
          };

          frontend-pablo-server = let PORT = 8002;
          in pkgs.writeShellApplication {
            name = "frontend-pablo-server";
            runtimeInputs = [ pkgs.miniserve ];
            text = ''
              miniserve -p ${
                builtins.toString PORT
              } --spa --index index.html ${frontend-static}/pablo
            '';
          };

          frontend-picasso-server = let PORT = 8003;
          in pkgs.writeShellApplication {
            name = "frontend-picasso-server";
            runtimeInputs = [ pkgs.miniserve ];
            text = ''
              miniserve -p ${
                builtins.toString PORT
              } --spa --index index.html ${frontend-static}/picasso
            '';
          };

        in rec {
          packages = rec {
            inherit simnode-tests;
            inherit subwasm;
            inherit frontend-static;
            inherit frontend-static-persistent;
            inherit frontend-static-picasso-persistent;
            inherit frontend-static-firebase;
            inherit frontend-pablo-server;
            inherit frontend-picasso-server;

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

            devnet-initialize-script-local = mkDevnetInitializeScript {
              polkadotUrl = "ws://localhost:9944";
              composableUrl = "ws://localhost:9988";
              parachainIds = [ 1000 2000 2087 ];
            };

            devnet-initialize-script-persistent = mkDevnetInitializeScript {
              polkadotUrl =
                "wss://persistent.devnets.composablefinance.ninja/chain/rococo";
              composableUrl =
                "wss://persistent.devnets.composablefinance.ninja/chain/dali";
              parachainIds = [ 1000 2000 2087 ];
            };

            devnet-initialize-script-picasso-persistent =
              mkDevnetInitializeScript {
                polkadotUrl =
                  "wss://persistent.picasso.devnets.composablefinance.ninja/chain/rococo";
                composableUrl =
                  "wss://persistent.picasso.devnets.composablefinance.ninja/chain/picasso";
                parachainIds = [ 1000 2000 2087 ];
              };

            xcvm-contract-asset-registry =
              mk-xcvm-contract "xcvm-asset-registry";
            xcvm-contract-router = mk-xcvm-contract "xcvm-router";
            xcvm-contract-interpreter = mk-xcvm-contract "xcvm-interpreter";

            price-feed = crane-nightly.buildPackage (common-attrs // {
              pnameSuffix = "-price-feed";
              cargoArtifacts = common-deps;
              cargoBuildCommand = "cargo build --release -p price-feed";
              meta = { mainProgram = "price-feed"; };
            });

            # Dali devnet
            devnet-dali = (pkgs.callPackage mk-devnet {
              inherit pkgs;
              inherit (packages) polkadot-launch composable-node polkadot-node;
              chain-spec = "dali-dev";
            }).script;

            # Dali bridge devnet
            bridge-devnet-dali = (mk-bridge-devnet {
              inherit pkgs packages polkadot-launch composable-node
                polkadot-node;
            }).script;

            # Dali bridge devnet with mmr-polkadot
            bridge-mmr-devnet-dali = (mk-bridge-devnet {
              inherit pkgs packages polkadot-launch composable-node;
              polkadot-node = mmr-polkadot-node;
            }).script;

            # Picasso devnet
            devnet-picasso = (pkgs.callPackage mk-devnet {
              inherit pkgs;
              inherit (packages) polkadot-launch composable-node polkadot-node;
              chain-spec = "picasso-dev";
            }).script;

            devnet-container = mk-devnet-container {
              inherit pkgs container-tools;
              containerName = "composable-devnet-container";
              devNet = packages.devnet-dali;
            };

            # Dali Bridge devnet container
            bridge-devnet-dali-container = mk-devnet-container {
              inherit pkgs container-tools;
              containerName = "composable-bridge-devnet-container";
              devNet = packages.bridge-devnet-dali;
            };

            # Dali Bridge devnet container with mmr-polkadot
            bridge-mmr-devnet-dali-container = mk-devnet-container {
              inherit pkgs container-tools;
              containerName = "composable-bridge-mmr-devnet-container";
              devNet = packages.bridge-mmr-devnet-dali;
            };

            # TODO: inherit and provide script to run all stuff

            # devnet-container-xcvm
            # NOTE: The devcontainer is currently broken for aarch64.
            # Please use the developers devShell instead

            devcontainer = pkgs.dockerTools.buildLayeredImage {
              name = "composable-devcontainer";
              fromImage = devcontainer-root-image;
              contents = [ composable-node ];
              # substituters, same as next script, but without internet access
              # ${pkgs.cachix}/bin/cachix use composable-community
              # to run root in buildImage needs qemu/kvm shell
              # non root extraCommands (in both methods) do not have permissions
              # not clear if using ENV or replace ENTRYPOINT will allow to setup
              # from nixos docker.nix - they build derivation which outputs into $out/etc/nix.conf
              # (and any other stuff like /etc/group)
              fakeRootCommands = ''
                mkdir --parents /etc/nix
                cat <<EOF >> /etc/nix/nix.conf
                sandbox = relaxed
                experimental-features = nix-command flakes
                narinfo-cache-negative-ttl = 30
                substituters = https://cache.nixos.org https://composable-community.cachix.org
                # TODO: move it separate file with flow of `cachix -> get keys -> output -> fail derivation if hash != key changed
                # // cspell: disable-next-line
                trusted-public-keys = cache.nixos.org-1:6nchdd59x431o0gwypbmraurkbj16zpmqfgspcdshjy= composable-community.cachix.org-1:GG4xJNpXJ+J97I8EyJ4qI5tRTAJ4i7h+NK2Z32I8sK8=
                EOF
              '';
              config = {
                User = "vscode";
                # TODO: expose ports and other stuff done in base here too
              };
            };

            check-dali-dev-benchmarks = benchmarks-run-once "dali-dev";
            check-picasso-dev-benchmarks = benchmarks-run-once "picasso-dev";
            check-composable-dev-benchmarks =
              benchmarks-run-once "composable-dev";

            check-picasso-integration-tests = crane-nightly.cargoBuild
              (common-attrs // {
                pname = "picasso-local-integration-tests";
                doInstallCargoArtifacts = false;
                cargoArtifacts = common-test-deps;
                cargoBuildCommand =
                  "cargo test --package local-integration-tests";
                cargoExtraArgs =
                  "--features=local-integration-tests,picasso,std --no-default-features --verbose";
              });
            check-dali-integration-tests = crane-nightly.cargoBuild
              (common-attrs // {
                pname = "dali-local-integration-tests";
                doInstallCargoArtifacts = false;
                cargoArtifacts = common-test-deps;
                cargoBuildCommand =
                  "cargo test --package local-integration-tests";
                cargoExtraArgs =
                  "--features=local-integration-tests,dali,std --no-default-features --verbose";
              });

            unit-tests = crane-nightly.cargoBuild (common-attrs // {
              pnameSuffix = "-tests";
              doInstallCargoArtifacts = false;
              cargoArtifacts = common-test-deps;
              # NOTE: do not add --features=runtime-benchmarks because it force multi ED to be 0 because of dependencies
              # NOTE: in order to run benchmarks as tests, just make `any(test, feature = "runtime-benchmarks")
              cargoBuildCommand =
                "cargo test --workspace --release --locked --verbose --exclude local-integration-tests";
            });

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

            unit-tests-with-coverage = crane-nightly.cargoBuild (common-attrs
              // {
                pnameSuffix = "-tests-with-coverage";
                buildInputs = with pkgs; [ cargo-llvm-cov ];
                cargoArtifacts = common-deps-nightly;
                # NOTE: do not add --features=runtime-benchmarks because it force multi ED to be 0 because of dependencies
                # NOTE: in order to run benchmarks as tests, just make `any(test, feature = "runtime-benchmarks")
                cargoBuildCommand = "cargo llvm-cov";
                cargoExtraArgs =
                  "--workspace --release --locked --verbose --lcov --output-path lcov.info";
                installPhase = ''
                  mkdir -p $out/lcov
                  mv lcov.info $out/lcov
                '';
              });
            kusama-picasso-karura-devnet = let
              config = (pkgs.callPackage
                ./scripts/polkadot-launch/kusama-local-picasso-dev-karura-dev.nix {
                  polkadot-bin = polkadot-node;
                  composable-bin = composable-node;
                  acala-bin = acala-node;
                }).result;
              config-file = pkgs.writeTextFile {
                name = "kusama-local-picasso-dev-karura-dev.json";
                text = "${builtins.toJSON config}";
              };
            in pkgs.writeShellApplication {
              name = "kusama-picasso-karura";
              text = ''
                cat ${config-file}
                rm -rf /tmp/polkadot-launch
                ${packages.polkadot-launch}/bin/polkadot-launch ${config-file} --verbose
              '';
            };

            devnet-rococo-dali-karura = let
              config = (pkgs.callPackage
                ./scripts/polkadot-launch/kusama-local-dali-dev-karura-dev.nix {
                  polkadot-bin = polkadot-node;
                  composable-bin = composable-node;
                  acala-bin = acala-node;
                }).result;
              config-file = pkgs.writeTextFile {
                name = "kusama-local-dali-dev-karura-dev.json";
                text = "${builtins.toJSON config}";
              };
            in pkgs.writeShellApplication {
              name = "run-rococo-dali-karura";
              text = ''
                cat ${config-file}
                rm -rf /tmp/polkadot-launch
                ${packages.polkadot-launch}/bin/polkadot-launch ${config-file} --verbose
              '';
            };

            devnet-picasso-complete = let
              config =
                (pkgs.callPackage ./scripts/polkadot-launch/all-dev-local.nix {
                  chainspec = "picasso-dev";
                  polkadot-bin = polkadot-node;
                  composable-bin = composable-node;
                  statemine-bin = statemine-node;
                  acala-bin = acala-node;
                }).result;
              config-file = pkgs.writeTextFile {
                name = "all-dev-local.json";
                text = "${builtins.toJSON config}";
              };
            in pkgs.writeShellApplication {
              name = "devnet-picasso-complete";
              text = ''
                cat ${config-file}
                rm -rf /tmp/polkadot-launch
                ${packages.polkadot-launch}/bin/polkadot-launch ${config-file} --verbose
              '';
            };

            devnet-dali-complete = let
              config =
                (pkgs.callPackage ./scripts/polkadot-launch/all-dev-local.nix {
                  chainspec = "dali-dev";
                  polkadot-bin = polkadot-node;
                  composable-bin = composable-node;
                  statemine-bin = statemine-node;
                  acala-bin = acala-node;
                }).result;
              config-file = pkgs.writeTextFile {
                name = "all-dev-local.json";
                text = "${builtins.toJSON config}";
              };
            in pkgs.writeShellApplication {
              name = "devnet-dali-complete";
              text = ''
                cat ${config-file}
                rm -rf /tmp/polkadot-launch
                ${packages.polkadot-launch}/bin/polkadot-launch ${config-file} --verbose
              '';
            };


            devnet-default-program =
              pkgs.composable.mkDevnetProgram "devnet-default"
              (import ./.nix/devnet-specs/default.nix {
                inherit pkgs;
                inherit price-feed;
                devnet = devnet-dali-complete;
                frontend = frontend-static;
              });

            devnet-xcvm-program = pkgs.composable.mkDevnetProgram "devnet-xcvm"
              (import ./.nix/devnet-specs/xcvm.nix {
                inherit pkgs;
                inherit devnet-dali;
              });

            devnet-persistent-program =
              pkgs.composable.mkDevnetProgram "devnet-persistent"
              (import ./.nix/devnet-specs/default.nix {
                inherit pkgs;
                inherit price-feed;
                devnet = devnet-dali-complete;
                frontend = frontend-static-persistent;
              });

            devnet-picasso-persistent-program =
              pkgs.composable.mkDevnetProgram "devnet-persistent"
              (import ./.nix/devnet-specs/default.nix {
                inherit pkgs;
                inherit price-feed;
                devnet = devnet-picasso-complete;
                frontend = frontend-static-picasso-persistent;
              });

            default = packages.composable-node;
          };


          apps = let
            makeApp = p: {
              type = "app";
              program = pkgs.lib.meta.getExe p;
            };
          in rec {
            devnet = makeApp packages.devnet-default-program;
            devnet-persistent = makeApp packages.devnet-persistent-program;
            devnet-picasso-persistent =
              makeApp packages.devnet-picasso-persistent-program;
            devnet-xcvm = makeApp packages.devnet-xcvm-program;
            devnet-dali = makeApp packages.devnet-dali;
            devnet-picasso = makeApp packages.devnet-picasso;
            devnet-kusama-picasso-karura =
              makeApp packages.kusama-picasso-karura-devnet;
            devnet-rococo-dali-karura =
              makeApp packages.devnet-rococo-dali-karura;
            devnet-picasso-complete = makeApp packages.devnet-picasso-complete;
            devnet-dali-complete = makeApp packages.devnet-dali-complete;
            price-feed = makeApp packages.price-feed;
            composable = makeApp packages.composable-node;
            acala = makeApp packages.acala-node;
            polkadot = makeApp packages.polkadot-node;
            junod = makeApp packages.junod;
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
            simnode-tests = makeApp packages.simnode-tests;
            simnode-tests-composable =
              flake-utils.lib.mkApp { drv = run-simnode-tests "composable"; };
            simnode-tests-picasso =
              flake-utils.lib.mkApp { drv = run-simnode-tests "picasso"; };
            simnode-tests-dali-rococo =
              flake-utils.lib.mkApp { drv = run-simnode-tests "dali-rococo"; };
            devnet-initialize-script-local =
              makeApp packages.devnet-initialize-script-local;
            devnet-initialize-script-persistent =
              makeApp packages.devnet-initialize-script-persistent;
            devnet-initialize-script-picasso-persistent =
              makeApp packages.devnet-initialize-script-picasso-persistent;
            default = devnet-dali;
          };
        });
    in eachSystemOutputs // {

      nixopsConfigurations = {
        default = let pkgs = nixpkgs.legacyPackages.x86_64-linux;
        in import ./.nix/devnet.nix {
          inherit nixpkgs;
          inherit gce-input;
          devnet-dali = pkgs.callPackage mk-devnet {
            inherit pkgs;
            inherit (eachSystemOutputs.packages.x86_64-linux)
              polkadot-launch composable-node polkadot-node;
            chain-spec = "dali-dev";
          };
          devnet-picasso = pkgs.callPackage mk-devnet {
            inherit pkgs;
            inherit (eachSystemOutputs.packages.x86_64-linux)
              polkadot-launch composable-node polkadot-node;
            chain-spec = "picasso-dev";
          };
          docs = eachSystemOutputs.packages.x86_64-linux.docs-static;
          rev = builtins.getEnv "GITHUB_SHA";
        };
      };
      homeConfigurations = let
        mk-docker-in-docker = pkgs: [
          # TODO: this home works well in VS Devcontainer launcher as it injects low-level Dockerd
          # For manual runs need tuning to setup it (need to mount docker links to root and do they executable)
          # INFO[2022-09-06T13:14:43.437764897Z] Starting up
          # dockerd needs to be started with root privileges. To run dockerd in rootless mode as an unprivileged user, see https://docs.docker.com/go/rootless/ dockerd-rootless-setuptool.sh install
          pkgs.docker
          pkgs.docker-buildx
          pkgs.docker-compose
        ];
        mk-containers-tools-minimal = pkgs: [
          pkgs.acl
          pkgs.direnv
          pkgs.cachix
        ];
      in {

        # minimal means we do not build in composable devnets and tooling, but allow to build or nix these
        vscode-minimal.x86_64-linux =
          let pkgs = nixpkgs.legacyPackages.x86_64-linux;
          in with pkgs;
          home-manager.lib.homeManagerConfiguration {
            inherit pkgs;
            modules = [{
              home = {
                username = "vscode";
                homeDirectory = "/home/vscode";
                stateVersion = "22.05";
                packages =
                  [ eachSystemOutputs.packages.x86_64-linux.rust-nightly subxt ]
                  ++ (mk-containers-tools-minimal pkgs)
                  ++ (mk-docker-in-docker pkgs);
              };
              programs = {
                home-manager.enable = true;
                direnv = {
                  enable = true;
                  nix-direnv = { enable = true; };
                };
              };
            }];
          };

        vscode-minimal.aarch64-linux =
          let pkgs = nixpkgs.legacyPackages.aarch64-linux;
          in with pkgs;
          home-manager.lib.homeManagerConfiguration {
            inherit pkgs;
            modules = [{
              home = {
                username = "vscode";
                homeDirectory = "/home/vscode";
                stateVersion = "22.05";
                packages = [
                  eachSystemOutputs.packages.aarch64-linux.rust-nightly
                  subxt
                ] ++ (mk-containers-tools-minimal pkgs)
                  ++ (mk-docker-in-docker pkgs);
              };
              programs = {
                home-manager.enable = true;
                direnv = {
                  enable = true;
                  nix-direnv = { enable = true; };
                };
              };
            }];
          };

      };
    };
}
