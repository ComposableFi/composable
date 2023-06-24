{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      mkXcvmContract = name:
        let binaryName = "${builtins.replaceStrings [ "-" ] [ "_" ] name}.wasm";
        in crane.nightly.buildPackage (systemCommonRust.common-attrs // {
          src = ./.;
          pnameSuffix = "-${name}";
          cargoBuildCommand =
            "cargo build --target wasm32-unknown-unknown --profile cosmwasm-contracts -p ${name}";
          RUSTFLAGS = "-C link-arg=-s";
          installPhaseCommand = ''
            mkdir -p $out/lib
            mv target/wasm32-unknown-unknown/cosmwasm-contracts/${binaryName} $out/lib/${binaryName}
          '';
        });
    in {
      packages = rec {
        cw20_base = pkgs.fetchurl {
          url =
            "https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm";
          hash = "sha256-nClak9UDPLdALVnN7e9yVKafnKUO7RAYDFO7sxwAXpI=";
        };

        xcvm-contract-interpreter = mkXcvmContract "cw-xc-interpreter";
        xcvm-contract-gateway = mkXcvmContract "cw-xc-gateway";
        xcvm-contract-pingpong = mkXcvmContract "cw-xc-pingpong";
        xcvm-contracts = pkgs.symlinkJoin {
          name = "xc-contracts";
          paths = [ xcvm-contract-interpreter xcvm-contract-gateway ];
        };
        xcvm-deps = crane.nightly.buildDepsOnly (systemCommonRust.common-attrs
          // {
            src = systemCommonRust.mkRustSrc ./.;
          });

        xcvm-mount = pkgs.stdenv.mkDerivation rec {
          name = "xcvm-mount";
          pname = "${name}";
          src = systemCommonRust.mkRustSrc ./.;
          patchPhase = "true";

          installPhase = ''
            mkdir --parents $out
            mkdir --parents $out/target/wasm32-unknown-unknown/cosmwasm-contracts/
            cp --recursive --no-preserve=mode,ownership $src/. $out/
            cp  "${xcvm-contracts}/lib/cw_xc_interpreter.wasm" $out/target/wasm32-unknown-unknown/cosmwasm-contracts/
            cp  "${xcvm-contracts}/lib/cw_xc_gateway.wasm" $out/target/wasm32-unknown-unknown/cosmwasm-contracts/
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
              cp ${cw20_base} "$NIX_CARGO_OUT_DIR"/cw20_base.wasm
              export NIX_CARGO_OUT_DIR
              cargo test --release --package xc-tests
            '';
            installPhase = ''
              mkdir --parents $out
            '';
            RUST_LOG = "debug";
          });
      };
    };
}
