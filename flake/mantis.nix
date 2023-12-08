{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, ... }: {
      packages = rec {
        mantis-simulate-solve = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ self.inputs.cvm.packages.${system}.mantis ];
          name = "mantis-simulate-solve";
          text = ''
            CHAIN_DATA="${cosmosTools.devnet-root-directory}/.centaurid"
            WALLET=${cosmosTools.cvm.centauri}
            ORDER_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/ORDER_CONTRACT_ADDRESS")
            GATEWAY_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/gateway_contract_address")

            RUST_TRACE=trace mantis --rpc-centauri "http://localhost:26657" --grpc-centauri "http://localhost:9090" --osmosis "http://127.0.0.1:${
              builtins.toString
              pkgs.networksLib.osmosis.devnet.CONSENSUS_RPC_PORT
            }" --neutron "http://127.0.0.1:46657" --cvm-contract "$GATEWAY_CONTRACT_ADDRESS" --wallet "$WALLET" --order-contract "$ORDER_CONTRACT_ADDRESS" --simulate "100ppica,100pdemo"
          '';
        };
      };
    };
}
