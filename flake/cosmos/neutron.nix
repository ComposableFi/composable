{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, bashTools, ... }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      validator-key = cosmosTools.validators.neutron;

    in {
      _module.args.neutron = rec { inherit env; };

      packages = rec {
        neutrond = pkgs.writeShellApplication {
          name = "neutrond";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            ${self.inputs.cosmos.packages.${system}.neutron}/bin/neutrond "$@"
          '';
        };     
      };
    };
}
