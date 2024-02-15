{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, bashTools, ... }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      devnet = pkgs.networksLib.osmosis.devnet;
      log = " --log_level trace --trace ";
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
            ${bashTools.export pkgs.networksLib.devnet.mnemonics}
            ${bashTools.export devnet}
            HOME=${devnet-root-directory}
            export HOME
            CHAIN_DATA="$HOME/.osmosisd"
            if test "''${1-reuse}" == "fresh" ; then
             echo "removing data dir"
             rm --force --recursive "$CHAIN_DATA" 
            fi

            KEYRING_TEST=$CHAIN_DATA
            CHAIN_ID="osmosis-dev"
            VALIDATOR_MONIKER="${cosmosTools.validators.moniker}"
            VALIDATOR_MNEMONIC="${cosmosTools.validators.mnemonic}"
            FAUCET_MNEMONIC="increase bread alpha rigid glide amused approve oblige print asset idea enact lawn proof unfold jeans rabbit audit return chuckle valve rather cactus great"
            CONFIG_FOLDER=$CHAIN_DATA/config
            GENESIS=$CONFIG_FOLDER/genesis.json
            mkdir --parents "$CHAIN_DATA/data/cs.wal"

            echo "$VALIDATOR_MNEMONIC" | osmosisd init --chain-id="$CHAIN_ID" --home "$CHAIN_DATA" --recover "$VALIDATOR_MONIKER"

            function dasel-genesis() {
              dasel put --type string --file "$GENESIS" --value "$2" "$1"   
            }             

            dasel-genesis '.app_state.staking.params.bond_denom' 'uosmo'
            dasel-genesis '.app_state.staking.params.unbonding_time' '960s'
            dasel  put --type json --file "$GENESIS" --value "[{},{},{}]" 'app_state.bank.denom_metadata'

            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.bank.denom_metadata.[0].denom_units'
            dasel-genesis '.app_state.bank.denom_metadata.[0].description' 'Registered denom uion for localosmosis testing'
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

            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.bank.denom_metadata.[2].denom_units'
            dasel-genesis '.app_state.bank.denom_metadata.[2].description' 'ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B'
            dasel-genesis '.app_state.bank.denom_metadata.[2].denom_units.[0].denom' 'ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B'
            dasel-genesis '.app_state.bank.denom_metadata.[2].denom_units.[0].exponent' 0
            dasel-genesis '.app_state.bank.denom_metadata.[2].base' 'ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B'
            dasel-genesis '.app_state.bank.denom_metadata.[2].display' 'ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B'
            dasel-genesis '.app_state.bank.denom_metadata.[2].name' 'ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B'
            dasel-genesis '.app_state.bank.denom_metadata.[2].symbol' 'ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B'

            dasel  put --type string --file "$GENESIS" --value "transfer" '.app_state.transfer.port_id'
            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.transfer.denom_traces'
            dasel  put --type string --file "$GENESIS" --value "transfer/channel-0" '.app_state.transfer.denom_traces.[0].path'
            dasel  put --type string --file "$GENESIS" --value "ppica" '.app_state.transfer.denom_traces.[0].base_denom'

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
              osmosisd add-genesis-account "$ACCOUNT" 100000000000000000uosmo,100000000000uion,100000000000stake,10000000000000ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B --home "$CHAIN_DATA"
            }

            add-genesis-account "$VALIDATOR_MNEMONIC" "$VALIDATOR_MONIKER"
            add-genesis-account "$FAUCET_MNEMONIC" "faucet"
            add-genesis-account "$RLY_MNEMONIC_3" "relayer"
            add-genesis-account "$RLY_MNEMONIC_4" "relayer4"
            add-genesis-account "$APPLICATION1" "cvm"
            add-genesis-account "${cosmosTools.pools.mnemonic}" "pools"

            osmosisd gentx $VALIDATOR_MONIKER 500000000uosmo --keyring-backend=test --chain-id=$CHAIN_ID --home "$CHAIN_DATA" 
            osmosisd collect-gentxs --home "$CHAIN_DATA"
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "" '.p2p.seeds'

            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "*" '.rpc.cors_allowed_origins.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "Accept-Encoding" '.rpc.cors_allowed_headers.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "DELETE" '.rpc.cors_allowed_methods.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "OPTIONS" '.rpc.cors_allowed_methods.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "PATCH" '.rpc.cors_allowed_methods.[]'
            dasel put --type string --file $CONFIG_FOLDER/config.toml --value "PUT" '.rpc.cors_allowed_methods.[]'
            dasel put --type bool --file $CONFIG_FOLDER/app.toml --value "true" '.api.swagger'
            dasel put --type bool --file $CONFIG_FOLDER/app.toml --value "true" '.api.enabled-unsafe-cors'
            dasel put --type bool --file $CONFIG_FOLDER/app.toml --value "true" '.grpc-web.enable-unsafe-cors'

            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "$CHAIN_ID" '.chain-id'
            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "test" '.keyring-backend'
            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "json" '.output'

            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "0.0.0.0:$GRPCPORT" '.grpc.address'
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "0.0.0.0:$GRPCWEB" '.grpc-web.address'
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "tcp://0.0.0.0:$RESTPORT" '.api.address'

            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value ":$PROMETHEUS_PORT" '.instrumentation.prometheus_listen_addr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "0.0.0.0:16060" '.rpc.pprof_laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:$P2PPORT" '.p2p.laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:$CONSENSUS_RPC_PORT" '.rpc.laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://127.0.0.1:36658" '.proxy_app'

            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "tcp://localhost:$CONSENSUS_RPC_PORT" '.node'

            osmosisd start --home "$CHAIN_DATA" --rpc.unsafe --pruning=nothing --p2p.pex false --p2p.upnp false --p2p.seed_mode true ${log} --minimum-gas-prices=0.00001uosmo
          '';
        };

        osmosis-cvm-init = pkgs.writeShellApplication {
          name = "osmosis-cvm-init";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ osmosisd pkgs.jq pkgs.dasel ];
          text = ''
            ${bashTools.export pkgs.networksLib.osmosis.devnet}
            # [osmosis-cvm-init	] OsmosisApp is not ready; please wait for first block: invalid height
            sleep 16
            KEY=${cosmosTools.cvm.osmosis}

            function init_cvm() {              
              local INSTANTIATE=$1
              echo $NETWORK_ID
              "$BINARY" tx wasm store  "${
                self.inputs.cvm.packages."${system}".cw-cvm-outpost
              }/lib/cw_cvm_outpost.wasm" --chain-id="$CHAIN_ID"  --node="tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --yes --gas 25000000 --fees 920000166$FEE --log_level=info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"
              GATEWAY_CODE_ID=1

              sleep "$BLOCK_SECONDS"
              "$BINARY" tx wasm store  "${
                self.inputs.cvm.packages."${system}".cw-cvm-executor
              }/lib/cw_cvm_executor.wasm" --chain-id="$CHAIN_ID"  --node="tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --yes --gas 25000000 --fees 920000166$FEE --log_level=info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"
              EXECUTOR_CODE_ID=2

              sleep "$BLOCK_SECONDS"
              "$BINARY" tx wasm store  ${
                self.inputs.cosmos.packages.${system}.cw20-base
              }/lib/cw20_base.wasm --chain-id="$CHAIN_ID"  --node="tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --yes --gas 25000000 --fees 920000166$FEE --log_level=info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"

              sleep "$BLOCK_SECONDS"
             
              "$BINARY" tx wasm instantiate2 $GATEWAY_CODE_ID "$INSTANTIATE" "1234" --label "composable_cvm_outpost" --chain-id="$CHAIN_ID"  --node="tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --yes --gas 25000000 --fees 920000166$FEE --log_level=info --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --admin "$KEY"

              sleep "$BLOCK_SECONDS"
              OUTPOST_CONTRACT_ADDRESS=$("$BINARY" query wasm list-contract-by-code "$GATEWAY_CODE_ID" --chain-id="$CHAIN_ID"  --node="tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --home "$CHAIN_DATA" | dasel --read json '.contracts.[0]' --write yaml)                    
              echo "$OUTPOST_CONTRACT_ADDRESS" | tee "$CHAIN_DATA/outpost_contract_address"
              echo "$EXECUTOR_CODE_ID" > "$CHAIN_DATA/executor_code_id"
            }

            INSTANTIATE=$(cat << EOF
                {
                    "admin" : "$KEY", 
                    "network_id" : $NETWORK_ID
                }                                 
            EOF
            )

            init_cvm "$INSTANTIATE"           
          '';
        };

        osmosisd-pools-init = pkgs.writeShellApplication {
          name = "osmosisd-pools-init";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ osmosisd pkgs.jq pkgs.dasel ];
          text = ''
            ${bashTools.export pkgs.networksLib.osmosis.devnet}

            "$BINARY" tx gamm create-pool --pool-file=${
              ./osmosis-gamm-pool-pica-osmo.json
            } --chain-id="$CHAIN_ID"  --node="tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --yes --gas=25000000 --fees=920000166"$FEE" --keyring-backend=test  --home="$CHAIN_DATA" --from=pools --keyring-dir="$KEYRING_TEST" ${log} --broadcast-mode=sync
            sleep "$BLOCK_SECONDS"
            "$BINARY" query gamm pools --chain-id="$CHAIN_ID"  --node="tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --home="$CHAIN_DATA"           
          '';
        };

        osmosisd-cvm-config = pkgs.writeShellApplication {
          name = "osmosisd-cvm-config";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ osmosisd pkgs.jq pkgs.dasel ];
          text = ''
            ${bashTools.export pkgs.networksLib.osmosis.devnet}
            KEY=${cosmosTools.cvm.osmosis}

            CENTAURI_OUTPOST_CONTRACT_ADDRESS=$(cat ${pkgs.networksLib.pica.devnet.CHAIN_DATA}/outpost_contract_address)        
            CENTAURI_EXECUTOR_CODE_ID=$(cat ${pkgs.networksLib.pica.devnet.CHAIN_DATA}/executor_code_id)
            OSMOSIS_OUTPOST_CONTRACT_ADDRESS=$(cat "$HOME/.osmosisd/outpost_contract_address")
            OSMOSIS_EXECUTOR_CODE_ID=$(cat "$HOME/.osmosisd/executor_code_id")
            NEUTRON_OUTPOST_CONTRACT_ADDRESS=$(cat "${pkgs.networksLib.pica.devnet.CHAIN_DATA}/outpost_contract_address")
            NEUTRON_EXECUTOR_CODE_ID=$(cat "${pkgs.networksLib.pica.devnet.CHAIN_DATA}/executor_code_id")

            FORCE_CONFIG=$(cat << EOF
              ${builtins.readFile ../cvm.json}
            EOF
            )
            "$BINARY" tx wasm execute "$OSMOSIS_OUTPOST_CONTRACT_ADDRESS" "$FORCE_CONFIG" --chain-id="$CHAIN_ID"  --node="tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --yes --gas 25000000 --fees 920000166"$FEE" --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" ${log}             


            sleep "$BLOCK_SECONDS"
            "$BINARY" query wasm contract-state all "$OSMOSIS_OUTPOST_CONTRACT_ADDRESS" --chain-id="$CHAIN_ID"  --node="tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --home "$CHAIN_DATA"
          '';
        };
      };
    };
}
