{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, ... }:
    let log = "debug";
    in {
      packages = rec {
        mantis-simulate-solve = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ self.inputs.cvm.packages.${system}.mantis ];
          name = "mantis-simulate-solve";
          text = ''
            CHAIN_DATA="${cosmosTools.devnet-root-directory}/.centaurid"
            KEY=${cosmosTools.cvm.centauri}
            ORDER_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/ORDER_CONTRACT_ADDRESS")
            GATEWAY_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/gateway_contract_address")
            RUST_TRACE=${log} mantis --centauri "http://localhost:26657" --osmosis "localhost:36657" --neutron "localhost:46657" --cvm-contract "$GATEWAY_CONTRACT_ADDRESS" --wallet "$KEY" --order-contract "$ORDER_CONTRACT_ADDRESS"
          '';
        };
      };
    };
}
