{
  description = "Composable Finance systems, tools and releases";
  inputs = {
    bundlers = {
      url = "github:NixOS/bundlers";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };
  outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, npm-buildpackage
    , arion-src, home-manager, helix, bundlers }:
    let

      eachSystemOutputs = flake-utils.lib.eachDefaultSystem (system:
        let
          mk-xcvm-contract = name:
            crane-nightly.buildPackage (common-attrs // {
              pnameSuffix = name;
              cargoBuildCommand =
                "cargo build --target wasm32-unknown-unknown --profile cosmwasm-contracts -p ${name}";
              RUSTFLAGS = "-C link-arg=-s";
            });

        in rec {
          packages = rec {
                       xcvm-contract-asset-registry =
              mk-xcvm-contract "xcvm-asset-registry";
            xcvm-contract-router = mk-xcvm-contract "xcvm-router";
            xcvm-contract-interpreter = mk-xcvm-contract "xcvm-interpreter";
            # TODO: inherit and provide script to run all stuff

          };


          apps = let
            makeApp = p: {
              type = "app";
              program = pkgs.lib.meta.getExe p;
            };
          in rec {
         };
        });
    in eachSystemOutputs // {};
}
