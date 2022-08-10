{
  # see ./docs/nix.md for design guidelines of nix organization
  description =
    "Composable Finance Local Networks Lancher and documentation Book";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
      # different version (we likely have old and conflicts)
      # inputs.flake-utils.follows = "flake-utils";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    # NOTE: gives weird error
    # nixops-flake = {
    #   url = "github:NixOS/nixops?rev=35ac02085169bc2372834d6be6cf4c1bdf820d09";
    #   inputs.nixpkgs.follows = "nixpkgs";
    #   inputs.utils.follows = "flake-utils";
    # };
  };
  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, # nixpkgs-fmt,
    #, nixops-flake
    }:
    let
      eachSystemOutputs = flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
            allowUnsupportedSystem = true; # we do not tirgger this on mac
            permittedInsecurePackages = [
              "openjdk-headless-16+36" # something depends on it
            ];
          };
          nixops = pkgs.callPackage ./.nix/nixops.nix { };
          overlays = [ (import rust-overlay) ];
          rust-toolchain = import ./.nix/rust-toolchain.nix;
        in with pkgs;
        let
          # Stable rust for anything except wasm runtime
          rust-stable = rust-bin.stable.latest.default;

          # Nightly rust used for wasm runtime compilation
          rust-nightly = rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.default.override {
              extensions = [ "rust-src" ];
              targets = [ "wasm32-unknown-unknown" ];
            });

          rust-nightly-dev = rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.default.override {
              extensions = [ "rust-src" "clippy" "rustfmt" ];
              targets = [ "wasm32-unknown-unknown" ];
            });

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
            # NOTE: we copy more then needed, but tht is simpler to setup, we depend pn substrae for sure so
          });

          # for containers which are intented for testing, debug and development (including running isolated runtime)
          container-tools =
            [ coreutils bash procps findutils nettools bottom nix procps ];

          # source relevant to build rust only
          rust-src = let
            dir-blacklist = [
              "nix"
              ".config"
              ".devcontainer"
              ".github"
              ".log"
              ".maintain"
              ".tools"
              ".vscode"
              "audits"
              "book"
              "devnet-stage"
              "devnet"
              "docker"
              "docs"
              "frontend"
              "rfcs"
              "scripts"
              "setup"
              "subsquid"
              "runtime-tests"
              "composablejs"
            ];
            file-blacklist = [ "flake.nix" "flake.lock" ];
          in lib.cleanSourceWith {
            filter = lib.cleanSourceFilter;
            src = lib.cleanSourceWith {
              filter = let
                customFilter = name: type:
                  (!(type == "directory"
                    && builtins.elem (baseNameOf name) dir-blacklist))
                  && (!(type == "file"
                    && builtins.elem (baseNameOf name) file-blacklist));
              in nix-gitignore.gitignoreFilterPure customFilter [ ./.gitignore ]
              ./.;
              src = ./.;
            };
          };

          # Common env required to build the node
          common-attrs = {
            src = rust-src;
            buildInputs = [ openssl zstd ];
            nativeBuildInputs = [ clang pkg-config ]
              ++ lib.optional stdenv.isDarwin
              (with darwin.apple_sdk.frameworks; [
                Security
                SystemConfiguration
              ]);
            doCheck = false;
            cargoCheckCommand = "true";
            # Don't build any wasm as we do it ourselves
            SKIP_WASM_BUILD = "1";
            LD_LIBRARY_PATH = lib.strings.makeLibraryPath [
              stdenv.cc.cc.lib
              llvmPackages.libclang.lib
            ];
            LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
            PROTOC = "${protobuf}/bin/protoc";
            ROCKSDB_LIB_DIR = "${rocksdb}/lib";
          };

          # Common dependencies, all dependencies listed that are out of this repo
          common-deps = crane-stable.buildDepsOnly (common-attrs // { });
          common-deps-nightly =
            crane-nightly.buildDepsOnly (common-attrs // { });
          common-attrs-with-benchmarks = common-attrs // {
            cargoExtraArgs =
              "--features=runtime-benchmarks --features=builtin-wasm";
          };
          common-deps-with-benchmarks =
            crane-stable.buildDepsOnly common-attrs-with-benchmarks;

          # Build a wasm runtime, unoptimized
          mk-runtime = name:
            let file-name = "${name}_runtime.wasm";
            in crane-nightly.buildPackage (common-attrs // {
              pname = "${name}-runtime";
              cargoArtifacts = common-deps-nightly;
              cargoBuildCommand =
                "cargo build --release -p ${name}-runtime-wasm --target wasm32-unknown-unknown";
              # From parity/wasm-builder
              RUSTFLAGS =
                "-Clink-arg=--export=__heap_base -Clink-arg=--import-memory";
            });

          # Derive an optimized wasm runtime from a prebuilt one, garbage collection + compression
          mk-optimized-runtime = name:
            let runtime = mk-runtime name;
            in stdenv.mkDerivation {
              name = "${runtime.name}-optimized";
              phases = [ "installPhase" ];
              installPhase = ''
                mkdir -p $out/lib
                ${wasm-optimizer}/bin/wasm-optimizer \
                  --input ${runtime}/lib/${name}_runtime.wasm \
                  --output $out/lib/runtime.optimized.wasm
              '';
            };

          # https://cloud.google.com/iam/docs/creating-managing-service-account-keys
          # or just use GOOGLE_APPLICATION_CREDENTIALS env as path to file
          service-account-credential-key-file-input =
            builtins.fromJSON (builtins.readFile ./devnet/ops.json);

          gce-to-nix = file: {
            project = file.project_id;
            serviceAccount = file.client_email;
            accessKey = file.private_key;
          };

          gce-input = gce-to-nix service-account-credential-key-file-input;

          devcontainer-base-image =
            pkgs.callPackage ./.nix/devcontainer-base-image.nix {
              inherit system;
            };

          dali-runtime = mk-optimized-runtime "dali";
          picasso-runtime = mk-optimized-runtime "picasso";
          composable-runtime = mk-optimized-runtime "composable";

          # NOTE: with docs, non nighly fails but nighly fails too...
          # /nix/store/523zlfzypzcr969p058i6lcgfmg889d5-stdenv-linux/setup: line 1393: --message-format: command not found
          composable-node = with packages;
            crane-stable.buildPackage (common-attrs // {
              pnameSuffix = "-node";
              cargoArtifacts = common-deps;
              cargoBuildCommand =
                "cargo build --release --package composable --features builtin-wasm";
              DALI_RUNTIME = "${dali-runtime}/lib/runtime.optimized.wasm";
              PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
              COMPOSABLE_RUNTIME =
                "${composable-runtime}/lib/runtime.optimized.wasm";
              installPhase = ''
                mkdir -p $out/bin
                cp target/release/composable $out/bin/composable
              '';
            });

          composable-node-with-benchmarks = crane-stable.cargoBuild
            (common-attrs-with-benchmarks // {
              pnameSuffix = "-node";
              cargoArtifacts = common-deps-with-benchmarks;
              cargoBuildCommand = "cargo build --release --package composable";
              DALI_RUNTIME = "${dali-runtime}/lib/runtime.optimized.wasm";
              PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
              COMPOSABLE_RUNTIME =
                "${composable-runtime}/lib/runtime.optimized.wasm";
              installPhase = ''
                mkdir -p $out/bin
                cp target/release/composable $out/bin/composable
              '';
            });

          run-with-benchmarks = chain:
            pkgs.writeShellScriptBin "run-benchmarks-once" ''
              ${composable-node-with-benchmarks}/bin/composable benchmark pallet \
                --chain="${chain}" \
                --execution=wasm \
                --wasm-execution=compiled \
                --wasm-instantiation-strategy=legacy-instance-reuse \
                --pallet="*" \
                --extrinsic='*' \
                --steps=1 \
                --repeat=1 \
                --output=$out \
                --log error
            '';

          mk-devnet =
            { polkadot-launch, composable-node, polkadot-node, chain-spec }:
            let
              original-config = import ./scripts/polkadot-launch/composable.nix;
              patched-config = lib.recursiveUpdate original-config {
                relaychain = { bin = "${polkadot-node}/bin/polkadot"; };
                parachains = builtins.map (parachain:
                  parachain // {
                    bin = "${composable-node}/bin/composable";
                    chain = "${chain-spec}";
                  }) original-config.parachains;
              };
              config = writeTextFile {
                name = "devnet-${chain-spec}-config.json";
                text = builtins.toJSON patched-config;
              };
            in {
              inherit chain-spec;
              parachain-nodes = builtins.map (parachain: parachain.nodes)
                patched-config.parachains;
              relaychain-nodes = patched-config.relaychain.nodes;
              script = writeShellScript "run-devnet-${chain-spec}" ''
                rm -rf /tmp/polkadot-launch
                ${polkadot-launch}/bin/polkadot-launch ${config} --verbose
              '';
            };

        in rec {
          nixopsConfigurations = {
            default = (pkgs.callPackage ./.nix/devnet.nix {
              inherit nixpkgs;
              inherit gce-input;
              inherit (packages) devnet-dali devnet-picasso;
              book = packages.composable-book;
            });
          };

          packages = rec {
            inherit wasm-optimizer;
            inherit common-deps;
            inherit dali-runtime;
            inherit picasso-runtime;
            inherit composable-runtime;
            inherit composable-node;

            runtime-tests = stdenv.mkDerivation {
              name = "runtime-tests";
              src = builtins.filterSource
                (path: type: baseNameOf path != "node_modules")
                ./integration-tests/runtime-tests;
              dontUnpack = true;
              installPhase = ''
                mkdir $out/
                cp -r $src/* $out/
              '';
            };

            price-feed = crane-stable.buildPackage (common-attrs // {
              pnameSuffix = "-price-feed";
              cargoArtifacts = common-deps;
              cargoBuildCommand = "cargo build --release -p price-feed";
            });

            taplo = pkgs.callPackage ./.nix/taplo.nix { inherit crane-stable; };

            mdbook =
              pkgs.callPackage ./.nix/mdbook.nix { inherit crane-stable; };

            composable-book = import ./book/default.nix {
              crane = crane-stable;
              inherit (pkgs) cargo stdenv;
              inherit mdbook;
            };

            # NOTE: crane can't be used because of how it vendors deps, which is incompatible with some packages in polkadot, an issue must be raised to the repo
            polkadot-node = rustPlatform.buildRustPackage rec {
              # HACK: break the nix sandbox so we can build the runtimes. This
              # requires Nix to have `sandbox = relaxed` in its config.
              # We don't realy care because polkadot is only used for local devnet.
              __noChroot = true;
              name = "polkadot-v${version}";
              version = "0.9.24";
              src = pkgs.fetchFromGitHub {
                repo = "polkadot";
                owner = "paritytech";
                rev = "v${version}";
                hash = "sha256-Vv8lnmGNdhKjMGmzBJVJvmR2rD3BsbaDD7LajkKxpXc=";
              };
              cargoSha256 =
                "sha256-53iEC0WQy/tkToTDqolXzR6sjfe2xolBlIjQXDGhsYc=";
              doCheck = false;
              buildInputs = [ openssl zstd ];
              nativeBuildInputs = [ rust-nightly clang pkg-config ]
                ++ lib.optional stdenv.isDarwin
                (with darwin.apple_sdk.frameworks; [
                  Security
                  SystemConfiguration
                ]);
              LD_LIBRARY_PATH = lib.strings.makeLibraryPath [
                stdenv.cc.cc.lib
                llvmPackages.libclang.lib
              ];
              LIBCLANG_PATH = "${llvmPackages.libclang.lib}/lib";
              PROTOC = "${protobuf}/bin/protoc";
              ROCKSDB_LIB_DIR = "${rocksdb}/lib";
            };

            polkadot-launch =
              pkgs.callPackage ./scripts/polkadot-launch/polkadot-launch.nix
              { };

            # Dali devnet
            devnet-dali = mk-devnet {
              inherit (packages) polkadot-launch composable-node polkadot-node;
              chain-spec = "dali-dev";
            };

            # Picasso devnet
            devnet-picasso = mk-devnet {
              inherit (packages) polkadot-launch composable-node polkadot-node;
              chain-spec = "picasso-dev";
            };

            # Dali devnet container
            devnet-container = dockerTools.buildLayeredImage {
              name = "composable-devnet-container";
              contents = [
                # just allow to bash into it and run with custom entries
                packages.devnet-dali.script
                packages.polkadot-launch
              ] ++ container-tools;
              config = { Cmd = [ "${packages.devnet-dali.script}" ]; };
            };

            # TODO: inherit and provide script to run all stuff
            # devnet-container-xcvm
            devcontainer = dockerTools.buildLayeredImage {
              name = "composable-devcontainer";
              fromImage = devcontainer-base-image;
              # be very carefull with this, so this must be version compatible with base and what vscode will inject
              contents = [
                # ISSUE: for some reason stable overrides nighly, need to set different order somehow
                #rust-stable
                rust-nightly-dev
                cachix
                rust-analyzer
                rustup # just if it wants to make ad hoc updates
                nix
                helix
                clang
                nodejs
                cmake
                nixpkgs-fmt
                yarn
                bottom
                packages.mdbook
                packages.taplo
                go
                libclang 
                gcc 
                openssl
                gnumake
                pkg-config
              ];
            };

            default = packages.composable-node;
          };

          # Derivations built when running `nix flake check`
          # TODO: pass --argstr and depending on it enable only part of checks (unit tests, local simulator tests, benches), and ensure existing run in parallel
          # TODO: because test runs are long
          checks = {
            # TODO: how to avoid run some tests? simpltes is read workspace, get all members, and filter out by mask integration
            tests = crane-stable.cargoBuild (common-attrs // {
              pnameSuffix = "-tests";
              doCheck = true;
              cargoArtifacts = common-deps;
              # NOTE: do not add --features=runtime-benchmarks because it force multi ED to be 0 because of dependencies
              # NOTE: in order to run benchmarks as tests, just make `any(test, feature = "runtime-benchmarks")
              cargoBuildCommand =
                "cargo test --workspace --release --locked --verbose";
            });

            dali-dev-benchmarks = run-with-benchmarks "dali-dev";
            picasso-dev-benchmarks = run-with-benchmarks "picasso-dev";
            composable-dev-benchmarks = run-with-benchmarks "composable-dev";

            picasso-integration-tests = crane-nightly.cargoBuild (common-attrs
              // {
                pname = "local-integration-tests";
                cargoArtifacts = common-deps-nightly;
                doCheck = true;
                cargoBuildCommand =
                  "cargo test --package local-integration-tests";
                cargoExtraArgs =
                  "--features local-integration-tests --features picasso --features std --no-default-features";
              });
            dali-integration-tests = crane-nightly.cargoBuild (common-attrs // {
              pname = "local-integration-tests";
              doCheck = true;
              cargoArtifacts = common-deps-nightly;
              cargoBuildCommand =
                "cargo test --package local-integration-tests";
              cargoExtraArgs =
                "--features local-integration-tests --features dali --features std --no-default-features";
            });
          };

          devShells = rec {
            developers = mkShell {
              inputsFrom = builtins.attrValues self.checks;
              buildInputs = with packages; [
                # with nix developers are empowered for local dry run of most ci
                rust-stable
                wasm-optimizer
                composable-node
                mdbook
                taplo
                python3
                nodejs
                nixpkgs-fmt
                nixops
                jq
                google-cloud-sdk # devs can list container images or binary releases
              ];
              NIX_PATH = "nixpkgs=${pkgs.path}";
              HISTFILE = toString ./.history;
            };

            technical-writers = mkShell {
              buildInputs = with packages; [
                mdbook
                python3
                plantuml
                graphviz
                pandoc
              ];
              NIX_PATH = "nixpkgs=${pkgs.path}";
            };

            sre = developers.overrideAttrs
              (oldAttrs: rec { buildInputs = oldAttrs.buildInputs ++ [ ]; });

            # developers-xcvm = developers // mkShell {
            #   buildInputs = with packages; [
            #     # TODO: hasura
            #     # TODO: junod client
            #     # TODO: junod server
            #     # TODO: solc
            #     # TODO: script to run all
            #     # TODO: compose export
            #       packages.dali-script
            #   ];
            #   NIX_PATH = "nixpkgs=${pkgs.path}";
            # };

            default = developers;
          };

          # Applications runnable with `nix run`
          # https://github.com/NixOS/nix/issues/5560
          apps = rec {
            devnet-dali = {
              type = "app";
              program = "${packages.devnet-dali.script}";
            };
            devnet-picasso = {
              type = "app";
              program = "${packages.devnet-picasso.script}";
            };
            price-feed = {
              type = "app";
              program = "${packages.price-feed}/bin/price-feed";
            };
            composable = {
              type = "app";
              program = "${packages.composable-node}/bin/composable";
            };
            polkadot = {
              type = "app";
              program = "${packages.polkadot-node}/bin/polkadot";
            };
            # TODO: move list of chains out of here and do fold
            benchmarks-once-composable = flake-utils.lib.mkApp {
              drv = run-with-benchmarks "composable-dev";
            };
            benchmarks-once-dali =
              flake-utils.lib.mkApp { drv = run-with-benchmarks "dali-dev"; };
            benchmarks-once-picasso = flake-utils.lib.mkApp {
              drv = run-with-benchmarks "picasso-dev";
            };
            default = devnet-dali;
          };
        });
    in eachSystemOutputs // {
      nixopsConfigurations = {
        # instead of rerusive merge, just do simpler
        x86_64-linux.default =
          eachSystemOutputs.nixopsConfigurations.x86_64-linux.default;
        default = eachSystemOutputs.nixopsConfigurations.x86_64-linux.default;
      };
    };
}
