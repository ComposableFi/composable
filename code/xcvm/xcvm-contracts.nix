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
        xcvm-contract-asset-registry = mkXcvmContract "cw-xcvm-asset-registry";
        xcvm-contract-router = mkXcvmContract "cw-xcvm-router";
        xcvm-contract-interpreter = mkXcvmContract "cw-xcvm-interpreter";
        xcvm-contract-gateway = mkXcvmContract "cw-xcvm-gateway";
        xcvm-contract-pingpong = mkXcvmContract "cw-xcvm-pingpong";
        xcvm-contracts = pkgs.symlinkJoin {
          name = "xcvm-contracts";
          paths = [
            xcvm-contract-asset-registry
            xcvm-contract-router
            xcvm-contract-interpreter
            xcvm-contract-gateway
          ];
        };
        xcvm-deps = crane.nightly.buildDepsOnly (systemCommonRust.common-attrs
          // {
            src = systemCommonRust.mkRustSrc ./.;
          });
        xcvm-tests = crane.nightly.cargoBuild (systemCommonRust.common-attrs
          // {
            src = systemCommonRust.mkRustSrc ./.;
            cargoArtifacts = xcvm-deps;
            buildPhase = "cargo test --release -p xcvm-tests";
            installPhase = "mkdir -p $out";
            CW_XCVM_ASSET_REGISTRY =
              "${xcvm-contracts}/lib/cw_xcvm_asset_registry.wasm";
            CW_XCVM_INTERPRETER =
              "${xcvm-contracts}/lib/cw_xcvm_interpreter.wasm";
            CW_XCVM_ROUTER = "${xcvm-contracts}/lib/cw_xcvm_router.wasm";
            CW_XCVM_GATEWAY = "${xcvm-contracts}/lib/cw_xcvm_gateway.wasm";
            CW20 = pkgs.fetchurl {
              url =
                "https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm";
              hash = "sha256-nClak9UDPLdALVnN7e9yVKafnKUO7RAYDFO7sxwAXpI=";
            };
            RUST_LOG = "debug";
          });
      };
    };
}
