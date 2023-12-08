{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, bashTools, ... }: {
      packages = rec {
        mantis-simulate-solve = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ self.inputs.cvm.packages.${system}.mantis ];
          name = "mantis-simulate-solve";
          text = ''
            ${bashTools.export pkgs.networksLib.devnet.mnemonics}

            ORDER_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/ORDER_CONTRACT_ADDRESS")
            OUTPOST_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/outpost_contract_address")

            RUST_BACKTRACE=1 RUST_TRACE=trace mantis solve --rpc-centauri "http://localhost:26657" --grpc-centauri "http://localhost:9090" --cvm-contract "$OUTPOST_CONTRACT_ADDRESS" --wallet "$APPLICATION1" --order-contract "$ORDER_CONTRACT_ADDRESS" --simulate "200000ppica,2000pdemo"
          '';
        };
      };
    };
}
