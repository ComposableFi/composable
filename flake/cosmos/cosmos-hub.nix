{ self, ... }: {
  perSystem =
    { self'
    , pkgs
    , systemCommonRust
    , subnix
    , lib
    , system
    , devnetTools
    , cosmosTools
    , bashTools
    , ...
    }:
    let devnet-root-directory = cosmosTools.devnet-root-directory;
    in {

      packages = rec {
        gaiad = pkgs.writeShellApplication {
          name = "gaiad";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            ${self.inputs.cosmos.packages.${system}.gaia}/bin/gaiad "$@"
          '';
        };

        cosmos-hub = pkgs.writeShellApplication {
          name = "cosmos-hub";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ gaiad pkgs.jq ];
          text = ''
            ${bashTools.export pkgs.networksLib.cosmos-hub.devnet}
            ${bashTools.export pkgs.networksLib.devnet.mnemonics}s
            if test "''${1-fresh}" == "fresh"; then
              if pgrep "^gaiad$"; then
                killall "$BINARY"
              fi
              rm -rf "$CHAIN_DATA"                
            fi
            mkdir --parents "$CHAIN_DATA"
          '';
        };
      };
    };
}
