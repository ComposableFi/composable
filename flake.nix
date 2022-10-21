{
  # see ./docs/nix-overview.md for design guidelines of nix organization
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
  };
  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, npm-buildpackage
    , arion-src, home-manager, helix }:
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

      mkDevnetProgram = { pkgs }:
        name: spec:
        pkgs.writeShellApplication {
          inherit name;
          runtimeInputs = [ pkgs.arion pkgs.docker pkgs.coreutils pkgs.bash ];
          text = ''
            arion --prebuilt-file ${
              pkgs.arion.build spec
            } up --build --force-recreate -V --always-recreate-deps --remove-orphans
          '';
        };

      composableOverlay = nixpkgs.lib.composeManyExtensions [
        arion-src.overlay
        (final: _prev: {
          composable = {
            mkDevnetProgram = final.callPackage mkDevnetProgram { };
          };
        })
      ];

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

      all-such-files = { pkgs, extension }:
        pkgs.stdenv.mkDerivation {
          name = "all-such-files-${extension}";
          src = builtins.filterSource (path: type:
            (type == "directory" && baseNameOf path != ".git") || (type
              == "regular" && pkgs.lib.strings.hasSuffix ".${extension}" path))
            ./.;
          dontUnpack = true;
          installPhase = ''
            mkdir $out/
            cp -r $src/. $out/
          '';
        };

      eachSystemOutputs = flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [
              composableOverlay
              rust-overlay.overlays.default
              npm-buildpackage.overlays.default
            ];
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
          npm-bp = pkgs.callPackage npm-buildpackage { };

          # Stable rust for anything except wasm runtime
          rust-stable = pkgs.rust-bin.stable.latest.default;

          rust-nightly =
            pkgs.rust-bin.fromRustupToolchainFile ./rust-toolchain.toml;

          # Crane lib instantiated with current nixpkgs
          crane-lib = crane.mkLib pkgs;

          # Crane pinned to stable Rust
          crane-stable = crane-lib.overrideToolchain rust-stable;

          # Crane pinned to nightly Rust
          crane-nightly = crane-lib.overrideToolchain rust-nightly;

          wasm-optimizer = crane-stable.buildPackage (common-attrs // {
            cargoCheckCommand = "true";
            pname = "wasm-optimizer";
            cargoArtifacts = common-deps;
            cargoBuildCommand =
              "cargo build --release --package wasm-optimizer";
            version = "0.1.0";
            # NOTE: we copy more then needed, but tht is simpler to setup, we depend on substrate for sure so
          });

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

          # source relevant to build rust only
          rust-src = let
            directoryBlacklist = [ "runtime-tests" ];
            fileBlacklist = [
              # does not makes sense to black list,
              # if we changed some version of tooling(seldom), we want to rebuild
              # so if we changed version of tooling, nix itself will detect invalidation and rebuild
              # "flake.lock"
            ];
          in pkgs.lib.cleanSourceWith {
            filter = pkgs.lib.cleanSourceFilter;
            src = pkgs.lib.cleanSourceWith {
              filter = let
                isBlacklisted = name: type:
                  let
                    blacklist = if type == "directory" then
                      directoryBlacklist
                    else if type == "regular" then
                      fileBlacklist
                    else
                      [ ]; # symlink, unknown
                  in builtins.elem (baseNameOf name) blacklist;
                isImageFile = name: type:
                  type == "regular" && pkgs.lib.strings.hasSuffix ".png" name;
                isPlantUmlFile = name: type:
                  type == "regular"
                  && pkgs.lib.strings.hasSuffix ".plantuml" name;
                isNixFile = name: type:
                  type == "regular" && pkgs.lib.strings.hasSuffix ".nix" name;
                customFilter = name: type:
                  !((isBlacklisted name type) || (isImageFile name type)
                    || (isPlantUmlFile name type)
                    # assumption that nix is final builder,
                    # so there would no be sandwich like  .*.nix <- build.rs <- *.nix
                    # and if *.nix changed, nix itself will detect only relevant cache invalidations
                    || (isNixFile name type));
              in pkgs.nix-gitignore.gitignoreFilterPure customFilter
              [ ./.gitignore ] ./code;
              src = ./code;
            };
          };

          substrate-attrs = {
            LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
              pkgs.stdenv.cc.cc.lib
              pkgs.llvmPackages.libclang.lib
            ];
            LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
            PROTOC = "${pkgs.protobuf}/bin/protoc";
            ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
          };

          # Common env required to build the node
          common-attrs = substrate-attrs // {
            src = rust-src;
            buildInputs = with pkgs; [ openssl zstd ];
            nativeBuildInputs = with pkgs;
              [ clang openssl pkg-config ] ++ pkgs.lib.optional stdenv.isDarwin
              (with darwin.apple_sdk.frameworks; [
                Security
                SystemConfiguration
              ]);
            doCheck = false;
            cargoCheckCommand = "true";
            # Don't build any wasm as we do it ourselves
            SKIP_WASM_BUILD = "1";
          };

          # TODO: refactor as mkOverride common-attrs
          common-test-deps-attrs = substrate-attrs // {
            src = rust-src;
            buildInputs = with pkgs; [ openssl zstd ];
            nativeBuildInputs = with pkgs;
              [ clang openssl pkg-config ] ++ pkgs.lib.optional stdenv.isDarwin
              (with darwin.apple_sdk.frameworks; [
                Security
                SystemConfiguration
              ]);
            doCheck = true;
            SKIP_WASM_BUILD = "1";
          };

          # Common dependencies, all dependencies listed that are out of this repo
          common-deps = crane-nightly.buildDepsOnly (common-attrs // { });
          common-deps-nightly =
            crane-nightly.buildDepsOnly (common-attrs // { });
          common-bench-attrs = common-attrs // {
            cargoExtraArgs = "--features=builtin-wasm,runtime-benchmarks";
          };
          common-test-deps =
            crane-nightly.buildDepsOnly (common-test-deps-attrs // { });

          common-bench-deps =
            crane-nightly.buildDepsOnly (common-bench-attrs // { });

          # Build a wasm runtime, unoptimized
          mk-runtime = name: features:
            crane-nightly.buildPackage (common-attrs // {
              pname = "${name}-runtime";
              cargoArtifacts = common-deps-nightly;
              cargoBuildCommand =
                "cargo build --release -p ${name}-runtime-wasm --target wasm32-unknown-unknown"
                + pkgs.lib.strings.optionalString (features != "")
                (" --features=${features}");
              # From parity/wasm-builder
              RUSTFLAGS =
                "-Clink-arg=--export=__heap_base -Clink-arg=--import-memory";
            });

          # Derive an optimized wasm runtime from a prebuilt one, garbage collection + compression
          mk-optimized-runtime = { name, features ? "" }:
            let runtime = mk-runtime name features;
            in pkgs.stdenv.mkDerivation {
              name = "${runtime.name}-optimized";
              phases = [ "installPhase" ];
              installPhase = ''
                mkdir -p $out/lib
                ${wasm-optimizer}/bin/wasm-optimizer \
                --input ${runtime}/lib/${name}_runtime.wasm \
                --output $out/lib/runtime.optimized.wasm
              '';
            };

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

          dali-runtime = mk-optimized-runtime {
            name = "dali";
            features = "";
          };
          picasso-runtime = mk-optimized-runtime {
            name = "picasso";
            features = "";
          };
          composable-runtime = mk-optimized-runtime {
            name = "composable";
            features = "";
          };
          dali-bench-runtime = mk-optimized-runtime {
            name = "dali";
            features = "runtime-benchmarks";
          };
          picasso-bench-runtime = mk-optimized-runtime {
            name = "picasso";
            features = "runtime-benchmarks";
          };
          composable-bench-runtime = mk-optimized-runtime {
            name = "composable";
            features = "runtime-benchmarks";
          };

          # NOTE: with docs, non nightly fails but nightly fails too...
          # /nix/store/523zlfzypzcr969p058i6lcgfmg889d5-stdenv-linux/setup: line 1393: --message-format: command not found
          composable-node = crane-nightly.buildPackage (common-attrs // {
            name = "composable";
            cargoArtifacts = common-deps;
            cargoBuildCommand =
              "cargo build --release --package composable --features=builtin-wasm";
            DALI_RUNTIME = "${dali-runtime}/lib/runtime.optimized.wasm";
            PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
            COMPOSABLE_RUNTIME =
              "${composable-runtime}/lib/runtime.optimized.wasm";
            installPhase = ''
              mkdir -p $out/bin
              cp target/release/composable $out/bin/composable
            '';
            meta = { mainProgram = "composable"; };
          });

          composable-node-release = crane-nightly.buildPackage (common-attrs
            // {
              name = "composable";
              cargoArtifacts = common-deps;
              cargoBuildCommand = "cargo build --release --package composable";
              SUBSTRATE_CLI_GIT_COMMIT_HASH = if self ? rev then
                self.rev
              else
                builtins.abort "Cannot build the release node in a dirty repo.";
              installPhase = ''
                mkdir -p $out/bin
                cp target/release/composable $out/bin/composable
              '';
              meta = { mainProgram = "composable"; };
            });

          composable-bench-node = crane-nightly.cargoBuild (common-bench-attrs
            // {
              name = "composable";
              cargoArtifacts = common-bench-deps;
              cargoBuildCommand = "cargo build --release --package composable";
              DALI_RUNTIME = "${dali-bench-runtime}/lib/runtime.optimized.wasm";
              PICASSO_RUNTIME =
                "${picasso-bench-runtime}/lib/runtime.optimized.wasm";
              COMPOSABLE_RUNTIME =
                "${composable-bench-runtime}/lib/runtime.optimized.wasm";
              installPhase = ''
                mkdir -p $out/bin
                cp target/release/composable $out/bin/composable
              '';
              meta = { mainProgram = "composable"; };
            });

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

          docs-renders = with pkgs; [ nodejs plantuml graphviz pandoc ];

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

          subwasm-release-body = let
            subwasm-call = runtime:
              builtins.readFile (pkgs.runCommand "subwasm-info" { }
                "${subwasm}/bin/subwasm info ${runtime}/lib/runtime.optimized.wasm | tail -n+2 > $out");
          in pkgs.writeTextFile {
            name = "release.txt";
            text = ''
              ## Runtimes
              ### Dali
              ```
              ${subwasm-call dali-runtime}
              ```
              ### Picasso
              ```
              ${subwasm-call picasso-runtime}
              ```
              ### Composable
              ```
              ${subwasm-call composable-runtime}
              ```
            '';
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
            inherit wasm-optimizer;
            inherit common-deps;
            inherit common-bench-deps;
            inherit common-test-deps;
            inherit dali-runtime;
            inherit picasso-runtime;
            inherit composable-runtime;
            inherit dali-bench-runtime;
            inherit picasso-bench-runtime;
            inherit composable-bench-runtime;
            inherit composable-node;
            inherit composable-node-release;
            inherit composable-bench-node;
            inherit rust-nightly;
            inherit simnode-tests;
            inherit subwasm;
            inherit subwasm-release-body;
            inherit frontend-static;
            inherit frontend-static-persistent;
            inherit frontend-static-picasso-persistent;
            inherit frontend-static-firebase;
            inherit frontend-pablo-server;
            inherit frontend-picasso-server;

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

            docs-static = npm-bp.buildNpmPackage {
              src = ./docs;
              npmBuild = "npm run build";
              installPhase = ''
                mkdir -p $out
                cp -a ./build/. $out
              '';
            };

            docs-server = let PORT = 8008;
            in pkgs.writeShellApplication {
              name = "docs-server";
              runtimeInputs = [ pkgs.miniserve ];
              text = ''
                miniserve -p ${
                  builtins.toString PORT
                } --spa --index index.html ${docs-static}
              '';
            };

            docs-dev = pkgs.writeShellApplication {
              name = "docs-dev";
              runtimeInputs = [ pkgs.nodejs ];
              text = ''
                cd docs
                npm install
                npm run start
              '';
            };

            xcvm-contract-asset-registry =
              mk-xcvm-contract "xcvm-asset-registry";
            xcvm-contract-router = mk-xcvm-contract "xcvm-router";
            xcvm-contract-interpreter = mk-xcvm-contract "xcvm-interpreter";
            subxt =
              pkgs.callPackage ./code/utils/composable-subxt/subxt.nix { };

            subsquid-processor = let
              processor = pkgs.buildNpmPackage {
                extraNodeModulesArgs = {
                  buildInputs = with pkgs; [
                    pkg-config
                    python3
                    nodePackages.node-gyp-build
                    nodePackages.node-gyp
                  ];
                  extraEnvVars = { npm_config_nodedir = "${pkgs.nodejs}"; };
                };
                src = ./subsquid;
                npmBuild = "npm run build";
                preInstall = ''
                  mkdir $out
                  mv lib $out/
                '';
                dontNpmPrune = true;
              };
            in (pkgs.writeShellApplication {
              name = "run-subsquid-processor";
              text = ''
                cd ${processor}
                ${pkgs.nodejs}/bin/npx sqd db migrate
                ${pkgs.nodejs}/bin/node lib/processor.js
              '';
            });

            runtime-tests = pkgs.stdenv.mkDerivation {
              name = "runtime-tests";
              src = builtins.filterSource
                (path: _type: baseNameOf path != "node_modules")
                ./code/integration-tests/runtime-tests;
              dontUnpack = true;
              installPhase = ''
                mkdir $out/
                cp -r $src/. $out/
              '';
            };

            all-directories-and-files = pkgs.stdenv.mkDerivation {
              name = "all-directories-and-files";
              src =
                builtins.filterSource (path: _type: baseNameOf path != ".git")
                ./.;
              dontUnpack = true;
              installPhase = ''
                mkdir $out/
                cp -r $src/. $out/
              '';
            };

            all-toml-files = all-such-files {
              inherit pkgs;
              extension = "toml";
            };

            all-nix-files = all-such-files {
              inherit pkgs;
              extension = "nix";
            };

            price-feed = crane-nightly.buildPackage (common-attrs // {
              pnameSuffix = "-price-feed";
              cargoArtifacts = common-deps;
              cargoBuildCommand = "cargo build --release -p price-feed";
              meta = { mainProgram = "price-feed"; };
            });

            fmt = pkgs.writeShellApplication {
              name = "fmt-composable";

              runtimeInputs = with pkgs; [
                nixfmt
                coreutils
                rust-nightly
                taplo
                nodePackages.prettier
              ];

              text = ''
                  # .nix
                	find . -name "*.nix" -type f -print0 | xargs -0 nixfmt;

                  # .toml
                  taplo fmt

                  # .rs
                	find . -path ./code/target -prune -o -name "*.rs" -type f -print0 | xargs -0 rustfmt --edition 2021;

                  # .js .ts .tsx
                  prettier \
                    --config="./code/integration-tests/runtime-tests/.prettierrc" \
                    --write \
                    --ignore-path="./code/integration-tests/runtime-tests/.prettierignore" \
                    ./code/integration-tests/runtime-tests/
              '';
            };

            docker-wipe-system =
              pkgs.writeShellScriptBin "docker-wipe-system" ''
                echo "Wiping all docker containers, images, and volumes";
                docker stop $(docker ps -q)
                docker system prune -f
                docker rmi -f $(docker images -a -q)
                docker volume prune -f
              '';

            # NOTE: crane can't be used because of how it vendors deps, which is incompatible with some packages in polkadot, an issue must be raised to the repo
            acala-node = pkgs.callPackage ./.nix/acala-bin.nix {
              rust-overlay = rust-nightly;
            };

            polkadot-node = pkgs.callPackage ./.nix/polkadot/polkadot-bin.nix {
              inherit rust-nightly;
            };

            statemine-node = pkgs.callPackage ./.nix/statemine-bin.nix {
              inherit rust-nightly;
            };

            mmr-polkadot-node =
              pkgs.callPackage ./.nix/polkadot/mmr-polkadot-bin.nix {
                inherit rust-nightly;
              };

            polkadot-launch =
              pkgs.callPackage ./scripts/polkadot-launch/polkadot-launch.nix
              { };

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

            cargo-fmt-check = crane-nightly.cargoFmt (common-attrs // {
              cargoArtifacts = common-deps-nightly;
              cargoExtraArgs = "--all --check --verbose";
            });

            taplo-cli-check = pkgs.stdenv.mkDerivation {
              name = "taplo-cli-check";
              dontUnpack = true;
              buildInputs = [ all-toml-files pkgs.taplo-cli ];
              installPhase = ''
                mkdir $out
                cd ${all-toml-files}
                taplo check --verbose
              '';
            };

            prettier-check = pkgs.stdenv.mkDerivation {
              name = "prettier-check";
              dontUnpack = true;
              buildInputs = [ pkgs.nodePackages.prettier runtime-tests ];
              installPhase = ''
                mkdir $out
                prettier \
                --config="${runtime-tests}/.prettierrc" \
                --ignore-path="${runtime-tests}/.prettierignore" \
                --check \
                --loglevel=debug \
                ${runtime-tests}
              '';
            };

            nixfmt-check = pkgs.stdenv.mkDerivation {
              name = "nixfmt-check";
              dontUnpack = true;

              buildInputs = [ all-nix-files pkgs.nixfmt ];
              installPhase = ''
                mkdir $out
                nixfmt --version
                SRC=$(find ${all-nix-files} -name "*.nix" -type f | tr "\n" " ")
                echo $SRC
                nixfmt --check $SRC
              '';
            };

            deadnix-check = pkgs.stdenv.mkDerivation {
              name = "deadnix-check";
              dontUnpack = true;

              buildInputs = [ all-nix-files pkgs.deadnix ];
              installPhase = ''
                mkdir $out
                deadnix --version
                SRC=$(find ${all-nix-files} -name "*.nix" -type f | tr "\n" " ")
                echo $SRC
                deadnix $SRC
              '';
            };

            cargo-clippy-check = crane-nightly.cargoBuild (common-attrs // {
              cargoArtifacts = common-deps-nightly;
              cargoBuildCommand = "cargo clippy";
              cargoExtraArgs = "--all-targets --tests -- -D warnings";
            });

            cargo-deny-check = crane-nightly.cargoBuild (common-attrs // {
              buildInputs = with pkgs; [ cargo-deny ];
              cargoArtifacts = common-deps;
              cargoBuildCommand = "cargo deny";
              cargoExtraArgs =
                "--manifest-path ./parachain/frame/composable-support/Cargo.toml check ban";
            });

            cargo-udeps-check = crane-nightly.cargoBuild (common-attrs // {
              DALI_RUNTIME = "${dali-runtime}/lib/runtime.optimized.wasm";
              PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
              COMPOSABLE_RUNTIME =
                "${composable-runtime}/lib/runtime.optimized.wasm";
              buildInputs = with pkgs; [
                cargo-udeps
                expat
                freetype
                fontconfig
                openssl
              ];
              cargoArtifacts = common-deps-nightly;
              cargoBuildCommand = "cargo udeps";
              cargoExtraArgs =
                "--workspace --exclude local-integration-tests --all-features";
            });

            benchmarks-check = crane-nightly.cargoBuild (common-attrs // {
              cargoArtifacts = common-deps-nightly;
              cargoBuildCommand = "cargo check";
              cargoExtraArgs = "--benches --all --features runtime-benchmarks";
            });

            spell-check = pkgs.stdenv.mkDerivation {
              name = "cspell-check";
              dontUnpack = true;
              buildInputs =
                [ all-directories-and-files pkgs.nodePackages.cspell ];
              installPhase = ''
                mkdir $out
                echo "cspell version: $(cspell --version)"
                cd ${all-directories-and-files}
                cspell lint --config cspell.yaml --no-progress "**"
              '';
            };

            hadolint-check = pkgs.stdenv.mkDerivation {
              name = "hadolint-check";
              dontUnpack = true;
              buildInputs = [ all-directories-and-files pkgs.hadolint ];
              installPhase = ''
                mkdir -p $out

                hadolint --version
                total_exit_code=0
                for file in $(find ${all-directories-and-files} -name "Dockerfile" -or -name "*.dockerfile"); do
                  echo "=== $file ==="
                  hadolint --config ${all-directories-and-files}/.hadolint.yaml $file || total_exit_code=$?
                  echo ""
                done
                exit $total_exit_code
              '';
            };

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

            junod = pkgs.callPackage ./code/xcvm/cosmos/junod.nix { };
            gex = pkgs.callPackage ./code/xcvm/cosmos/gex.nix { };
            wasmswap = pkgs.callPackage ./code/xcvm/cosmos/wasmswap.nix {
              crane = crane-nightly;
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

          devShells = rec {

            base-shell = pkgs.mkShell {
              buildInputs = [ helix.packages.${pkgs.system}.default ];
              NIX_PATH = "nixpkgs=${pkgs.path}";
            };

            docs = base-shell.overrideAttrs (base: {
              buildInputs = base.buildInputs ++ (with pkgs; [ python3 nodejs ]);
            });

            developers-minimal = base-shell.overrideAttrs (base:
              common-attrs // {
                buildInputs = base.buildInputs ++ [
                  pkgs.clang
                  packages.rust-nightly
                  packages.subwasm
                  pkgs.nodejs
                  pkgs.python3
                  pkgs.yarn
                ];
                LD_LIBRARY_PATH = pkgs.lib.strings.makeLibraryPath [
                  pkgs.stdenv.cc.cc.lib
                  pkgs.llvmPackages.libclang.lib
                ];
                LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
                PROTOC = "${pkgs.protobuf}/bin/protoc";
                ROCKSDB_LIB_DIR = "${pkgs.rocksdb}/lib";
                NIX_PATH = "nixpkgs=${pkgs.path}";
              });

            developers = developers-minimal.overrideAttrs (base: {
              buildInputs = base.buildInputs ++ [
                pkgs.bacon
                pkgs.google-cloud-sdk
                pkgs.grub2
                pkgs.jq
                pkgs.lldb
                pkgs.llvmPackages_latest.bintools
                pkgs.llvmPackages_latest.lld
                pkgs.llvmPackages_latest.llvm
                pkgs.nix-tree
                pkgs.nixpkgs-fmt
                pkgs.openssl
                pkgs.openssl.dev
                pkgs.pkg-config
                pkgs.qemu
                pkgs.rnix-lsp
                pkgs.taplo
                pkgs.xorriso
                pkgs.zlib.out
                pkgs.nix-tree
                pkgs.nixfmt
                pkgs.rnix-lsp
                pkgs.nodePackages.typescript
                pkgs.nodePackages.typescript-language-server
                packages.subxt
              ] ++ docs-renders;
            });

            developers-with-wasm-optimizer = developers.overrideAttrs (base: {
              buildInputs = base.buildInputs ++ [ packages.wasm-optimizer ];
            });

            developers-xcvm = developers.overrideAttrs (base: {
              buildInputs = with packages;
                base.buildInputs ++ [ junod gex ] ++ pkgs.lib.lists.optional
                (pkgs.lib.strings.hasSuffix "linux" system) arion;
              shellHook = ''
                echo ""
                echo ""
                echo ""
                echo "==================================================================================================="
                echo " /!\ Generating alice key, junod will abort if the key is already present (everything is fine.) /!\ "
                echo "==================================================================================================="
                echo ""
                echo ""
                echo ""
                echo "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose" | junod keys add alice --recover --keyring-backend test || true
              '';
            });

            ci = pkgs.mkShell {
              buildInputs = [ pkgs.nixopsUnstable ];
              NIX_PATH = "nixpkgs=${pkgs.path}";
            };

            default = developers;
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
              };
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
            };
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

      overlays.default = composableOverlay;
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
