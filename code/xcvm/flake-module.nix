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
        makeCosmwasmContract name rust "--no-default-features";
    in {
      packages = rec {
        cw-xc-executor = mkCvmContract "cw-xc-interpreter";
        cw-xc-gateway = mkCvmContract "cw-xc-gateway";
        cw-xc-pingpong = mkCvmContract "cw-xc-pingpong";
        cw-mantis-order = mkMantisContract "cw-mantis-order";
        xc-cw-contracts = pkgs.symlinkJoin {
          name = "xc-cw-contracts";
          paths = [ cw-xc-executor cw-xc-gateway cw-mantis-order ];
        };
        xcvm-deps = crane.nightly.buildDepsOnly (systemCommonRust.common-attrs
          // {
            src = systemCommonRust.rustSrc;
          });

        xcvm-mount = pkgs.stdenv.mkDerivation rec {
          name = "xcvm-mount";
          pname = "${name}";
          src = systemCommonRust.rustSrc;
          patchPhase = "true";

          installPhase = ''
            mkdir --parents $out
            mkdir --parents $out/target/wasm32-unknown-unknown/cosmwasm-contracts/
            cp --recursive --no-preserve=mode,ownership $src/. $out/
            cp  "${xc-cw-contracts}/lib/cw_xc_interpreter.wasm" $out/target/wasm32-unknown-unknown/cosmwasm-contracts/
            cp  "${xc-cw-contracts}/lib/cw_xc_gateway.wasm" $out/target/wasm32-unknown-unknown/cosmwasm-contracts/
          '';
          dontFixup = true;
          dontStrip = true;
        };

        xcvm-tests = crane.nightly.cargoBuild (systemCommonRust.common-attrs
          // {
            src = xcvm-mount;
            cargoArtifacts = xcvm-deps;
            buildPhase = ''
              NIX_CARGO_OUT_DIR="$TEMP/out/"
              mkdir --parents "$NIX_CARGO_OUT_DIR"
              cp ${self'.packages.cw20_base} "$NIX_CARGO_OUT_DIR"/cw20_base.wasm
              export NIX_CARGO_OUT_DIR
              # just build, will be moved to testing with host at hand
              cargo build --release --package xc-tests --tests --features="std"
            '';
            installPhase = ''
              mkdir --parents $out
            '';
            RUST_LOG = "debug";
          });
      };
    };
}
