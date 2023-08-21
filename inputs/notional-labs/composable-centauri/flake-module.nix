{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, subnix, devnetTools, cosmosTools, ... }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      validator-mnemonic = cosmosTools.validators.mnemonic;
      validator-key = cosmosTools.validators.centauri;
      gov = {
        account = "centauri10d07y265gmmuvt4z0w9aw880jnsr700j7g7ejq";
        voting_period = "20s";
        max_deposit_period = "10s";
      };
      native_denom = "ppica";
      name = "centaurid";
      centaurid = pkgs.writeShellApplication {
        name = "centaurid";
        text = ''
          ${self.inputs.cosmos.packages.${system}.centauri}/bin/centaurid "$@"
        '';
      };

      ibc-lightclients-wasm-v1-msg-push-new-wasm-code = code: {
        "messages" = [{
          "@type" = "/ibc.lightclients.wasm.v1.MsgPushNewWasmCode";
          "signer" = "${gov.account}";
          "code" = code;
        }];
        "deposit" = "5000000000000000ppica";
        "metadata" = "none";
        "title" = "none";
        "summary" = "none";
      };

      ics10-grandpa-cw-proposal = let
        code = builtins.readFile
          "${self'.packages.ics10-grandpa-cw}/lib/ics10_grandpa_cw.wasm.gz.txt";
        code-file = builtins.toFile "ics10_grandpa_cw.wasm.json"
          (builtins.toJSON
            (ibc-lightclients-wasm-v1-msg-push-new-wasm-code code));
      in pkgs.stdenv.mkDerivation {
        name = "ics10-grandpa-cw-proposal";
        dontUnpack = true;
        installPhase = ''
          mkdir --parents $out
          cp ${code-file} $out/ics10_grandpa_cw.wasm.json
        '';
      };
      centaurid-init = pkgs.writeShellApplication {
        name = "centaurid-init";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq self'.packages.xc-cw-contracts ];

        text = ''
          CHAIN_DATA="${devnet-root-directory}/.centaurid"

          CHAIN_ID="centauri-dev"
          KEYRING_TEST="$CHAIN_DATA/keyring-test"
          VALIDATOR_KEY=${validator-key}
          PORT=26657
          BLOCK_SECONDS=5
          FEE=ppica
          BINARY=centaurid

          "$BINARY" tx gov submit-proposal ${ics10-grandpa-cw-proposal}/ics10_grandpa_cw.wasm.json --from "$VALIDATOR_KEY"  --keyring-backend test --gas 9021526220000 --fees 92000000166$FEE --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CHAIN_DATA" --output json
          sleep $BLOCK_SECONDS
          "$BINARY" query auth module-account gov --chain-id "$CHAIN_ID" --node tcp://localhost:$PORT --home "$CHAIN_DATA" | jq '.account.base_account.address' --raw-output
          PROPOSAL_ID=1          
          "$BINARY" tx gov vote $PROPOSAL_ID yes --from "$VALIDATOR_KEY"  --keyring-backend test --gas 9021526220000 --fees 92000000166$FEE --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CHAIN_DATA" --output json          
          sleep 20          
          "$BINARY" query gov proposal $PROPOSAL_ID --chain-id "$CHAIN_ID" --node tcp://localhost:$PORT --home "$CHAIN_DATA" | jq '.status'
          sleep $BLOCK_SECONDS         
          "$BINARY" query 08-wasm all-wasm-code --chain-id "$CHAIN_ID" --home "$CHAIN_DATA" --output json --node tcp://localhost:$PORT | jq '.code_ids[0]' --raw-output | tee "$CHAIN_DATA/code_id"
        '';
      };

      centaurid-xcvm-init = pkgs.writeShellApplication {
        name = "centaurid-xcvm-init";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq self'.packages.xc-cw-contracts ];

        text = ''
          CHAIN_DATA="${devnet-root-directory}/.centaurid"

          CHAIN_ID="centauri-dev"
          KEYRING_TEST="$CHAIN_DATA/keyring-test"
          KEY=${cosmosTools.xcvm.centauri}
          PORT=26657
          BLOCK_SECONDS=5
          FEE=ppica 
          NETWORK_ID=2
          BINARY=centaurid

          function init_xcvm() {
              local INSTANTIATE=$1
              "$BINARY" tx wasm store  "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166$FEE --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"
              GATEWAY_CODE_ID=1

              sleep $BLOCK_SECONDS
              "$BINARY" tx wasm store  "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166$FEE --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"

              sleep $BLOCK_SECONDS
              "$BINARY" tx wasm store  "${self'.packages.cw20_base}" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166$FEE --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"

              sleep $BLOCK_SECONDS
             
              "$BINARY" tx wasm instantiate2 $GATEWAY_CODE_ID "$INSTANTIATE" "1234" --label "xc-gateway" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166$FEE --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --admin "$KEY" --amount 1000000000000$FEE

              sleep $BLOCK_SECONDS
              GATEWAY_CONTRACT_ADDRESS=$("$BINARY" query wasm list-contract-by-code "$GATEWAY_CODE_ID" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --home "$CHAIN_DATA" | dasel --read json '.contracts.[0]' --write yaml)      
              echo "$GATEWAY_CONTRACT_ADDRESS" > "$CHAIN_DATA/gateway_contract_address"        
              echo "2" > "$CHAIN_DATA/interpreter_code_id"  
          }

          INSTANTIATE=$(cat << EOF
              {
                  "admin" : "$KEY", 
                  "network_id" : $NETWORK_ID
              }                                 
          EOF
          )

          init_xcvm "$INSTANTIATE"        
        '';
      };

      centaurid-xcvm-config = pkgs.writeShellApplication {
        name = "centaurid-xcvm-config";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq self'.packages.xc-cw-contracts ];

        text = ''

          HOME=${devnet-root-directory}
          export HOME
          KEY=${cosmosTools.xcvm.centauri}

          CHAIN_DATA="$HOME/.centaurid"
          CHAIN_ID="centauri-dev"
          KEYRING_TEST="$CHAIN_DATA/keyring-test"
          PORT=26657
          BLOCK_SECONDS=5
          FEE=ppica 
          BINARY=centaurid

          GATEWAY_CONTRACT_ADDRESS=$(cat $CHAIN_DATA/gateway_contract_address)        
          INTERPRETER_CODE_ID=$(cat $CHAIN_DATA/interpreter_code_id)
          OSMOSIS_GATEWAY_CONTRACT_ADDRESS=$(cat "$HOME/.osmosisd/gateway_contract_address")  

          FORCE_NETWORK_OSMOSIS=$(cat << EOF
            {
              "config": {
                  "force_network": {
                    "network_id": 3,
                    "accounts": {
                        "bech": "osmo"
                    },
                    "gateway": {
                        "cosm_wasm": {
                          "contract": "$OSMOSIS_GATEWAY_CONTRACT_ADDRESS",
                          "interpreter_code_id": $INTERPRETER_CODE_ID,
                          "admin": "$KEY"
                        }
                    },
                    "ibc": {
                        "channels": {
                          "ics20": {
                              "sender": "CosmosStargateIbcApplicationsTransferV1MsgTransfer",
                              "features": {
                                "pfm": {},
                                "wasm_hooks": {
                                    "callback": true
                                }
                              }
                          }
                        }
                    }
                  }
              }
            }                                   
          EOF
          )
          "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_NETWORK_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace             

          sleep $BLOCK_SECONDS
          FORCE_NETWORK_CENTAURI=$(cat << EOF
            {
              "config": {
                  "force_network": {
                    "network_id": 2,
                    "accounts": {
                        "bech": "centauri"
                    },
                    "gateway": {
                        "cosm_wasm": {
                          "contract": "$GATEWAY_CONTRACT_ADDRESS",
                          "interpreter_code_id": $INTERPRETER_CODE_ID,
                          "admin": "$KEY"
                        }
                    },
                    "ibc": {
                        "channels": {
                          "ics20": {
                              "sender": "CosmosStargateIbcApplicationsTransferV1MsgTransfer",
                              "features": {
                                "pfm": {},
                                "wasm_hooks": {
                                    "callback": true
                                }
                              }
                          }
                        }
                    }
                  }
              }
            }                                   
          EOF
          )
          "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_NETWORK_CENTAURI" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace    


          sleep $BLOCK_SECONDS
          FORCE_CENTAURI_TO_OSMOSIS=$(cat << EOF
            {
              "config": {
                  "force_network_to_network": {
                    "from": 2,
                    "to": 3,
                    "other": {
                        "counterparty_timeout": {
                          "seconds" : 120
                        },
                        "ics_20": {
                          "source" : "channel-0", 
                          "sink" : "channel-0" 
                        }                                                
                    }
                  }
              }
            }                                 
          EOF
          )
          "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_CENTAURI_TO_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace

          sleep $BLOCK_SECONDS
          FORCE_PICA=$(cat << EOF
          {
            "config": {
              "force_asset": {
                "asset_id": "158456325028528675187087900673",
                "network_id": 2,
                "local": {
                  "native": {
                    "denom": "ppica"
                  }
                },
                "bridged": {
                  "location_on_network": {
                    "ibc_ics20": {
                      "base_denom" : "ppica",
                      "trace_path" : "transfer/channel-0"
                    }
                  }
                }
              }
            }
          }                               
          EOF
          )
          "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_PICA" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace


          sleep $BLOCK_SECONDS
          FORCE_PICA_ON_CENTAURI=$(cat << EOF
          {
            "config": {
              "force_asset": {
                "asset_id": "158456325028528675187087900674",
                "network_id": 2,
                "local": {
                  "native": {
                    "denom": "uatom"
                  }
                },
                "bridged": {
                  "location_on_network": {
                    "ibc_ics20": {
                      "base_denom" : "uatom",
                      "trace_path" : "transfer/channel-0"
                    }
                  }
                }
              }
            }
          }                               
          EOF
          )
          "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_PICA_ON_CENTAURI" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace


          sleep $BLOCK_SECONDS
          FORCE_PICA_ON_OSMOSIS=$(cat << EOF
          {
            "config": {
              "force_asset": {
                "asset_id": "237684487542793012780631851009",
                "network_id": 3,
                "local": {
                  "native": {
                    "denom": "ppica"
                  }
                },
                "bridged": {
                  "location_on_network": {
                    "ibc_ics20": {
                      "base_denom" : "ppica",
                      "trace_path" : "transfer/channel-0"
                    }
                  }
                }
              }
            }
          }                               
          EOF
          )
          "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_PICA_ON_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace


          sleep $BLOCK_SECONDS
          FORCE_PICA_FROM_CENTAURI_TO_OSMOSIS=$(cat << EOF
          {
            "config": {
              "force_asset_to_network_map": {
                "this_asset": "158456325028528675187087900673",
                "other_network": 3,
                "other_asset": "237684487542793012780631851009"          
              }
            }
          }                               
          EOF
          )
          "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_PICA_FROM_CENTAURI_TO_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace

          sleep $BLOCK_SECONDS
          FORCE_UATOM=$(cat << EOF
            {
              "config": {
                  "force_asset": {
                    "asset_id": "237684487542793012780631851010",
                    "network_id": 2,
                    "local": {
                      "native": {
                        "denom" : "uatom"
                      }
                    }
                  }
              }
            }                                 
          EOF
          )
          "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_UATOM" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace
          sleep $BLOCK_SECONDS
          "$BINARY" query wasm contract-state all "$GATEWAY_CONTRACT_ADDRESS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --home "$CHAIN_DATA"
        '';
      };

      xc-swap-pica-to-osmos = pkgs.writeShellApplication {
        name = "centaurid-execute-program";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq ];
        text = ''
          CHAIN_DATA="${devnet-root-directory}/.centaurid"          
          CHAIN_ID="centauri-dev"
          KEYRING_TEST="$CHAIN_DATA/keyring-test"
          PORT=26657
          FEE=ppica
          BLOCK_SECONDS=5
          BINARY=centaurid
          GATEWAY_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/gateway_contract_address")

          SWAP_PICA_TO_OSMOSIS=$(cat << EOF
            {
              "execute_program": {
                "execute_program": {
                  "salt": "737061776e5f776974685f6173736574",
                  "program": {
                    "tag": "737061776e5f776974685f6173736574",
                    "instructions": [
                      {
                        "spawn": {
                          "network": 3,
                          "salt": "737061776e5f776974685f6173736574",
                          "assets": [
                            [
                              "237684487542793012780631851009",
                              {
                                "amount": {
                                  "intercept": "1000000000",
                                  "slope": "0"
                                },
                                "is_unit": false
                              }
                            ]
                          ],
                          "program": {
                            "tag": "737061776e5f776974685f6173736574",
                            "instructions": [
                              {
                                "exchange": {
                                  "id": "237684489387467420151587012609",
                                  "give": [
                                    [
                                      "237684487542793012780631851009",
                                      {
                                        "amount": {
                                          "intercept": "1000000000",
                                          "slope": "0"
                                        },
                                        "is_unit": false
                                      }
                                    ]
                                  ],
                                  "want": [
                                    [
                                      "237684487542793012780631851010",
                                      {
                                        "amount": {
                                          "intercept": "1000",
                                          "slope": "0"
                                        },
                                        "is_unit": false
                                      }
                                    ]
                                  ]
                                }
                              },
                              {
                                "spawn": {
                                  "network": 2,
                                  "salt": "737061776e5f776974685f6173736574",
                                  "assets": [
                                    [
                                      "158456325028528675187087900674",
                                      {
                                        "amount": {
                                          "intercept": "0",
                                          "slope": "1000000000000000000"
                                        },
                                        "is_unit": false
                                      }
                                    ]
                                  ],
                                  "program": {
                                    "tag": "737061776e5f776974685f6173736574",
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
                                                "amount": {
                                                  "intercept": "0",
                                                  "slope": "1000000000000000000"
                                                },
                                                "is_unit": false
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
                      "1000000000"
                    ]
                  ]
                },
                "tip": "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"
              }
            }
          EOF
          )                  

          # check route
          "$BINARY" query wasm contract-state smart "$GATEWAY_CONTRACT_ADDRESS" '{ "get_ibc_ics20_route" : { "for_asset" : "158456325028528675187087900673", "to_network": 3 } }' --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --home "$CHAIN_DATA"

          "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$SWAP_PICA_TO_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 1000000000"$FEE" --amount 1234567890"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace
          sleep "$BLOCK_SECONDS"
        '';
      };

      centaurid-gen-fresh = pkgs.writeShellApplication {
        name = "centaurid-gen-fresh";
        runtimeInputs = [ centaurid-gen ];
        text = ''
          centaurid-gen fresh
        '';
      };

      centaurid-gen = pkgs.writeShellApplication {
        name = "centaurid-gen";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq ];
        text = ''
          CHAIN_DATA="${devnet-root-directory}/.centaurid"
          CHAIN_ID="centauri-dev"
          KEYRING_TEST="$CHAIN_DATA/keyring-test"

          if test "''${1-reuse}" == "fresh"; then
             echo "removing data dir"
             rm --force --recursive "$CHAIN_DATA"
          fi
          PICA_CHANNEL_ID=''${2-1}

          if [[ ! -d "$CHAIN_DATA" ]]; then            
            mkdir --parents "$CHAIN_DATA"
            mkdir --parents "$CHAIN_DATA/config/gentx"
            mkdir --parents "$KEYRING_TEST"
            echo "${validator-mnemonic}" | centaurid init "$CHAIN_ID" --chain-id "$CHAIN_ID" --default-denom ${native_denom} --home "$CHAIN_DATA"  --recover           

            function jq-genesis() {
              jq -r  "$1"  > "$CHAIN_DATA/config/genesis-update.json"  < "$CHAIN_DATA/config/genesis.json"
              mv --force "$CHAIN_DATA/config/genesis-update.json" "$CHAIN_DATA/config/genesis.json"
            }

            jq-genesis '.consensus_params.block.max_gas |= "-1"'  
            jq-genesis '.app_state.gov.params.voting_period |= "${gov.voting_period}"'  
            jq-genesis '.app_state.gov.params.max_deposit_period |= "${gov.max_deposit_period}"'  

           function pica_setup() {
              jq-genesis '.app_state.transmiddleware.token_infos[0].ibc_denom |= "ibc/632DBFDB06584976F1351A66E873BF0F7A19FAA083425FEC9890C90993E5F0A4"'            
              jq-genesis ".app_state.transmiddleware.token_infos[0].channel_id |= \"channel-$PICA_CHANNEL_ID\""  
              jq-genesis '.app_state.transmiddleware.token_infos[0].native_denom |= "ppica"'
              jq-genesis '.app_state.transmiddleware.token_infos[0].asset_id |= "1"'
           }
           pica_setup

            sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$CHAIN_DATA/config/client.toml"
            sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$CHAIN_DATA/config/client.toml"            
            sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$CHAIN_DATA/config/client.toml"
            sed -i 's/output = "text"/output = "json"/' "$CHAIN_DATA/config/client.toml"
            sed -i "s/cors_allowed_origins = \[\]/cors_allowed_origins = \[\"\*\"\]/" "$CHAIN_DATA/config/config.toml"
            sed -i "s/swagger = false/swagger = true/" "$CHAIN_DATA/config/app.toml"           
            sed -i "s/rpc-max-body-bytes = 1000000/rpc-max-body-bytes = 10000000/" "$CHAIN_DATA/config/app.toml"
            sed -i "s/max_body_bytes = 1000000/max_body_bytes = 10000000/" "$CHAIN_DATA/config/config.toml"
            sed -i "s/max_header_bytes = 1048576/max_header_bytes = 10485760/" "$CHAIN_DATA/config/config.toml"
            sed -i "s/max_tx_bytes = 1048576/max_tx_bytes = 10485760/" "$CHAIN_DATA/config/config.toml"

            echo "document prefer nurse marriage flavor cheese west when knee drink sorry minimum thunder tilt cherry behave cute stove elder couch badge gown coral expire" | centaurid keys add alice --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true    
            echo "bleak slush nose opinion document sample embark couple cabbage soccer cage slow father witness canyon ring distance hub denial topic great beyond actress problem" | centaurid keys add bob --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "coffee hospital claim ability wrap load display submit lecture solid secret law base barrel miss tattoo desert want wall bar ketchup sauce real unknown" | centaurid keys add charlie --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "${validator-mnemonic}" | centaurid keys add ${cosmosTools.validators.moniker} --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "${cosmosTools.xcvm.mnemonic}" | centaurid keys add ${cosmosTools.xcvm.moniker} --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius" | centaurid keys add test1 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty" | centaurid keys add test2 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb" | centaurid keys add test3 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "black frequent sponsor nice claim rally hunt suit parent size stumble expire forest avocado mistake agree trend witness lounge shiver image smoke stool chicken" | centaurid keys add relayer --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            function add-genesis-account () {
              centaurid --keyring-backend test add-genesis-account "$1" "10000000000000000000000ppica" --keyring-backend test --home "$CHAIN_DATA"          
            }

            add-genesis-account centauri1qvdeu4x34rapp3wc8fym5g4wu343mswxxgc6wf
            add-genesis-account centauri1zr4ng42laatyh9zx238n20r74spcrlct6jsqaw
            add-genesis-account centauri1makf5hslxqxzl29uyeyyddf89ff7edxyr7ewm5
            add-genesis-account ${validator-key}
            add-genesis-account ${cosmosTools.xcvm.centauri}
            add-genesis-account centauri1cyyzpxplxdzkeea7kwsydadg87357qnamvg3y3
            add-genesis-account centauri18s5lynnmx37hq4wlrw9gdn68sg2uxp5ry85k7d
            add-genesis-account centauri1qwexv7c6sm95lwhzn9027vyu2ccneaqapystyu
            centaurid --keyring-backend test --keyring-dir "$KEYRING_TEST" --home "$CHAIN_DATA" gentx ${cosmosTools.validators.moniker} "250000000000000ppica" --chain-id="$CHAIN_ID" --amount="250000000000000ppica"
            centaurid collect-gentxs --home "$CHAIN_DATA"  --gentx-dir "$CHAIN_DATA/config/gentx"
          else
            echo "WARNING: REUSING EXISTING DATA FOLDER"
          fi
          centaurid start --rpc.unsafe --rpc.laddr tcp://0.0.0.0:26657 --pruning=nothing --minimum-gas-prices=0ppica --log_level debug --home "$CHAIN_DATA" --db_dir "$CHAIN_DATA/data" --trace --with-tendermint true --transport socket --trace-store $CHAIN_DATA/kvstore.log --grpc.address localhost:9090 --grpc.enable true --grpc-web.enable false --api.enable true --cpu-profile $CHAIN_DATA/cpu-profile.log --p2p.pex false --p2p.upnp  false
        '';
      };
    in {
      packages = rec {
        inherit centaurid centaurid-gen centaurid-init centaurid-gen-fresh
          ics10-grandpa-cw-proposal xc-swap-pica-to-osmos
          centaurid-xcvm-init centaurid-xcvm-config;

        centauri-exec = pkgs.writeShellApplication {
          name = "centaurid-xcvm-config";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ centaurid pkgs.jq self'.packages.xc-cw-contracts ];

          text = ''
            CHAIN_DATA="${devnet-root-directory}/.centaurid"
            CHAIN_ID="centauri-dev"
            KEYRING_TEST="$CHAIN_DATA/keyring-test"
            PORT=26657
            FEE=ppica 
            BINARY=centaurid
            GATEWAY_CONTRACT_ADDRESS=$(cat $CHAIN_DATA/gateway_contract_address)
            MSG=$1
            "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$MSG"  --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace             
          '';
        };
        centauri-tx = pkgs.writeShellApplication {
          name = "centaurid-xcvm-config";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ centaurid pkgs.jq ];

          text = ''
            CHAIN_DATA="${devnet-root-directory}/.centaurid"
            CHAIN_ID="centauri-dev"
            KEYRING_TEST="$CHAIN_DATA/keyring-test"
            PORT=26657
            FEE=ppica 
            BINARY=centaurid
            "$BINARY" tx ibc-transfer transfer transfer channel-0 osmo1x99pkz8mk7msmptegg887wy46vrusl7kk0sudvaf2uh2k8qz7spsyy4mg8 9876543210ppica --memo '{ "wasm" : { "contract" : "osmo1x99pkz8mk7msmptegg887wy46vrusl7kk0sudvaf2uh2k8qz7spsyy4mg8" } }' --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level trace --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace             
          '';
        };
      };
    };
}
