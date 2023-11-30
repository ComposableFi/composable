{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, bashTools, networkTools, ... }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      validator-key = cosmosTools.validators.neutron;

    in {
      packages = rec {
        neutrond = pkgs.writeShellApplication {
          name = "neutrond";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            ${self.inputs.cosmos.packages.${system}.neutron}/bin/neutrond "$@"
          '';
        };   

        neutrond-gen = pkgs.writeShellApplication {
          name = "neutrond-gen";
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ neutrond ];
          text = ''
              ${bashTools.export networksLib.neutron.devnet}
              if test "''${1-reuse} == "fresh"; then
                killall 
                killall "s$BINARY"
                rm -rf "$CHAIN_DATA"                
              fi

              mkdir --parents "$CHAIN_DATADIR"
          '';
        };   
      };
    };
}
