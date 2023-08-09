{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, ... }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      validator-key = cosmosTools.validators.osmosis;
    in {
      packages = rec {
        osmosisd = pkgs.writeShellApplication {
          name = "osmosisd";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            ${self.inputs.cosmos.packages.${system}.osmosis}/bin/osmosisd "$@"
          '';
        };

        osmosisd-gen = pkgs.writeShellApplication {
          name = "osmosisd-gen";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ osmosisd pkgs.jq ];
          text = ''
            HOME=${devnet-root-directory}
            export HOME
            CHAIN_DATA="$HOME/.osmosisd"
            if test "''${1-reuse}" == "fresh" ; then
             echo "removing data dir"
             rm --force --recursive "$CHAIN_DATA" 
            fi

            PORT=36657
            KEYRING_TEST=$CHAIN_DATA
            CHAIN_ID="osmosis-dev"
            VALIDATOR_MONIKER="${cosmosTools.validators.moniker}"
            VALIDATOR_MNEMONIC="${cosmosTools.validators.mnemonic}"
            FAUCET_MNEMONIC="increase bread alpha rigid glide amused approve oblige print asset idea enact lawn proof unfold jeans rabbit audit return chuckle valve rather cactus great"
            RELAYER_MNEMONIC="black frequent sponsor nice claim rally hunt suit parent size stumble expire forest avocado mistake agree trend witness lounge shiver image smoke stool chicken"
            CONFIG_FOLDER=$CHAIN_DATA/config
            GENESIS=$CONFIG_FOLDER/genesis.json
            mkdir --parents "$CHAIN_DATA/data/cs.wal"

            echo "$VALIDATOR_MNEMONIC" | osmosisd init --chain-id="$CHAIN_ID" --home "$CHAIN_DATA" --recover "$VALIDATOR_MONIKER"

            function dasel-genesis() {
              dasel put --type string --file "$GENESIS" --value "$2" "$1"   
            }             

            dasel-genesis '.app_state.staking.params.bond_denom' 'uosmo'
            dasel-genesis '.app_state.staking.params.unbonding_time' '120s'
            dasel  put --type json --file "$GENESIS" --value "[{},{}]" 'app_state.bank.denom_metadata'
            dasel-genesis '.app_state.bank.denom_metadata.[0].description' 'Registered denom uion for localosmosis testing'
            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.bank.denom_metadata.[0].denom_units'
            dasel-genesis '.app_state.bank.denom_metadata.[0].denom_units.[0].denom' 'uion'
            dasel-genesis '.app_state.bank.denom_metadata.[0].denom_units.[0].exponent' 0
            dasel-genesis '.app_state.bank.denom_metadata.[0].base' 'uion'
            dasel-genesis '.app_state.bank.denom_metadata.[0].display' 'uion'
            dasel-genesis '.app_state.bank.denom_metadata.[0].name' 'uion'
            dasel-genesis '.app_state.bank.denom_metadata.[0].symbol' 'uion'

            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.bank.denom_metadata.[1].denom_units'
            dasel-genesis '.app_state.bank.denom_metadata.[1].description' 'Registered denom uosmo for localosmosis testing'
            dasel-genesis '.app_state.bank.denom_metadata.[1].denom_units.[0].denom' 'uosmo'
            dasel-genesis '.app_state.bank.denom_metadata.[1].denom_units.[0].exponent' 0
            dasel-genesis '.app_state.bank.denom_metadata.[1].base' 'uosmo'
            dasel-genesis '.app_state.bank.denom_metadata.[1].display' 'uosmo'
            dasel-genesis '.app_state.bank.denom_metadata.[1].name' 'uosmo'
            dasel-genesis '.app_state.bank.denom_metadata.[1].symbol' 'uosmo'

            dasel-genesis '.app_state.crisis.constant_fee.denom' 'uosmo'
            dasel-genesis '.app_state.gov.voting_params.voting_period' '30s'
            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.gov.deposit_params.min_deposit'
            dasel-genesis '.app_state.gov.deposit_params.min_deposit.[0].denom' 'uosmo'
            dasel-genesis '.app_state.gov.deposit_params.min_deposit.[0].amount' '1000000000'
            dasel-genesis '.app_state.epochs.epochs.[1].duration' "60s"
            dasel  put --type json --file "$GENESIS" --value "[{},{},{}]" '.app_state.poolincentives.lockable_durations'
            dasel-genesis '.app_state.poolincentives.lockable_durations.[0]' "120s"
            dasel-genesis '.app_state.poolincentives.lockable_durations.[1]' "180s"
            dasel-genesis '.app_state.poolincentives.lockable_durations.[2]' "240s"
            dasel-genesis '.app_state.poolincentives.params.minted_denom' "uosmo"
            dasel  put --type json --file "$GENESIS" --value "[{},{},{},{}]" '.app_state.incentives.lockable_durations'
            dasel-genesis '.app_state.incentives.lockable_durations.[0]' "1s"
            dasel-genesis '.app_state.incentives.lockable_durations.[1]' "120s"
            dasel-genesis '.app_state.incentives.lockable_durations.[2]' "180s"
            dasel-genesis '.app_state.incentives.lockable_durations.[3]' "240s"
            dasel-genesis '.app_state.incentives.params.distr_epoch_identifier' "hour"
            dasel-genesis '.app_state.mint.params.mint_denom' "uosmo"
            dasel-genesis '.app_state.mint.params.epoch_identifier' "day"
            dasel-genesis '.app_state.poolmanager.params.pool_creation_fee.[0].denom' "uosmo"

            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.gamm.params.pool_creation_fee'
            dasel-genesis '.app_state.gamm.params.pool_creation_fee.[0].denom' "uosmo"
            dasel-genesis '.app_state.gamm.params.pool_creation_fee.[0].amount' "10000000"
            dasel-genesis '.app_state.txfees.basedenom' "uosmo"
            dasel-genesis '.app_state.wasm.params.code_upload_access.permission' "Everybody"            

            function add-genesis-account() {
              echo "$1" | osmosisd keys add "$2" --recover --keyring-backend test --home "$CHAIN_DATA" --keyring-dir "$KEYRING_TEST"
              ACCOUNT=$(osmosisd keys show --address "$2" --keyring-backend test --home "$CHAIN_DATA" )
              echo "===================================="
              echo "$ACCOUNT"
              osmosisd add-genesis-account "$ACCOUNT" 100000000000uosmo,100000000000uion,100000000000stake --home "$CHAIN_DATA"
            }

            add-genesis-account "$VALIDATOR_MNEMONIC" "$VALIDATOR_MONIKER"
            add-genesis-account "$FAUCET_MNEMONIC" "faucet"
            add-genesis-account "$RELAYER_MNEMONIC" "relayer"
            add-genesis-account "${cosmosTools.xcvm.mnemonic}" "xcvm"

            osmosisd gentx $VALIDATOR_MONIKER 500000000uosmo --keyring-backend=test --chain-id=$CHAIN_ID --home "$CHAIN_DATA" 
            osmosisd collect-gentxs --home "$CHAIN_DATA"
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "" '.p2p.seeds'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:$PORT" '.rpc.laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "0.0.0.0:16060" '.rpc.pprof_laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://127.0.0.1:36658" '.proxy_app'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value ":36660" '.instrumentation.prometheus_listen_addr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:36656" '.p2p.laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://localhost:$PORT" '.node'

            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "0.0.0.0:19090" '.grpc.address'
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "0.0.0.0:19091" '.grpc-web.address'
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "tcp://0.0.0.0:11317" '.api.address'

            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "*" '.rpc.cors_allowed_origins.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "Accept-Encoding" '.rpc.cors_allowed_headers.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "DELETE" '.rpc.cors_allowed_methods.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "OPTIONS" '.rpc.cors_allowed_methods.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "PATCH" '.rpc.cors_allowed_methods.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "PUT" '.rpc.cors_allowed_methods.[]'
            dasel put --type bool --file $CONFIG_FOLDER/app.toml --value "true" '.api.swagger'
            dasel put --type bool --file $CONFIG_FOLDER/app.toml --value "true" '.api.enabled-unsafe-cors'
            dasel put --type bool --file $CONFIG_FOLDER/app.toml --value "true" '.grpc-web.enable-unsafe-cors'

            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "tcp://localhost:$PORT" '.node'
            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "$CHAIN_ID" '.chain-id'
            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "test" '.keyring-backend'
            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "json" '.output'

            osmosisd start --home "$CHAIN_DATA" --rpc.unsafe --rpc.laddr tcp://0.0.0.0:$PORT --pruning=nothing --grpc.address localhost:19090 --address "tcp://0.0.0.0:36658" --p2p.external-address 43421 --p2p.laddr "tcp://0.0.0.0:36656" --p2p.pex false --p2p.upnp  false  --p2p.seed_mode true --log_level trace --trace
          '';
        };

        osmosisd-gen-fresh = pkgs.writeShellApplication {
          name = "osmosisd-gen-fresh";
          runtimeInputs = [ osmosisd-gen ];
          text = ''
            rm --force --recursive ${devnet-root-directory} 
            osmosisd-gen
          '';
        };

        osmosisd-init = pkgs.writeShellApplication {
          name = "osmosisd-init";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ osmosisd pkgs.jq pkgs.dasel ];
          text = ''
            HOME=/tmp/composable-devnet
            export HOME
            CHAIN_DATA="$HOME/.osmosisd"             
            KEYRING_TEST=$CHAIN_DATA
            CHAIN_ID="osmosis-dev"            
            PORT=36657
            BLOCK_SECONDS=5
            FEE=uosmo
            NETWORK_ID=3
            KEY=${cosmosTools.xcvm.osmosis}
            BINARY=osmosisd

            function init_xcvm() {              
              local INSTANTIATE=$1
              echo $NETWORK_ID
              "$BINARY" tx wasm store  "${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166$FEE --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"
              GATEWAY_CODE_ID=1

              sleep $BLOCK_SECONDS
              "$BINARY" tx wasm store  "${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166$FEE --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"
              INTERPRETER_CODE_ID=2

              sleep $BLOCK_SECONDS
              "$BINARY" tx wasm store  "${self'.packages.cw20_base}" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166$FEE --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"

              sleep $BLOCK_SECONDS
             
              "$BINARY" tx wasm instantiate2 $GATEWAY_CODE_ID "$INSTANTIATE" "1234" --label "xc-gateway" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166$FEE --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --admin "$KEY"

              sleep $BLOCK_SECONDS
              GATEWAY_CONTRACT_ADDRESS=$("$BINARY" query wasm list-contract-by-code "$GATEWAY_CODE_ID" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --home "$CHAIN_DATA" | dasel --read json '.contracts.[0]' --write yaml)      
              echo "$GATEWAY_CONTRACT_ADDRESS" > "$CHAIN_DATA/gateway_contract_address"
            }

            INSTANTIATE=$(cat << EOF
                {
                    "admin" : "$KEY", 
                    "here_id" : $NETWORK_ID
                }                                 
            EOF
            )

            init_xcvm "$INSTANTIATE"

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
            "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_NETWORK_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --trace --log_level trace             

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
            "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_NETWORK_CENTAURI" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --trace --log_level trace    


            sleep $BLOCK_SECONDS
            FORCE_CENTAURI_TO_OSMOSIS=$(cat << EOF
              {
                "config": {
                    "force_network_to_network": {
                      "from": 2,
                      "to": 3,
                      "other": {
                          "counterparty_timeout": {
                            "timestamp": "60"
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
            "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_CENTAURI_TO_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --trace --log_level trace


            sleep $BLOCK_SECONDS
            FORCE_PICA=$(cat << EOF
            {
              "config": {
                "force_asset": {
                  "asset_id": "79228162514264337593543950337",
                  "from_network_id": 2,
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
            "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_PICA" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --trace --log_level trace


            sleep $BLOCK_SECONDS
            FORCE_OSMO_DIRECT_ON_CENTAURI=$(cat << EOF
              {
                "config": {
                    "force_asset": {
                      "asset_id": "158456325028528675187087901673",
                      "from_network_id": 3,
                      "local": {
                        "native": {
                          "denom" : "uosmo"
                        }
                      }
                    }
                }
              }                                 
            EOF
            )
            "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_OSMO_DIRECT_ON_CENTAURI" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --trace --log_level trace

            sleep $BLOCK_SECONDS
            FORCE_OSMO_ON_OSMOSIS=$(cat << EOF
              {
                "config": {
                    "force_asset": {
                      "asset_id": "237684487542793012780631852009",
                      "from_network_id": 3,
                      "local": {
                        "native": {
                          "denom" : "uosmo"
                        }
                      }
                    }
                }
              }                                 
            EOF
            )
            "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE_OSMO_ON_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --trace --log_level trace


            sleep $BLOCK_SECONDS
            "$BINARY" query wasm contract-state all "$GATEWAY_CONTRACT_ADDRESS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --home "$CHAIN_DATA"
          '';
        };

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
                                                          "intercept": "1000000000",
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
                                  "1000000000"
                              ]
                          ]
                      },
                      "tip": "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n"
                  }
              }
              EOF
              )                  

              "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$TRANSFER_PICA_TO_OSMOSIS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 1000000000"$FEE" --amount 1000000000"$FEE" --log_level info --keyring-backend test  --home "$CHAIN_DATA" --from ${cosmosTools.xcvm.moniker} --keyring-dir "$KEYRING_TEST" --trace --log_level trace
              sleep "$BLOCK_SECONDS"
            '';
          };

      };
    };
}
