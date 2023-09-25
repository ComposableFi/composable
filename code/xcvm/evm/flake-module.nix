{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      mkXcvmContract = name:
        let binaryName = "${builtins.replaceStrings [ "-" ] [ "_" ] name}.wasm";
        in crane.nightly.buildPackage (systemCommonRust.common-attrs // {
          src = systemCommonRust.rustSrc;
          version = "0.1";
          pnameSuffix = "-${name}";
          nativeBuildInputs = [
            pkgs.binaryen
            self.inputs.cosmos.packages.${system}.cosmwasm-check
          ];
          pname = name;
          cargoBuildCommand =
            "cargo build --target wasm32-unknown-unknown --profile cosmwasm-contracts --package ${name} --no-default-features";
          RUSTFLAGS = "-C link-arg=-s";
          installPhaseCommand = ''
            mkdir --parents $out/lib
            # from CosmWasm/rust-optimizer
            # --signext-lowering is needed to support blockchains runnning CosmWasm < 1.3. It can be removed eventually
            wasm-opt target/wasm32-unknown-unknown/cosmwasm-contracts/${binaryName} -o $out/lib/${binaryName} -Os --signext-lowering
            cosmwasm-check $out/lib/${binaryName}
          '';
        });
    in {
      packages = rec {
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
      };
    };
}
