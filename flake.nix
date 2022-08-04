{
  description = "Composable Finance Local Networks Lancher and documentation Book";
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
  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, #nixpkgs-fmt,
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
        nixops = pkgs.callPackage ./.nix/nixops.nix {};
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

        # Wasm optimizer, used to replicate build.rs behavior in an explicit fashion
        wasm-optimizer = crane-stable.buildPackage {
          cargoCheckCommand = "true";
          src = let customFilter = name: type: true;
          in lib.cleanSourceWith {
            filter = lib.cleanSourceFilter;
            src = lib.cleanSourceWith {
              filter =
                nix-gitignore.gitignoreFilterPure customFilter [ ./.gitignore ]
                ./utils/wasm-optimizer;
              src = ./utils/wasm-optimizer;
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
              nix-gitignore.gitignoreFilterPure customFilter [ ./.gitignore ]
              ./.;
            src = ./.;
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
        common-deps-nightly = crane-nightly.buildDepsOnly (common-args // { });
        common-args-with-benchmarks = common-args // { cargoExtraArgs = "--features=runtime-benchmarks --features=builtin-wasm";};
        common-deps-with-benchmarks = crane-stable.buildDepsOnly common-args-with-benchmarks;

        # Build a wasm runtime, unoptimized
        mk-runtime = name:
          let file-name = "${name}_runtime.wasm";
          in crane-nightly.buildPackage (common-args // {
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

        polkadot = import ./.nix/polkadot-version.nix;
        
        devnet-input = builtins.fromJSON (builtins.readFile ./devnet/devnet.json);      

        # https://cloud.google.com/iam/docs/creating-managing-service-account-keys
        # or just use GOOGLE_APPLICATION_CREDENTIALS env as path to file        
        service-account-credential-key-file-input = builtins.fromJSON (builtins.readFile ./devnet/ops.json);
        gce-to-nix = file: 
        {
          project = file.project_id;
          serviceAccount = file.client_email;
          accessKey = file.private_key;
        };
        gce-input = gce-to-nix service-account-credential-key-file-input;

        devnet-deploy = pkgs.callPackage ./.nix/devnet.nix {inherit devnet-input; inherit gce-input; inherit nixpkgs;};
        codespace-base-container = pkgs.callPackage ./.devcontainer/nix/codespace-base-container.nix {inherit system;};

        dali-runtime = mk-optimized-runtime "dali";   
        picasso-runtime = mk-optimized-runtime "picasso";
        composable-runtime = mk-optimized-runtime "composable";

         # NOTE: with docs, non nighly fails but nighly fails too...
          # /nix/store/523zlfzypzcr969p058i6lcgfmg889d5-stdenv-linux/setup: line 1393: --message-format: command not found
        composable-node = with packages;
          crane-stable.buildPackage (common-args // {
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

        composable-node-with-benchmarks = crane-stable.cargoBuild (common-args-with-benchmarks // {
          pnameSuffix = "-node";
          cargoArtifacts = common-deps-with-benchmarks;
          cargoBuildCommand =
            "cargo build --release --package composable";
          DALI_RUNTIME = "${dali-runtime}/lib/runtime.optimized.wasm";
          PICASSO_RUNTIME = "${picasso-runtime}/lib/runtime.optimized.wasm";
          COMPOSABLE_RUNTIME = "${composable-runtime}/lib/runtime.optimized.wasm";
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/composable $out/bin/composable
          '';
        });      

        run-with-benchmarks = chain :  pkgs.writeShellScriptBin "run-benchmarks-once" ''
            ${pkgs.findutils}/bin/find . -name composable
            which ${composable-node-with-benchmarks}/bin/composable
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

      in rec {
        nixopsConfigurations = {
          default = devnet-deploy.machines;
        };
        packages = rec {
          inherit wasm-optimizer;
          inherit common-deps;
          inherit dali-runtime;
          inherit picasso-runtime;
          inherit composable-runtime;
          inherit composable-node;

          price-feed = crane-stable.buildPackage (common-args // {
            pnameSuffix = "-price-feed";
            cargoArtifacts = common-deps;
            cargoBuildCommand = "cargo build --release -p price-feed";
          });
          
          taplo = pkgs.callPackage ./.nix/taplo.nix { inherit crane-stable;};
          mdbook = pkgs.callPackage ./.nix/mdbook.nix { inherit crane-stable;};
          composable-book = import ./book/default.nix { crane = crane-stable; inherit (pkgs) cargo stdenv; inherit mdbook; };
          polkadot-node = pkgs.callPackage ./.nix/polkadot-bin.nix { inherit polkadot;};

          polkadot-launch = pkgs.callPackage ./scripts/polkadot-launch/polkadot-launch.nix {};
          devnet = let
            original-config = import ./scripts/polkadot-launch/composable.nix;
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
          
          # TODO: inherit and provide script to run all stuff
          # devnet-container-xcvm

          codespace-container = dockerTools.buildLayeredImage {
            name = "composable-codespace";
            fromImage = codespace-base-container;
            # be very carefull with this, so this must be version compatible with base and what vscode will inject
            contents = [                                          
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

          codespace-container-xcvm = dockerTools.buildLayeredImage {
            name = "composable-codespace-xcvm";
            fromImage = codespace-container;
            contents = [                                          
                go                
              ];
          };

          dali-script = devnet-deploy.dali.script;
          picasso-script = devnet-deploy.picasso.script;
          dali-composable-book = devnet-deploy.dali.composable-book;
    
          default = packages.composable-node;
        };

        # Derivations built when running `nix flake check`
        checks = {
          tests = crane-stable.cargoBuild (common-args // {
            pnameSuffix = "-tests";
            cargoArtifacts = common-deps;
            cargoBuildCommand = "cargo test --workspace --release";            
          });
          # TODO: on on next runs and replace bash version with this
          dali-dev-benchmarks = run-with-benchmarks "dali-dev";
          picasso-dev-benchmarks = run-with-benchmarks "picasso-dev";
          composable-dev-benchmarks = run-with-benchmarks "composable-dev";
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
            
          sre = developers.overrideAttrs (oldAttrs : rec  {
            buildInputs = oldAttrs.buildInputs ++ 
            [
                packages.picasso-script
                # TODO: replace fetching binries with approciate cachix builds
                # TODO: binaries are referenced by git commit hash (so can retarted to git easy)                
                packages.dali-script 
                packages.dali-composable-book # book deploy couuld be secure deployed too                                                        
            ];
          });       

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
          devnet = {
              # nix run .#devnet
              type = "app";
              program = "${packages.devnet}/bin/composable-devnet";    
          };
          # TODO: move list of chains out of here and do fold
          benchmarks-once-composable = flake-utils.lib.mkApp {
            drv = run-with-benchmarks "composable-dev";
          };
          benchmarks-once-dali = flake-utils.lib.mkApp {
            drv = run-with-benchmarks "dali-dev";
          };
          benchmarks-once-picasso = flake-utils.lib.mkApp {
            drv = run-with-benchmarks "picasso-dev";
          };
          default = devnet;
        };
      });
    in
      eachSystemOutputs
      //
      {
        nixopsConfigurations = {
          # instead of rerusive merge, just do simpler
          x86_64-linux.default = eachSystemOutputs.nixopsConfigurations.x86_64-linux.default; 
          default = eachSystemOutputs.nixopsConfigurations.x86_64-linux.default;
        };
      };
}
