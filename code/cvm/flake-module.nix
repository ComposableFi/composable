{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      makeCosmwasmContract = name: rust: std-config:
        let binaryName = "${builtins.replaceStrings [ "-" ] [ "_" ] name}.wasm";
        in rust.buildPackage (systemCommonRust.common-attrs // {
          src = systemCommonRust.rustSrc;
          version = "0.1";
          pnameSuffix = "-${name}";
          nativeBuildInputs = [
            pkgs.binaryen
            self.inputs.cosmos.packages.${system}.cosmwasm-check
          ];
          pname = name;
          cargoBuildCommand =
            "cargo build --target wasm32-unknown-unknown --profile cosmwasm-contracts --package ${name} ${std-config}";
          RUSTFLAGS = "-C link-arg=-s";
          installPhaseCommand = ''
            mkdir --parents $out/lib
            # from CosmWasm/rust-optimizer
            # --signext-lowering is needed to support blockchains runnning CosmWasm < 1.3. It can be removed eventually
            wasm-opt target/wasm32-unknown-unknown/cosmwasm-contracts/${binaryName} -o $out/lib/${binaryName} -Os --signext-lowering
            cosmwasm-check $out/lib/${binaryName}
          '';
        });
      rust = (self.inputs.crane.mkLib pkgs).overrideToolchain
        (pkgs.rust-bin.stable."1.73.0".default.override {
          targets = [ "wasm32-unknown-unknown" ];
        });
      mkCvmContract = name:
        makeCosmwasmContract name crane.nightly "--no-default-features";
      mkMantisContract = name:
        makeCosmwasmContract name crane.nightly "--no-default-features";
    in {
      packages = rec {
        cw-xc-executor = mkCvmContract "cw-xc-executor";
        cw-xc-gateway = mkCvmContract "cw-xc-gateway";

        xc-cw-contracts = pkgs.symlinkJoin {
          name = "xc-cw-contracts";
          paths = [
            cw-xc-executor
            cw-xc-gateway
            self.inputs.cvm.packages.${system}.cw-mantis-order
          ];
        };
        cvm-deps = crane.nightly.buildDepsOnly (systemCommonRust.common-attrs
          // {
            SKIP_WASM_BUILD = 1;
            src = systemCommonRust.rustSrc;
          });

        build-cvm-json-schema-ts = pkgs.writeShellApplication {
          name = "build-ts-schema";
          runtimeInputs = with pkgs; [
            self'.packages.rust-nightly
            nodejs
            nodePackages.npm
          ];
          text = ''
            echo "generating TypeScript types and client definitions from JSON schema of CosmWasm contracts"
            cd code/cvm
            npm install
            rm --recursive --force dist

            rm --recursive --force schema
            cargo run --bin order --package cw-mantis-order
            npm run build-cw-mantis-order

            rm --recursive --force schema
            cargo run --bin gateway --package xc-core
            npm run build-xc-core

            npm publish
          '';
        };

        cvm-mount = pkgs.stdenv.mkDerivation rec {
          name = "cvm-mount";
          pname = "${name}";
          src = systemCommonRust.rustSrc;
          patchPhase = "true";

          installPhase = ''
            mkdir --parents $out
            mkdir --parents $out/target/wasm32-unknown-unknown/cosmwasm-contracts/
            cp --recursive --no-preserve=mode,ownership $src/. $out/
            cp  "${xc-cw-contracts}/lib/cw_xc_executor.wasm" $out/target/wasm32-unknown-unknown/cosmwasm-contracts/
            cp  "${xc-cw-contracts}/lib/cw_xc_gateway.wasm" $out/target/wasm32-unknown-unknown/cosmwasm-contracts/
          '';
          dontFixup = true;
          dontStrip = true;
        };
      };
    };
}
