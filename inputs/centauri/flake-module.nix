{ self, ... }: {
  perSystem =
    { config
    , self'
    , inputs'
    , pkgs
    , system
    , crane
    , subnix
    , systemCommonRust
    , ...
    }:
    let
      cargo-lock = builtins.fromTOML (builtins.readFile ../../code/Cargo.lock);
      centauri-runtime-dep = builtins.head
        (builtins.filter (x: x.name == "pallet-ibc") (cargo-lock.package));
      centauri-runtime-commit =
        builtins.elemAt (builtins.split "#" centauri-runtime-dep.source) 2;

      hyperspace-picasso-kusama-config = {
        channel_whitelist = [ ];
        client_id = "10-grandpa-0";
        commitment_prefix = "0x6962632f";
        finality_protocol = "Grandpa";
        connection_id = "connection-0";
        key_type = "sr25519";
        name = "picasso_1";
        para_id = 2087;
        parachain_rpc_url = "ws://devnet-a:9988";
        private_key = "//Alice";
        relay_chain_rpc_url = "ws://devnet-a:9944";
        ss58_version = 49;
        type = "picasso_kusama";
      };

      hyperspace-core-config = { prometheus_endpoint = "https://127.0.0.1"; };

      hyperspace-composable-polkadot-config = {
        type = "composable";
        channel_whitelist = [ ];
        client_id = "10-grandpa-0";
        commitment_prefix = "0x6962632f";
        connection_id = "connection-0";
        finality_protocol = "Grandpa";
        key_type = "sr25519";
        name = "picasso_2";
        para_id = 2087;
        parachain_rpc_url = "ws://devnet-b:29988";
        private_key = "//Alice";
        relay_chain_rpc_url = "ws://devnet-b:29944";
        ss58_version = 50;
      };

      toDockerImage = package:
        self.inputs.bundlers.bundlers."${system}".toDockerImage package;



      #  mkRuntime = name: features: cargoArtifacts:
      #     crane.nightly.buildPackage (systemCommonRust.common-attrs // {
      #       pname = "${name}-runtime";
      #       src = rustSrc;
      #       inherit cargoArtifacts;
      #       cargoBuildCommand =
      #         "cargo build --release --package ${name}-runtime-wasm --target wasm32-unknown-unknown"
      #         + pkgs.lib.strings.optionalString (features != "")
      #         (" --features=${features}");
      #       # From parity/wasm-builder
      #       RUSTFLAGS =
      #         "-C target-cpu=mvp -C target-feature=-sign-ext -C link-arg=--export-table -Clink-arg=--export=__heap_base -C link-arg=--import-memory";
      #     });

      #   # Derive an optimized wasm runtime from a prebuilt one, garbage collection + compression
      #   mkOptimizedRuntime = { name, features ? ""
      #     , common-deps ? self'.packages.common-deps-nightly }:
      #     let runtime = mkRuntime name features common-deps;
      #     in pkgs.stdenv.mkDerivation {
      #       name = "${runtime.name}-optimized";
      #       phases = [ "installPhase" ];
      #       nativeBuildInputs = [ pkgs.binaryen ];
      #       installPhase = ''
      #         mkdir --parents $out/lib
      #         # https://github.com/paritytech/substrate/blob/30cb4d10b3118d1b3aa5b2ae7fa8429b2c4f28de/utils/wasm-builder/src/wasm_project.rs#L694
      #         wasm-opt ${runtime}/lib/${name}_runtime.wasm -o $out/lib/runtime.optimized.wasm -Os --strip-dwarf --debuginfo --mvp-features            
      #         ${self'.packages.subwasm}/bin/subwasm compress $out/lib/runtime.optimized.wasm $out/lib/runtime.optimized.wasm
      #       '';
      #     };

      build-wasm = name: src: crane.nightly.buildPackage (systemCommonRust.common-attrs // {
        pname = name;
        src = src;
        cargoBuildCommand =
          "cargo build --release --package ${name} --target wasm32-unknown-unknown";
        RUSTFLAGS =
          "-C target-cpu=mvp -C target-feature=-sign-ext -C link-arg=--export-table -Clink-arg=--export=__heap_base -C link-arg=--import-memory";
      });

      build-optimized-wasm = name: src: file : 
        let wasm = build-wasm name src;
        in pkgs.stdenv.mkDerivation {
          name = name;
          phases = [ "installPhase" ];
          nativeBuildInputs = [ pkgs.binaryen self'.packages.subwasm];
          installPhase = ''
            mkdir --parents $out/lib
            wasm-opt ${wasm}/lib/${file}.wasm -o $out/lib/${file}.wasm -Os --strip-dwarf --debuginfo --mvp-features            
            subwasm compress $out/lib/${file}.wasm $out/lib/${file}.wasm
          '';
        };

    in
    {
      packages = rec {
        centauri-src = pkgs.fetchFromGitHub {
          owner = "ComposableFi";
          repo = "centauri";
          rev = centauri-runtime-commit;
          #hash = "sha256-GJ0xGg44e+iidkTqeotTqPHMC0ymqDcrD6x/P+XGSUc=";
          hash = "sha256-qIsC8+b2OD7Wv/4jRSGQVirxNXSF0Vn8cOcQNIH5hDo=";
        };

        ics10-grandpa-cw = build-optimized-wasm "ics10-grandpa-cw" centauri-src "ics10_grandpa_cw";

        centauri-codegen = crane.stable.buildPackage (subnix.subenv // {
          name = "centauri-codegen";
          cargoArtifacts = crane.stable.buildDepsOnly (subnix.subenv // {
            src = centauri-src;
            cargoExtraArgs = "--package codegen";
            cargoTestCommand = "";
          });
          src = centauri-src;
          cargoExtraArgs = "--package codegen";
          cargoTestCommand = "";
          meta = { mainProgram = "codegen"; };
        });
        centauri-hyperspace = crane.stable.buildPackage (subnix.subenv // {
          name = "centauri-hyperspace";
          cargoArtifacts = crane.stable.buildDepsOnly (subnix.subenv // {
            src = centauri-src;
            doCheck = false;
            cargoExtraArgs = "--package hyperspace";
            cargoTestCommand = "";
          });
          src = centauri-src;
          doCheck = false;
          cargoExtraArgs = "--package hyperspace";
          cargoTestCommand = "";
          meta = { mainProgram = "hyperspace"; };
        });

        composable-rococo-picasso-rococo-subxt-hyperspace-patch =
          pkgs.stdenv.mkDerivation rec {
            name = "composable-rococo-picasso-rococo-subxt-hyperspace-patch";
            pname = "${name}";
            buildInputs = [
              self'.packages.composable-rococo-subxt-client
              self'.packages.picasso-rococo-subxt-client
            ];
            src = centauri-src;
            patchPhase = "false";
            installPhase = ''
              mkdir --parents $out
            '';
            dontFixup = true;
            dontStrip = true;
          };

        composable-polkadot-picasso-kusama-subxt-hyperspace-patch =
          pkgs.stdenv.mkDerivation rec {
            name = "composable-polkadot-picasso-kusama-subxt-hyperspace-patch";
            pname = "${name}";
            buildInputs = [
              self'.packages.composable-polkadot-subxt-client
              self'.packages.picasso-kusama-subxt-client
            ];
            src = centauri-src;
            patchPhase = "false";
            installPhase = ''
              mkdir --parents $out
            '';
            dontFixup = true;
            dontStrip = true;
          };

        composable-rococo-picasso-rococo-centauri-patched-src =
          pkgs.stdenv.mkDerivation rec {
            name = "composable-rococo-picasso-rococo-centauri-patched-src";
            pname = "${name}";
            src = centauri-src;
            buildInputs = with pkgs; [ sd git ];
            patchFlags = "";
            installPhase = ''
              mkdir --parents $out
              cp --recursive --no-preserve=mode,ownership $src/. $out/
            '';
            dontFixup = true;
            dontStrip = true;
          };

        composable-polkadot-picasso-kusama-centauri-patched-src =
          pkgs.stdenv.mkDerivation rec {
            name = "composable-polkadot-picasso-kusama-centauri-patched-src";
            pname = "${name}";
            src = centauri-src;
            buildInputs = with pkgs; [ sd git ];
            patchFlags = "";
            installPhase = ''
              mkdir --parents $out
              cp --recursive --no-preserve=mode,ownership $src/. $out/
            '';
            dontFixup = true;
            dontStrip = true;
          };

        hyperspace-config-chain-a = pkgs.writeText "config-chain-a.toml"
          (self.inputs.nix-std.lib.serde.toTOML
            hyperspace-picasso-kusama-config);

        hyperspace-config-chain-b = pkgs.writeText "config-chain-b.toml"
          (self.inputs.nix-std.lib.serde.toTOML
            hyperspace-composable-polkadot-config);

        hyperspace-config-core = pkgs.writeText "config-core.toml"
          (self.inputs.nix-std.lib.serde.toTOML hyperspace-core-config);

        hyperspace-composable-rococo-picasso-rococo = crane.stable.buildPackage
          (subnix.subenv // rec {
            name = "hyperspace-composable-rococo-picasso-rococo";
            pname = name;
            cargoArtifacts = crane.stable.buildDepsOnly (subnix.subenv // {
              src = composable-rococo-picasso-rococo-centauri-patched-src;
              doCheck = false;
              cargoExtraArgs = "--package hyperspace";
              cargoTestCommand = "";
            });
            src = composable-rococo-picasso-rococo-centauri-patched-src;
            doCheck = false;
            cargoExtraArgs = "--package hyperspace";
            cargoTestCommand = "";
            meta = { mainProgram = "hyperspace"; };
          });

        hyperspace-composable-polkadot-picasso-kusama =
          crane.stable.buildPackage (subnix.subenv // rec {
            name = "hyperspace-composable-polkadot-picasso-kusama";
            pname = name;
            cargoArtifacts = crane.stable.buildDepsOnly (subnix.subenv // {
              src = composable-polkadot-picasso-kusama-centauri-patched-src;
              doCheck = false;
              cargoExtraArgs = "--package hyperspace";
              cargoTestCommand = "";
            });
            src = composable-polkadot-picasso-kusama-centauri-patched-src;
            doCheck = false;
            cargoExtraArgs = "--package hyperspace";
            cargoTestCommand = "";
            meta = { mainProgram = "hyperspace"; };
          });

        hyperspace-composable-rococo-picasso-rococo-image =
          toDockerImage hyperspace-composable-rococo-picasso-rococo;

        hyperspace-composable-polkadot-picasso-kusama-image =
          toDockerImage hyperspace-composable-polkadot-picasso-kusama;
      };
    };
}
