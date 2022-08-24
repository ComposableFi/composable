{
  # see ./docs/nix.md for design guidelines of nix organization
  description = "Composable Finance systems, tools and releases";
  # when flake runs, ask for interactie answers first time
  # nixConfig.sandbox = "relaxed";
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
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
  };
  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay }:
    let
      # https://cloud.google.com/iam/docs/creating-managing-service-account-keys
      # or just use GOOGLE_APPLICATION_CREDENTIALS env as path to file
      service-account-credential-key-file-input =
        builtins.fromJSON (builtins.readFile ./devnet/ops.json);

      gce-to-nix = { project_id, client_email, private_key }: {
        project = project_id;
        serviceAccount = client_email;
        accessKey = private_key;
      };

      gce-input = gce-to-nix service-account-credential-key-file-input;

      mk-devnet =
        { pkgs
        , lib
        , writeTextFile
        , writeShellApplication
        , polkadot-launch
        , composable-node
        , polkadot-node
        , chain-spec
        }:
        let
          original-config = (pkgs.callPackage
            ./scripts/polkadot-launch/rococo-local-dali-dev.nix
            {
              polkadot-bin = polkadot-node;
              composable-bin = composable-node;
            }).result;

          patched-config = lib.recursiveUpdate original-config {
            parachains = builtins.map
              (parachain: parachain // { chain = "${chain-spec}"; })
              original-config.parachains;
          };
          config = writeTextFile {
            name = "devnet-${chain-spec}-config.json";
            text = builtins.toJSON patched-config;
          };
        in
        {
          inherit chain-spec;
          parachain-nodes = builtins.concatMap (parachain: parachain.nodes)
            patched-config.parachains;
          relaychain-nodes = patched-config.relaychain.nodes;
          script = writeShellApplication {
            name = "run-devnet-${chain-spec}";
            text = ''
              rm -rf /tmp/polkadot-launch
              ${polkadot-launch}/bin/polkadot-launch ${config} --verbose
            '';
          };
        };

      eachSystemOutputs = flake-utils.lib.eachDefaultSystem (system:
        let
          pkgs = import nixpkgs {
            inherit system;
            overlays = [ rust-overlay.overlays.default ];
            allowUnsupportedSystem = true; # we do not tirgger this on mac
            config = {
              permittedInsecurePackages = [
                "openjdk-headless-16+36"
                "openjdk-headless-15.0.1-ga"
                "openjdk-headless-14.0.2-ga"
                "openjdk-headless-13.0.2-ga"
              ];
            };
          };
          overlays = [ rust-overlay.overlay ];
          rust-toolchain = import ./.nix/rust-toolchain.nix;
        in
        with pkgs;
        let
          # Stable rust for anything except wasm runtime
          rust-stable = rust-bin.stable.latest.default;

          # Nightly rust used for wasm runtime compilation
          rust-nightly = rust-bin.selectLatestNightlyWith (toolchain:
            toolchain.default.override {
              extensions = [ "rust-src" "clippy" "rustfmt" "rust-analyzer" ];
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
          rust-src =
            let
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
            in
            lib.cleanSourceWith {
              filter = lib.cleanSourceFilter;
              src = lib.cleanSourceWith {
                filter =
                  let
                    customFilter = name: type:
                      (
                        !(type == "directory"
                        && builtins.elem (baseNameOf name) dir-blacklist)
                      )
                      && (
                        !(type == "file"
                        && builtins.elem (baseNameOf name) file-blacklist)
                      );
                  in
                  nix-gitignore.gitignoreFilterPure customFilter [ ./.gitignore ]
                    ./.;
                src = ./.;
              };
            };

          # Common env required to build the node
          common-attrs = {
            src = rust-src;
            buildInputs = [ openssl zstd ];
            nativeBuildInputs = [ clang openssl pkg-config ]
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
          common-bench-attrs = common-attrs // {
            cargoExtraArgs = "--features=builtin-wasm,runtime-benchmarks";
          };
          common-bench-deps =
            crane-stable.buildDepsOnly (common-bench-attrs // { });

          # Build a wasm runtime, unoptimized
          mk-runtime = name: features:
            let file-name = "${name}_runtime.wasm";
            in crane-nightly.buildPackage (common-attrs // {
              pname = "${name}-runtime";
              cargoArtifacts = common-deps-nightly;
              cargoBuildCommand =
                "cargo build --release -p ${name}-runtime-wasm --target wasm32-unknown-unknown" + lib.strings.optionalString (features != "") (" --features=${features}");
              # From parity/wasm-builder
              RUSTFLAGS =
                "-Clink-arg=--export=__heap_base -Clink-arg=--import-memory";
            });

          # Derive an optimized wasm runtime from a prebuilt one, garbage collection + compression
          mk-optimized-runtime = { name, features ? "" }:
            let runtime = mk-runtime name features;
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

          devcontainer-base-image =
            callPackage ./.nix/devcontainer-base-image.nix { inherit system; };

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

          # NOTE: with docs, non nighly fails but nighly fails too...
          # /nix/store/523zlfzypzcr969p058i6lcgfmg889d5-stdenv-linux/setup: line 1393: --message-format: command not found
          composable-node = with packages;
            crane-stable.buildPackage (common-attrs // {
              pnameSuffix = "-node";
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
            });

          composable-bench-node = crane-stable.cargoBuild
            (common-bench-attrs // {
              pnameSuffix = "-node";
              cargoArtifacts = common-bench-deps;
              cargoBuildCommand = "cargo build --release --package composable";
              DALI_RUNTIME = "${dali-bench-runtime}/lib/runtime.optimized.wasm";
              PICASSO_RUNTIME = "${picasso-bench-runtime}/lib/runtime.optimized.wasm";
              COMPOSABLE_RUNTIME =
                "${composable-bench-runtime}/lib/runtime.optimized.wasm";
              installPhase = ''
                mkdir -p $out/bin
                cp target/release/composable $out/bin/composable
              '';
            });

          run-with-benchmarks = chain:
            writeShellScriptBin "run-benchmarks-once" ''
              ${composable-bench-node}/bin/composable benchmark pallet \
                --chain="${chain}" \
                --execution=wasm \
                --wasm-execution=compiled \
                --wasm-instantiation-strategy=legacy-instance-reuse \
                --pallet="*" \
                --extrinsic="*" \
                --steps=1 \
                --repeat=1
            '';
          docs-renders = [
                mdbook
                plantuml
                graphviz
                pandoc
          ];

        in
        rec {
          packages = rec {
            inherit wasm-optimizer;
            inherit common-deps;
            inherit common-bench-deps;
            inherit dali-runtime;
            inherit picasso-runtime;
            inherit composable-runtime;
            inherit dali-bench-runtime;
            inherit picasso-bench-runtime;
            inherit composable-bench-runtime;
            inherit composable-node;
            inherit composable-bench-node;

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

            composable-book = import ./book/default.nix {
              crane = crane-stable;
              inherit cargo stdenv;
              inherit mdbook;
            };

            # NOTE: crane can't be used because of how it vendors deps, which is incompatible with some packages in polkadot, an issue must be raised to the repo
            acala-node = pkgs.callPackage ./.nix/acala-bin.nix {
              rust-overlay = rust-nightly;
            };
            polkadot-node = rustPlatform.buildRustPackage rec {
              # HACK: break the nix sandbox so we can build the runtimes. This
              # requires Nix to have `sandbox = relaxed` in its config.
              # We don't realy care because polkadot is only used for local devnet.
              __noChroot = true;
              name = "polkadot-v${version}";
              version = "0.9.27";
              src = fetchFromGitHub {
                repo = "polkadot";
                owner = "paritytech";
                rev = "v${version}";
                hash = "sha256-LEz3OrVgdFTCnVwzU8C6GeEougaOl2qo7jS9qIdMqAM=";
              };
              cargoSha256 =
                "sha256-6y+WK2k1rhqMxMjEJhzJ26WDMKZjXQ+q3ca2hbbeLvA=";
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
              callPackage ./scripts/polkadot-launch/polkadot-launch.nix { };

            # Dali devnet
            devnet-dali = (callPackage mk-devnet {
              inherit pkgs;
              inherit (packages) polkadot-launch composable-node polkadot-node;
              chain-spec = "dali-dev";
            }).script;

            # Picasso devnet
            devnet-picasso = (callPackage mk-devnet {
              inherit pkgs;
              inherit (packages) polkadot-launch composable-node polkadot-node;
              chain-spec = "picasso-dev";
            }).script;

            # Dali devnet container
            devnet-container = dockerTools.buildLayeredImage {
              name = "composable-devnet-container";
              contents = container-tools;
              config = {
                Cmd = [ "${packages.devnet-dali}/bin/run-devnet-dali-dev" ];
              };
            };

            # TODO: inherit and provide script to run all stuff
            # devnet-container-xcvm
            # NOTE: The devcontainer is currently broken for aarch64.
            # Please use the developers devShell instead
            devcontainer = dockerTools.buildLayeredImage {
              name = "composable-devcontainer";
              fromImage = devcontainer-base-image;
              # be very carefull with this, so this must be version compatible with base and what vscode will inject
              contents = [
                # ISSUE: for some reason stable overrides nighly, need to set different order somehow
                #rust-stable
                rust-nightly
                cachix
                rustup # just if it wants to make ad hoc updates
                nix
                helix
                clang
                nodejs
                cmake
                nixpkgs-fmt
                yarn
                bottom
                mdbook
                taplo
                go
                libclang
                gcc
                openssl
                gnumake
                pkg-config
              ];
            };

            check-dali-dev-benchmarks = run-with-benchmarks "dali-dev";
            check-picasso-dev-benchmarks = run-with-benchmarks "picasso-dev";
            check-composable-dev-benchmarks =
              run-with-benchmarks "composable-dev";

            check-picasso-integration-tests = crane-nightly.cargoBuild
              (common-attrs // {
                pname = "picasso-local-integration-tests";
                cargoBuildCommand =
                  "cargo test --package local-integration-tests";
                cargoExtraArgs =
                  "--features=local-integration-tests,picasso,std --no-default-features --verbose";
              });
            check-dali-integration-tests = crane-nightly.cargoBuild
              (common-attrs // {
                pname = "dali-local-integration-tests";
                cargoBuildCommand =
                  "cargo test --package local-integration-tests";
                cargoExtraArgs =
                  "--features=local-integration-tests,dali,std --no-default-features --verbose";
              });

            unit-tests = crane-stable.cargoBuild (common-attrs // {
              pnameSuffix = "-tests";
              cargoArtifacts = common-deps;
              # NOTE: do not add --features=runtime-benchmarks because it force multi ED to be 0 because of dependencies
              # NOTE: in order to run benchmarks as tests, just make `any(test, feature = "runtime-benchmarks")
              cargoBuildCommand =
                "cargo test --workspace --release --locked --verbose";
            });

            kusama-picasso-karura-devnet =
              let
                config = (pkgs.callPackage
                  ./scripts/polkadot-launch/kusama-local-picasso-dev-karura-dev.nix
                  {
                    polkadot-bin = polkadot-node;
                    composable-bin = composable-node;
                    acala-bin = acala-node;
                  }).result;
                config-file = writeTextFile {
                  name = "kusama-local-picasso-dev-karura-dev.json";
                  text = "${builtins.toJSON config}";
                };
              in
              writeShellApplication {
                name = "kusama-picasso-karura";
                text = ''
                  cat ${config-file}
                  ${packages.polkadot-launch}/bin/polkadot-launch ${config-file} --verbose
                '';
              };

            junod = pkgs.callPackage ./xcvm/cosmos/junod.nix { };
            gex = pkgs.callPackage ./xcvm/cosmos/gex.nix { };
            wasmswap = pkgs.callPackage ./xcvm/cosmos/wasmswap.nix { crane = crane-nightly;  };
            default = packages.composable-node;
          };

          devShells = rec {
            developers = developers-minimal.overrideAttrs (base: {
              buildInputs = with packages;
                base.buildInputs ++ [
                  bacon
                  google-cloud-sdk
                  grub2
                  jq
                  lldb
                  llvmPackages_latest.bintools
                  llvmPackages_latest.lld
                  llvmPackages_latest.llvm
                  mdbook
                  nix-tree
                  nixpkgs-fmt
                  openssl
                  openssl.dev
                  pkg-config
                  qemu
                  rnix-lsp
                  rust-nightly
                  taplo
                  wasm-optimizer
                  xorriso
                  zlib.out
                  nix-tree
                ] ++ docs-renders;
            });

            developers-minimal = mkShell (common-attrs // {
              buildInputs = with packages; [ rust-nightly ];
              NIX_PATH = "nixpkgs=${pkgs.path}";
            });

            developers-xcvm = developers-minimal.overrideAttrs (base: {
              buildInputs = with packages;
                base.buildInputs ++
                [
                  junod
                  gex
                  # junod wasm swap web interface 
                  devnet-dali
                  # TODO: hasura
                  # TODO: some well know wasm contracts deployed                
                  # TODO: junod server
                  # TODO: solc
                  # TODO: gex
                  # TODO: https://github.com/forbole/bdjuno                  
                  # TODO: script to run all
                  # TODO: compose export
                ]
              ++ lib.lists.optional (lib.strings.hasSuffix "linux" system) arion;
              shellHook = ''
                # TODO: how to make it work - setup defaul admin client key
                #"clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose" > junod keys add alice --recover 
              '';

            });

            writers = mkShell {
              buildInputs = with packages; [
                python3
                nodejs
              ] ++ doc-renders;
              NIX_PATH = "nixpkgs=${pkgs.path}";
            };

            sre = mkShell {
              buildInputs = [ nixopsUnstable ];
              NIX_PATH = "nixpkgs=${pkgs.path}";
            };

            default = developers;
          };

          # Applications runnable with `nix run`
          # https://github.com/NixOS/nix/issues/5560
          apps = rec {
            devnet-xcvm-up =
              let
                devnet-xcvm =
                  pkgs.arion.build
                    {
                      modules = [
                        ({ pkgs, ... }: {
                          config = {
                            project = {
                              name = "devnet-xcvm";
                            };
                            services = {
                              junod-testing-local = {
                                service = {
                                  name = "junod-testing-local";
                                  # NOTE: the do not release git hash tags, so not clear how to share client and docker image
                                  image = "ghcr.io/cosmoscontracts/juno:v9.0.0";
                                  environment = {
                                    STAKE_TOKEN = "ujunox";
                                    UNSAFE_CORS = "true";
                                    USER = "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y";
                                    GAS_LIMIT = 100000000;
                                  };
                                  # TODO: mount proper genesis here as per
                                  # "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose" > junod keys add alice --recover
                                  # `"wasm":{"codes":[],"contracts":[],"gen_msgs":[],"params":{"code_upload_access":{"address":"","permission":"Everybody"},`
                                  #network_mode 
                                  command = ''                                     
                                    ./setup_and_run.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
                                  '';
                                  #network_mode = "host";
                                  # these ports are open by default
                                  ports = [
                                      "1317:1317" # rest openapi
                                      "26656:26656" # p2p
                                      "26657:26657" # rpc json-rpc
                                  ];
                                };
                              };
                            };
                          };
                        })
                      ];
                      inherit pkgs;
                    }
                ;
              in
              {
                type = "app";
                program = "${pkgs.writeShellScript "arion-up" ''
                ${pkgs.arion}/bin/arion --prebuilt-file ${devnet-xcvm} up --remove-orphans
              ''}";
              };

            devnet-dali = {
              type = "app";
              program = "${packages.devnet-dali}/bin/run-devnet-dali-dev";
            };
            devnet-picasso = {
              type = "app";
              program =
                "${packages.devnet-picasso.script}/bin/run-devnet-picasso-dev";
            };

            kusama-picasso-karura-devnet = {
              # nix run .#devnet
              type = "app";
              program =
                "${packages.kusama-picasso-karura-devnet}/bin/kusama-picasso-karura";
            };

            price-feed = {
              type = "app";
              program = "${packages.price-feed}/bin/price-feed";
            };
            composable = {
              type = "app";
              program = "${packages.composable-node}/bin/composable";
            };
            acala = {
              type = "app";
              program = "${packages.acala-node}/bin/acala";
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
    in
    eachSystemOutputs // {
      nixopsConfigurations = {
        default =
          let pkgs = nixpkgs.legacyPackages.x86_64-linux;
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
            book = eachSystemOutputs.packages.x86_64-linux.composable-book;
          };
      };
    };
}
