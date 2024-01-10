# cross chain apps, which require all to be setup and running
{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, bashTools, osmosis, centauri, ... }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      cosmos-log = "--log_level trace --trace";
    in {
      packages = rec {
        xc-transfer-osmo-from--osmosis-to-centauri =
          pkgs.writeShellApplication {
            name = "xc-transfer-osmo-from--osmosis-to-centauri";
            runtimeInputs = devnetTools.withBaseContainerTools
              ++ [ self'.packages.osmosisd pkgs.jq ];
            text = ''
                            HOME=/tmp/composable-devnet
                            export HOME
                            CHAIN_DATA="$HOME/.osmosisd"             
                            KEYRING_TEST=$CHAIN_DATA
                            CHAIN_ID="osmosis-dev"            
                            PORT=${pkgs.networksLib.osmosis.devnet.PORT}
                            BLOCK_SECONDS=5
                            FEE=uosmo
                            BINARY=osmosisd
                            OUTPOST_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/outpost_contract_address")
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

                            "$BINARY" tx wasm execute "$OUTPOST_CONTRACT_ADDRESS" "$TRANSFER_PICA_TO_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 1000000000"$FEE" --amount 1234567890"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.cvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace
                            sleep "$BLOCK_SECONDS"
            '';
          };
        xapp-osmosis-osmo-to-centauri = pkgs.writeShellApplication {
          name = "osmosis-osmo-to-centauri";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ self'.packages.osmosisd pkgs.jq ];

          text = ''
            ${bashTools.export pkgs.networksLib.osmosis.devnet}
            osmosisd tx ibc-transfer transfer transfer channel-0 ${cosmosTools.cvm.centauri} 66642100500uosmo --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level trace --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.cvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace             
          '';
        };

        transfer-centauri-pica-to-osmosis = pkgs.writeShellApplication {
          name = "transfer-centauri-pica-to-osmosis";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ self'.packages.centaurid pkgs.jq ];

          text = ''
            ${bashTools.export pkgs.networksLib.pica.devnet}

            FROM_ADDRESS=$("$BINARY" keys show APPLICATION1 --keyring-backend test --home "$CHAIN_DATA" --keyring-dir "$KEYRING_TEST" --output json | jq -r '.address')
            echo "$FROM_ADDRESS"
            centaurid tx ibc-transfer transfer transfer channel-0 "$FROM_ADDRESS" 1366642100500ppica --chain-id="$CHAIN_ID"  --node="$NODE" --output=json --yes --gas=25000000 --fees=920000166"$FEE" ${cosmos-log} --keyring-backend=test  --home="$CHAIN_DATA" --from="$FROM_ADDRESS" --keyring-dir="$KEYRING_TEST"             
          '';
        };

        transfer-centauri-pica-to-neutron = pkgs.writeShellApplication {
          name = "transfer-centauri-pica-to-neutron";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ self'.packages.centaurid pkgs.jq ];

          text = ''
            ${bashTools.export pkgs.networksLib.pica.devnet}
            FROM_ADDRESS=$("$BINARY" keys show APPLICATION1 --keyring-backend test --home "$CHAIN_DATA" --keyring-dir "$KEYRING_TEST" --output json | jq -r '.address')
            centaurid tx ibc-transfer transfer transfer channel-1 "$FROM_ADDRESS" 1366642100500ppica --chain-id="$CHAIN_ID"  --node "tcp://localhost:$RPCPORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" ${cosmos-log} --keyring-backend test  --home "$CHAIN_DATA" --from "$FROM_ADDRESS" --keyring-dir "$KEYRING_TEST"             
          '';
        };

        devnet-transfer-to-virtual-wallet = pkgs.writeShellApplication {
          name = "devnet-transfer-to-virtual-wallet";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ self'.packages.centaurid ];
          text = ''
            sleep 12 # just stupid wait for previous transfer of osmo, need to improve
            ${bashTools.export pkgs.networksLib.pica.devnet}
            OUTPOST_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/outpost_contract_address")

            APP_MSG=$(cat << EOF            
            {
              "execute_program": {
                "program": {
                  "instructions": [
                    {
                      "transfer": {
                        "to": {
                          "account": "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k"
                        },
                        "assets": [
                          [
                            "158456325028528675187087900673",
                            {
                              "slope": "1000000000000000000"
                            }
                          ]
                        ]
                      }
                    }                                         
                  ]
                }
              }
            }            
            EOF
            )
            centaurid tx wasm execute "$OUTPOST_CONTRACT_ADDRESS" "$APP_MSG" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --yes --gas 25000000 --fees 1000000000"$FEE" --amount 3232323"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from test1 --keyring-dir "$KEYRING_TEST" --trace --log_level trace
          '';
        };

        devnet-swap-centauri-osmo-to-osmosis-pica-and-back =
          pkgs.writeShellApplication {
            name = "devnet-swap-centauri-osmo-to-osmosis-pica-and-back";
            runtimeInputs = devnetTools.withBaseContainerTools
              ++ [ self'.packages.centaurid ];
            text = ''
              sleep 12 # just stupid wait for previous transfer of osmo, need to improve
              ${bashTools.export pkgs.networksLib.pica.devnet}
              OUTPOST_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/outpost_contract_address")

              APP_MSG=$(cat << EOF            
              {
                "execute_program": {
                  "program": {
                    "instructions": [
                      {
                        "spawn": {
                          "network_id": 3,
                          "assets": [
                            [
                              "158456325028528675187087900674",
                              {
                                "slope": "1000000000000000000"
                              }
                            ]
                          ],
                          "program": {
                            "instructions": [
                              {
                                "exchange": {
                                  "exchange_id": "237684489387467420151587012609",
                                  "give": [
                                    [
                                      "237684487542793012780631851010",
                                      {
                                        "slope": "1000000000000000000"
                                      }
                                    ]
                                  ],
                                  "want": [
                                    [
                                      "237684487542793012780631851009",
                                      {
                                        "slope": "950000000000000000"
                                      }
                                    ]
                                  ]
                                }
                              },
                              {
                                "spawn": {
                                  "network_id": 2,
                                  "assets": [
                                    [
                                      "237684487542793012780631851009",
                                      {
                                        "slope": "1000000000000000000"
                                      }
                                    ]
                                  ],
                                  "program": {
                                    "instructions": [
                                      {
                                        "transfer": {
                                          "to": {
                                            "account": "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k"
                                          },
                                          "assets": [
                                            [
                                              "158456325028528675187087900673",
                                              {
                                                "slope": "1000000000000000000"
                                              }
                                            ]
                                          ]
                                        }
                                      }
                                    ]
                                  }
                                }
                              }
                            ]
                          }
                        }
                      }
                    ]
                  }
                }
              }            
              EOF
              )
              "$BINARY" tx wasm execute "$OUTPOST_CONTRACT_ADDRESS" "$APP_MSG" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --yes --gas 25000000 --fees 1000000000"$FEE" --amount "1212121ibc/ED07A3391A112B175915CD8FAF43A2DA8E4790EDE12566649D0C2F97716B8518" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.cvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace
            '';
          };

        devnet-swap-pica-to-osmo = pkgs.writeShellApplication {
          name = "devnet-swap-pica-to-osmo";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ self'.packages.centaurid pkgs.jq ];
          text = ''
            ${bashTools.export pkgs.networksLib.pica.devnet}
            OUTPOST_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/outpost_contract_address")

            SWAP_PICA_TO_OSMOSIS=$(cat << EOF
              {
                  "execute_program": {
                    "program": {
                      "instructions": [
                        {
                          "spawn": {
                            "network_id": 3,
                            "assets": [
                              [
                                "158456325028528675187087900673",
                                {
                                    "intercept": "1234567890",
                                    "slope": "0"
                                }
                              ]
                            ],
                            "program": {
                              "instructions": [
                                {
                                  "exchange": {
                                    "exchange_id": "237684489387467420151587012609",
                                    "give": [
                                      [
                                        "237684487542793012780631851009",
                                        {
                                            "intercept": "123456789",
                                            "slope": "0"
                                        }
                                      ]
                                    ],
                                    "want": [
                                      [
                                        "237684487542793012780631851010",
                                        {
                                            "intercept": "1000",
                                            "slope": "0"
                                        }
                                      ]
                                    ]
                                  }
                                },
                                {
                                  "spawn": {
                                    "network_id": 2,
                                    "assets": [
                                      [
                                        "237684487542793012780631851010",
                                        {
                                            "intercept": "0",
                                            "slope": "1000000000000000000"
                                        }
                                      ]
                                    ],
                                    "program": {
                                      "instructions": [
                                        {
                                          "transfer": {
                                            "to": {
                                              "account": "AB9vNpqXOevUvR5+JDnlljDbHhw="
                                            },
                                            "assets": [
                                              [
                                                "158456325028528675187087900674",
                                                {
                                                    "slope": "1000000000000000000"
                                                }
                                              ]
                                            ]
                                          }
                                        }
                                      ]
                                    }
                                  }
                                }
                              ]
                            }
                          }
                        }
                      ]
                    },
                    "assets": [
                      [
                        "158456325028528675187087900673",
                        "1234567890"
                      ]
                    ]
                  }       
              }
            EOF
            )                  

            centaurid tx wasm execute "$OUTPOST_CONTRACT_ADDRESS" "$SWAP_PICA_TO_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --yes --gas 25000000 --fees 1000000000"$FEE" --amount 1234567890"$FEE" --log_level info --keyring-backend test --home "$CHAIN_DATA" --from test1 --keyring-dir "$KEYRING_TEST" --trace --log_level trace
          '';
        };

      };
    };
}
