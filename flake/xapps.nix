# cross chain apps, which require all to be setup and running
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
    , osmosis
    , ...
    }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      validator-key = cosmosTools.validators.osmosis;
    in
    {
      packages = rec {
        xc-transfer-osmo-from--osmosis-to-centauri =
          pkgs.writeShellApplication {
            name = "xc-transfer-osmo-from--osmosis-to-centauri";
            runtimeInputs = devnetTools.withBaseContainerTools
              ++ [ osmosisd pkgs.jq ];
            text = ''
                            HOME=/tmp/composable-devnet
                            export HOME
                            CHAIN_DATA="$HOME/.osmosisd"             
                            KEYRING_TEST=$CHAIN_DATA
                            CHAIN_ID="osmosis-dev"            
                            PORT=36657
                            BLOCK_SECONDS=5
                            FEE=uosmo
                            BINARY=osmosisd
                            GATEWAY_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/gateway_contract_address")
              a
                            TRANSFER_PICA_TO_OSMOSIS=$(cat << EOF
                            {
                                "execute_program": {
                                    "execute_program": {
                                        "salt": "737061776e5f776974685f6173736574",
                                        "program": {
                                            "tag": "737061776e5f776974685f6173736574",
                                            "instructions": [
                                                {
                                                    "spawn": {
                                                        "network": 2,
                                                        "salt": "737061776e5f776974685f6173736574",
                                                        "assets": [
                                                            [
                                                                "158456325028528675187087901673",
                                                                {
                                                                    "amount": {
                                                                        "intercept": "1234567890",
                                                                        "slope": "0"
                                                                    },
                                                                    "is_unit": false
                                                                }
                                                            ]
                                                        ],
                                                        "program": {
                                                            "tag": "737061776e5f776974685f6173736574",
                                                            "instructions": []
                                                        }
                                                    }
                                                }
                                            ]
                                        },
                                        "assets": [
                                            [
                                                "237684487542793012780631852009",
                                                "1234567890"
                                            ]
                                        ]
                                    },
                                    "tip": "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"
                                }
                            }
                            EOF
                            )                  

                            "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$TRANSFER_PICA_TO_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 1000000000"$FEE" --amount 1234567890"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace
                            sleep "$BLOCK_SECONDS"
            '';
          };
        osmosis-tx = pkgs.writeShellApplication {
          name = "osmosis-tx";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ self'.packages.osmosisd pkgs.jq ];

          text = ''
            ${bashTools.export osmosis.env.devnet}
            osmosis tx ibc-transfer transfer transfer channel-0 centauri1qq0k7d56juu7h49arelzgw09jccdk8sujrcrjd 66642100500uosmo --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level trace --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace             
          '';
        };

      };
    };
}
