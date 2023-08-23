{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , devnetTools, ... }: {
      packages = let
        nix-config = ''
          --allow-import-from-derivation --extra-experimental-features "flakes nix-command" --no-sandbox --accept-flake-config --option sandbox relaxed'';
        packages = self'.packages;
        make-bundle = type: package:
          self.inputs.bundlers.bundlers."${system}"."${type}" package;
        subwasm-version = runtime:
          builtins.readFile (pkgs.runCommand "subwasm-version" { } ''
            ${packages.subwasm}/bin/subwasm version ${runtime}/lib/runtime.optimized.wasm | grep specifications | cut -d ":" -f2 | cut -d " " -f3 | head -c -1 > $out
          '');

      in rec {
        generated-release-body = let
          subwasm-call = runtime:
            builtins.readFile (pkgs.runCommand "subwasm-info" { } ''
              ${packages.subwasm}/bin/subwasm info ${runtime}/lib/runtime.optimized.wasm | tail -n+2 | head -c -1 > $out
            '');
          flake-url =
            "github:ComposableFi/composable/release-v${packages.composable-node.version}";
        in pkgs.writeTextFile {
          name = "release.txt";
          text = ''
            ## Runtimes
            ### Picasso
            ```
            ${subwasm-call packages.picasso-runtime}
            ```
            ### Composable
            ```
            ${subwasm-call packages.composable-runtime}
            ```
            ## Nix
            ```bash
            # Generate the Wasm runtimes
            nix build ${flake-url}#picasso-runtime ${nix-config}
            nix build ${flake-url}#composable-runtime ${nix-config}

            # Run the Composable node (release mode) alone
            nix run ${flake-url}#composable-node ${nix-config}

            # Spin up a local devnet
            nix run ${flake-url}#devnet-picasso ${nix-config}
            nix run ${flake-url}#devnet-composable ${nix-config}

            # CW CLI tool
            nix run ${flake-url}#ccw ${nix-config}

            # Spin up a local XC(Inter chain) devnet
            nix run ${flake-url}#devnet-xc-fresh ${nix-config}
            ```
          '';
        };

        tag-release = pkgs.writeShellApplication {
          name = "tag-release";
          runtimeInputs = [ pkgs.git pkgs.yq ];
          text = ''
            git tag --sign "release-v$1" --message "RC" && git push origin "release-v$1" --force
          '';
        };

        delete-release-tag-unsafe = pkgs.writeShellApplication {
          name = "delete-release-tag-unsafe";
          runtimeInputs = [ pkgs.git ];
          text = ''
            # shellcheck disable=SC2015
            git tag --delete "release-v$1" || true && git push --delete origin "release-v$1"
          '';
        };

        generate-release-artifacts = pkgs.writeShellApplication {
          name = "generate-release-artifacts";
          runtimeInputs = devnetTools.withBuildTools;
          text = ''
            mkdir -p release-artifacts/to-upload/

            echo "Generate release body"
            cp ${generated-release-body} release-artifacts/release.txt

            echo "Generate wasm runtimes"
            cp ${packages.picasso-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/picasso_runtime_${
              subwasm-version packages.picasso-runtime
            }.wasm
            cp ${packages.composable-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/composable_runtime_${
              subwasm-version packages.composable-runtime
            }.wasm

            cp ${packages.picasso-testfast-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/picasso_testfast_runtime_${
              subwasm-version packages.picasso-testfast-runtime
            }.wasm

            cp ${packages.composable-testfast-runtime}/lib/runtime.optimized.wasm release-artifacts/to-upload/composable_testfast_runtime_${
              subwasm-version packages.composable-testfast-runtime
            }.wasm


            # XCVM
            cp ${packages.cw-xc-gateway}/lib/cw_xc_gateway.wasm release-artifacts/to-upload/cw_xc_gateway.wasm
            cp ${packages.cw-xc-interpreter}/lib/cw_xc_interpreter.wasm release-artifacts/to-upload/cw_xc_interpreter.wasm

            echo "Generate node packages"
            cp ${
              make-bundle "toRPM" packages.composable-node
            }/*.rpm release-artifacts/to-upload/composable-node-${packages.composable-node.version}-1.x86_64.rpm
            cp ${
              make-bundle "toDEB" packages.composable-node
            }/*.deb release-artifacts/to-upload/composable-node_${packages.composable-node.version}-1_amd64.deb
            cp ${packages.composable-node-image} release-artifacts/composable-image

            cp ${
              make-bundle "toRPM" packages.composable-testfast-node
            }/*.rpm release-artifacts/to-upload/composable-testfast-node-${packages.composable-testfast-node.version}-1.x86_64.rpm
            cp ${
              make-bundle "toDEB" packages.composable-testfast-node
            }/*.deb release-artifacts/to-upload/composable-testfast-node_${packages.composable-testfast-node.version}-1_amd64.deb
            cp ${
              make-bundle "toDockerImage" packages.composable-testfast-node
            } release-artifacts/composable-testfast-node-docker-image

            echo "Devnet"
            cp ${packages.devnet-image} release-artifacts/devnet-image

            echo "Bridge"
            cp ${packages.hyperspace-composable-polkadot-picasso-kusama-image} release-artifacts/hyperspace-composable-polkadot-picasso-kusama-image


            echo "CosmWasm tools"
            cp ${
              make-bundle "toRPM" packages.ccw
            }/*.rpm release-artifacts/to-upload/ccw-${packages.ccw.version}-1.x86_64.rpm
            cp ${
              make-bundle "toDEB" packages.ccw
            }/*.deb release-artifacts/to-upload/ccw_${packages.ccw.version}-1_amd64.deb


            # Checksum everything
            cd release-artifacts/to-upload
            sha256sum ./* > checksums.txt
          '';
        };

        release-xcvm-osmosis = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ packages.osmosisd pkgs.jq ];
          name = "release-xcvm-osmosis";
          text = ''
            if [[ -f .secret/CI_COSMOS_MNEMONIC ]]; then
              CI_COSMOS_MNEMONIC="$(cat .secret/CI_COSMOS_MNEMONIC)"
            fi
            FEE=uosmo
            NETWORK_ID=3
            CHAIN_ID=osmo-test-5
            CI_COSMOS_MNEMONIC="''${1-$CI_COSMOS_MNEMONIC}"
            NETWORK_ID=''${2-$NETWORK_ID}
            DIR=.osmosisd
            BINARY=osmosisd
            NODE=https://rpc.testnet.osmosis.zone:443

            rm --force --recursive .secret/$DIR 
            mkdir --parents .secret/$DIR
            INTERPRETER="${packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm"
            GATEWAY="${packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm"

            echo "$CI_COSMOS_MNEMONIC" | "$BINARY" keys add CI_COSMOS_MNEMONIC --recover --keyring-backend test --home .secret/$DIR --output json

            ADDRESS=$("$BINARY" keys show CI_COSMOS_MNEMONIC --keyring-backend test --home .secret/$DIR --output json | jq -r '.address')

            echo "$ADDRESS"
            GATEWAY=$("$BINARY" tx wasm store "$GATEWAY" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode block)
            echo "$GATEWAY"
            GATEWAY_CODE_ID=$(echo "$GATEWAY" | jq -r '.logs[0].events[1].attributes[1].value')
            echo "$GATEWAY_CODE_ID" > .secret/$DIR/GATEWAY_CODE_ID

            INTERPRETER=$("$BINARY" tx wasm store "$INTERPRETER" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode block)
            echo "$INTERPRETER"

            INTERPRETER_CODE_ID=$(echo "$INTERPRETER" | jq -r '.logs[0].events[1].attributes[1].value')
            echo "$INTERPRETER_CODE_ID" > .secret/$DIR/INTERPRETER_CODE_ID

            INSTANTIATE=$(cat << EOF
                {
                    "admin" : "$ADDRESS", 
                    "network_id" : $NETWORK_ID
                }                                 
            EOF
            )

            INSTANTIATE=$("$BINARY" tx wasm instantiate "$GATEWAY_CODE_ID" "$INSTANTIATE" --label "xc-gateway-2" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1$FEE --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode block --admin "$ADDRESS")
            echo "$INSTANTIATE"
            GATEWAY_CONTRACT_ADDRESS=$(echo "$INSTANTIATE" | jq -r '.logs[0].events[] | select(.type == "instantiate") | .attributes[0].value')
            echo "$GATEWAY_CONTRACT_ADDRESS" > .secret/$DIR/GATEWAY_CONTRACT_ADDRESS
          '';
        };

        release-xcvm-centauri = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ packages.centaurid pkgs.jq ];
          name = "release-xcvm-centauri";
          text = ''
            FEE=ppica
            NETWORK_ID=2
            CHAIN_ID=banksy-testnet-3
            DIR=.centaurid
            BINARY=centaurid
            NODE=https://rpc-t.composable.nodestake.top:443
                      
            if [[ -f .secret/CI_COSMOS_MNEMONIC ]]; then
              CI_COSMOS_MNEMONIC="$(cat .secret/CI_COSMOS_MNEMONIC)"
            fi            
            CI_COSMOS_MNEMONIC="''${1-$CI_COSMOS_MNEMONIC}"            
            NETWORK_ID=''${2-$NETWORK_ID}
            BLOCK_TIME=6
            rm --force --recursive .secret/$DIR 
            mkdir --parents .secret/$DIR

            INTERPRETER="${packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm"
            GATEWAY="${packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm"

            echo "$CI_COSMOS_MNEMONIC" | "$BINARY" keys add CI_COSMOS_MNEMONIC --recover --keyring-backend test --home .secret/$DIR --output json
            ADDRESS=$("$BINARY" keys show CI_COSMOS_MNEMONIC --keyring-backend test --home .secret/$DIR --output json | jq -r '.address')
            echo "$ADDRESS" > .secret/$DIR/ADDRESS

            GATEWAY_TX=$("$BINARY" tx wasm store "$GATEWAY" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode sync)
            echo "$GATEWAY_TX"
            GATEWAY_HASH=$(sha256sum < "$GATEWAY" | head -c 64 | tr "[:lower:]" "[:upper:]")

            INTERPRETER_HASH=$(sha256sum < "$INTERPRETER" | head -c 64 | tr "[:lower:]" "[:upper:]")

            sleep $BLOCK_TIME
            echo "$GATEWAY_HASH"
            GATEWAY_CODE_ID=$("$BINARY" query wasm list-code --home .secret/$DIR --output json --node "$NODE" | jq -r ".code_infos[] | select(.data_hash == \"$GATEWAY_HASH\" and .creator == \"$ADDRESS\" ) | .code_id " | tail --lines 1)
            echo "$GATEWAY_CODE_ID" > .secret/$DIR/GATEWAY_CODE_ID


            INTERPRETER_TX=$("$BINARY" tx wasm store "$INTERPRETER" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1uosmo --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode sync)
            echo "$INTERPRETER_TX"

            echo "$INTERPRETER_HASH"
            sleep $BLOCK_TIME
            INTERPRETER_CODE_ID=$("$BINARY" query wasm list-code --home .secret/$DIR --output json --node "$NODE" | jq -r ".code_infos[] | select(.data_hash == \"$INTERPRETER_HASH\" and .creator == \"$ADDRESS\" ) | .code_id " | tail --lines 1)
            echo "$INTERPRETER_CODE_ID" > .secret/$DIR/INTERPRETER_CODE_ID

            INSTANTIATE=$(cat << EOF
                {
                    "admin" : "$ADDRESS", 
                    "network_id" : $NETWORK_ID
                }                                 
            EOF
            )

            INSTANTIATE=$("$BINARY" tx wasm instantiate "$GATEWAY_CODE_ID" "$INSTANTIATE" --label "xc-gateway-2" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1$FEE --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode sync --admin "$ADDRESS")
            echo "$INSTANTIATE"

            GATEWAY_CONTRACT_ADDRESS=$("$BINARY" query wasm list-contract-by-code 23 --home .secret/$DIR --output json --node "$NODE"  | jq -r ".contracts | .[-1]")
            echo "$GATEWAY_CONTRACT_ADDRESS" > .secret/$DIR/GATEWAY_CONTRACT_ADDRESS
          '';
        };

        release-xcvm-config = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ packages.centaurid pkgs.jq packages.osmosisd ];
          name = "release-xcvm-centauri";
          text = ''
            FEE=ppica
            CHAIN_ID=banksy-testnet-3
            DIR=.centaurid
            BINARY=centaurid
            NODE=https://rpc-t.composable.nodestake.top:443
                      
            if [[ -f .secret/CI_COSMOS_MNEMONIC ]]; then
              CI_COSMOS_MNEMONIC="$(cat .secret/CI_COSMOS_MNEMONIC)"
            fi            
            CI_COSMOS_MNEMONIC="''${1-$CI_COSMOS_MNEMONIC}"            
            BLOCK_TIME=6
            GATEWAY_CONTRACT_ADDRESS=$(cat .secret/$DIR/GATEWAY_CONTRACT_ADDRESS)
            OSMOSIS_GATEWAY_CONTRACT_ADDRESS=$(cat .secret/.osmosisd/GATEWAY_CONTRACT_ADDRESS)
            INTERPRETER_CODE_ID=$(cat .secret/$DIR/INTERPRETER_CODE_ID)
            OSMOSIS_INTERPRETER_CODE_ID=$(cat .secret/.osmosisd/INTERPRETER_CODE_ID)
            ADDRESS=$("$BINARY" keys show CI_COSMOS_MNEMONIC --keyring-backend test --home .secret/$DIR --output json | jq -r '.address')
            OSMOSIS_ADDRESS=$(osmosisd keys show CI_COSMOS_MNEMONIC --keyring-backend test --home .secret/.osmosisd --output json | jq -r '.address')

            FORCE=$(cat << EOF
            {
              "config": {
                "force": [
                  {
                    "force_network": {
                      "network_id": 2,
                      "accounts": {
                          "bech": "centauri"
                      },
                      "gateway": {
                          "cosm_wasm": {
                            "contract": "$GATEWAY_CONTRACT_ADDRESS",
                            "interpreter_code_id": $INTERPRETER_CODE_ID,
                            "admin": "$ADDRESS"
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
                  },
                  {
                    "force_network": {
                      "network_id": 3,
                      "accounts": {
                          "bech": "osmo"
                      },
                      "gateway": {
                          "cosm_wasm": {
                            "contract": "$OSMOSIS_GATEWAY_CONTRACT_ADDRESS",
                            "interpreter_code_id": $OSMOSIS_INTERPRETER_CODE_ID,
                            "admin": "$OSMOSIS_ADDRESS"
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
                  },
                  
                  {
                    "force_network_to_network": {
                      "from": 2,
                      "to": 3,
                      "other": {
                          "counterparty_timeout": {
                            "seconds" : 120
                          },
                          "ics_20": {
                            "source" : "channel-20", 
                            "sink" : "channel-124" 
                          }                                                
                      }
                    }                    
                  },
                  
                  {
                    "force_network_to_network": {
                      "from": 3,
                      "to": 2,
                      "other": {
                          "counterparty_timeout": {
                            "seconds" : 120
                          },
                          "ics_20": {
                            "source" : "channel-124", 
                            "sink" : "channel-20" 
                          }                                                
                      }
                    }                    
                  },

                  {
                    "force_asset": {
                      "asset_id": "158456325028528675187087900673",
                      "network_id": 2,
                      "local": {
                        "native": {
                          "denom": "ppica"
                        }
                      }
                    }                    
                  },
                  {
                    "force_asset": {
                      "asset_id": "237684487542793012780631851010",
                      "network_id": 3,
                      "local": {
                        "native": {
                          "denom" : "uatom"
                        }
                      }
                    }
                  },                  

                  {
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
                            "trace_path" : "transfer/channel-20"
                          }
                        }
                      }                      
                    }                    
                  },
                  {
                    "force_asset": {
                      "asset_id": "237684487542793012780631851009",
                      "network_id": 3,
                      "local": {
                        "native": {
                          "denom": "ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B"
                        }
                      },
                      "bridged": {
                        "location_on_network": {
                          "ibc_ics20": {
                            "base_denom" : "ppica",
                            "trace_path" : "transfer/channel-124"
                          }
                        }
                      }                      
                    }                    
                  },
                  {
                    "force_asset": {
                      "asset_id": "158456325028528675187087901673",
                      "network_id": 2,
                      "local": {
                        "native": {
                          "denom": "ibc/0471F1C4E7AFD3F07702BEF6DC365268D64570F7C1FDC98EA6098DD6DE59817B"
                        }
                      },
                      "bridged": {
                        "location_on_network": {
                          "ibc_ics20": {
                            "base_denom" : "uosmo",
                            "trace_path" : "transfer/channel-20"
                          }
                        }
                      }                      
                    }                    
                  },                      
                  {
                    "force_asset_to_network_map": {
                      "this_asset": "158456325028528675187087900673",
                      "other_network": 3,
                      "other_asset": "237684487542793012780631851009"          
                    }                    
                  }

                ]                  
              }
            }               
            EOF
            )

            "$BINARY" tx wasm execute "$GATEWAY_CONTRACT_ADDRESS" "$FORCE" --keyring-backend test --home .secret/$DIR --output json --node "$NODE" --from CI_COSMOS_MNEMONIC --gas-prices 0.1$FEE --gas auto --gas-adjustment 1.3 --chain-id "$CHAIN_ID" --yes --broadcast-mode sync
            sleep $BLOCK_TIME

            "$BINARY" query wasm contract-state all "$GATEWAY_CONTRACT_ADDRESS" --chain-id="$CHAIN_ID"  --node "$NODE" --output json --home .secret/$DIR
          '';
        };

      };
    };

}
