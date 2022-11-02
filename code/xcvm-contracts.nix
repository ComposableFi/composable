{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      mkXcvmContract = name:
        crane.nightly.buildPackage (systemCommonRust.common-attrs // {
          pnameSuffix = name;
          cargoBuildCommand =
            "cargo build --target wasm32-unknown-unknown --profile cosmwasm-contracts -p ${name}";
          RUSTFLAGS = "-C link-arg=-s";
        });
    in
    {
      packages = {
        xcvm-contract-asset-registry = mkXcvmContract "xcvm-asset-registry";
        xcvm-contract-router = mkXcvmContract "xcvm-router";
        xcvm-contract-interpreter = mkXcvmContract "xcvm-interpreter";
        # TODO: inherit and provide script to run all stuff
      };
    };
}
