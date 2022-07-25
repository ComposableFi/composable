{
  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils = {
      url = "github:numtide/flake-utils";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ rust-overlay.overlays.default ];
        };
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

        # Wasm optimizer, used to replicate build.rs behavior in an explicit fashion
        wasm-optimizer = crane-stable.buildPackage {
          cargoCheckCommand = "true";
          src = let customFilter = name: type: true;
          in lib.cleanSourceWith {
            filter = lib.cleanSourceFilter;
            src = lib.cleanSourceWith {
              filter =
                nix-gitignore.gitignoreFilterPure customFilter [ ../.gitignore ]
                ../utils/wasm-optimizer;
              src = ../utils/wasm-optimizer;
            };
          };
        };

        # for containers which are intented for testing, debug and development (including running isolated runtime)
        container-tools =     
        [
          coreutils
          bash
          procps
          findutils
          nettools
          bottom
          nix   
          procps
        ];

        src = let
          blacklist = [
            "nix"
            "nix-crane"
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
          ];
          customFilter = name: type:
            !(type == "directory" && builtins.elem (baseNameOf name) blacklist);
        in lib.cleanSourceWith {
          filter = lib.cleanSourceFilter;
          src = lib.cleanSourceWith {
            filter =
              nix-gitignore.gitignoreFilterPure customFilter [ ../.gitignore ]
              ../.;
            src = ../.;
          };
        };

        # Common env required to build the node
        common-args = {
          inherit src;
          buildInputs = [ openssl zstd ];
          nativeBuildInputs = [ clang pkg-config ]
            ++ lib.optional stdenv.isDarwin (with darwin.apple_sdk.frameworks; [
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
        common-deps = crane-stable.buildDepsOnly (common-args // { });

        # Build a wasm runtime, unoptimized
        mk-runtime = name:
          let file-name = "${name}_runtime.wasm";
          in crane-nightly.buildPackage (common-args // {
            pname = "${name}-runtime";
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

        # Current polkadot version
        polkadot = {
          version = "0.9.24";
          hash = "sha256-k+6mXjAsHbS4gnHljGmEkMcok77zBd8jhyp56mXyKgI=";
        };

        nix-to-container-image = import ./nix-to-container-image.nix;
      in rec {
        packages = {
          inherit wasm-optimizer;
          inherit common-deps;
          dali-runtime = mk-optimized-runtime "dali";
          picasso-runtime = mk-optimized-runtime "picasso";
          composable-runtime = mk-optimized-runtime "composable";
          price-feed = crane-stable.buildPackage (common-args // {
            pnameSuffix = "-price-feed";
            cargoArtifacts = common-deps;
            cargoBuildCommand = "cargo build --release -p price-feed";
          });
          composable-node = with packages;
            crane-stable.buildPackage (common-args // {
              pnameSuffix = "-node";
              cargoArtifacts = common-deps;
              cargoBuildCommand =
                "cargo build --release -p composable --features builtin-wasm";
              DALI_RUNTIME = "${dali-runtime}/lib/runtime.optimized.wasm";
              PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
              COMPOSABLE_RUNTIME =
                "${composable-runtime}/lib/runtime.optimized.wasm";
              installPhase = ''
                mkdir -p $out/bin
                cp target/release/composable $out/bin/composable
              '';
            });

          # also mdbook has releases for all targets,
          # so it simple to build it as it is rust
          # and also we then can have fork easy
          # and nix allows to use any version of rust
          # so if will need to special docs for substrate, we will have it
          # and, anyway cargo install also compiles
          mdbook = with packages; 
            crane-stable.buildPackage {
              src = fetchFromGitHub {
                owner = "rust-lang";
                repo = "mdBook";
                rev = "40c06f5e774924bef97d339cf8c64343c9056d86";
                hash = "sha256-ggcyOsA4cyo5l87cZmOMI0w1gCzmWy9NRJiWxjBdB1E=";
              };          
            };

          taplo = with packages; 
            crane-stable.buildPackage {
              src = fetchFromGitHub {
                owner = "tamasfe";
                repo = "taplo";
                rev = "eeb62dcbada89f13de73cfc063ffe67a890c4bc6";
                hash = "sha256-ggcyOsA4cyo5l87cZmOMI0w1gCzmWy9NRJiWxjBdB1E=";
              };          
            };            

          polkadot-node = stdenv.mkDerivation {
            name = "polkadot-${polkadot.version}";
            version = polkadot.version;
            src = fetchurl {
              url =
                "https://github.com/paritytech/polkadot/releases/download/v${polkadot.version}/polkadot";
              sha256 = polkadot.hash;
            };
            nativeBuildInputs = [ pkgs.autoPatchelfHook ];
            buildInputs = [ pkgs.stdenv.cc.cc ];
            dontUnpack = true;
            installPhase = ''
              mkdir -p $out/bin
              cp $src $out/bin/polkadot
              chmod +x $out/bin/polkadot
            '';
          };

          polkadot-launch = let
            src = fetchFromGitHub {
              owner = "paritytech";
              repo = "polkadot-launch";
              rev = "951af7055e2c9abfa7a03ee7848548c1a3efdc16";
              hash = "sha256-ZaCHgkr5lVsGFg/Yvx6QY/zSiIafwSec+oiioOWTZMg=";
            };
          in mkYarnPackage {
            name = "polkadot-launch";
            inherit src;
            packageJSON = "${src}/package.json";
            yarnLock = "${src}/yarn.lock";
            buildPhase = ''
              yarn build
            '';
            distPhase = "true";
            postInstall = ''
              chmod +x $out/bin/polkadot-launch
            '';
          };
          devnet = let
            original-config = import ./composable.nix;
            patched-config = lib.recursiveUpdate original-config {
              relaychain = { bin = "${packages.polkadot-node}/bin/polkadot"; };
              parachains = builtins.map (parachain:
                parachain // {
                  bin = "${packages.composable-node}/bin/composable";
                }) original-config.parachains;
            };
            config = writeTextFile {
              name = "devnet-config.json";
              text = "${builtins.toJSON patched-config}";
            };
          in writeShellApplication {
            name = "composable-devnet";
            text = ''
              rm -rf /tmp/polkadot-launch
              ${packages.polkadot-launch}/bin/polkadot-launch ${config} --verbose
            '';
          };
          devnet-container = dockerTools.buildLayeredImage {
            name = "composable-devnet-container";
            contents = [                                     
              # just allow to bash into it and run with custom entries
              packages.devnet
              packages.polkadot-launch
              ] ++ container-tools;
              config = { Cmd = [ "${packages.devnet}/bin/composable-devnet" ]; };
          };
          
          # image which will be base for remote development
          # we do not start from nixos:
          # - no all people like devcontainer to be nix (gh know better)
          # - devcontainer has setup in shell for code, users, groups and remote stuff
          # - it has nice cli/shell setup, unlike bare nixos docker        
          # we want devcontainer to be built of nix:
          # - so it has same version or rust as our env and ci
          # - it has same all tooling we have
          # - and we do not need to maintain separate script for that
          codespace-base-container = dockerTools.pullImage ((nix-to-container-image system) // {
            imageName = "mcr.microsoft.com/vscode/devcontainers/base";
            os = "linux";
            imageDigest = "sha256:269cbbb2056243e2a88e21501d9a8166d1825d42abf6b67846b49b1856f4b133";
            sha256 = "0vraf6iwbddpcy4l9msks6lmi35k7wfgpafikb56k3qinvvcjm9b";
            finalImageTag = "0.202.7-bullseye";             
          });
          
          codespace-container = dockerTools.buildLayeredImage {
            name = "composable-codespace";
            fromImage = packages.codespace-base-container;
            contents = [                                          
              # be very carefull with this, so this must be version compatible with base and what vscode will inject
              # ISSUE: for some reason stable overrides nighly, need to set different order somehow
              #rust-stable
              rust-nightly-dev
              cachix
              rust-analyzer
              rustup # just if it wants to make ad hoc updates
              nodejs
              bottom
              packages.mdbook
              packages.taplo
              ];
          };
          default = packages.composable-node;
        };

        # Derivations built when running `nix flake check`
        checks = {
          tests = crane-stable.cargoBuild (common-args // {
            pnameSuffix = "-tests";
            cargoArtifacts = common-deps;
            cargoBuildCommand = "cargo test --workspace --release";
          });
        };

        # Applications runnable with `nix run`
        apps = {
          # nix run .#devnet
          type = "app";
          program = "${packages.devnet}/bin/composable-devnet";
        };

        # Shell env a user can enter with `nix develop`
        devShells = {
          default = mkShell {
            buildInputs = with packages; [
              rust-stable
              wasm-optimizer
              composable-node
              mdbook
              taplo
            ];
            NIX_PATH = "nixpkgs=${pkgs.path}";
          };
        };
      });
}
