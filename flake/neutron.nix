{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, bashTools, ... }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      validator-key = cosmosTools.validators.neutron;
      env = {
        mainnet = {
          FEE = "uosmo";
          NETWORK_ID = 3;
          CHAIN_ID = "neutron-1";
          DIR = "prod/.neutrond";
          BINARY = "neutrond";
          BLOCK_SECONDS = 6;
          NODE = "https://rpc.neutron.zone:443";
        };
        devnet = rec {
          HOME = "/tmp/composable-devnet";
          CHAIN_DATA = "${HOME}/.neutrond";
          KEYRING_TEST = CHAIN_DATA;
          CHAIN_ID = "neutron-dev";
          PORT = 36657;
          BLOCK_SECONDS = 5;
          FEE = "uosmo";
          BINARY = "neutrond";
        };
      };
    in {
      _module.args.neutron = rec { inherit env; };

      packages = rec {
        neutrond = pkgs.writeShellApplication {
          name = "neutrond";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            ${self.inputs.cosmos.packages.${system}.neutron}/bin/neutrond "$@"
          '';
        };

        neutrond-gen = pkgs.writeShellApplication {
          name = "neutrond-gen";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ neutrond pkgs.jq ];

          text = ''
            HOME=${devnet-root-directory}
            export HOME
            CHAIN_DATA="$HOME/.neutrond"
            if test "''${1-reuse}" == "fresh" ; then
             echo "removing data dir"
             rm --force --recursive "$CHAIN_DATA" 
            fi

            PORT=36657
            KEYRING_TEST=$CHAIN_DATA
            CHAIN_ID="neutron-dev"
            VALIDATOR_MONIKER="${cosmosTools.validators.moniker}"
            VALIDATOR_MNEMONIC="${cosmosTools.validators.mnemonic}"
            FAUCET_MNEMONIC="increase bread alpha rigid glide amused approve oblige print asset idea enact lawn proof unfold jeans rabbit audit return chuckle valve rather cactus great"
            RELAYER_MNEMONIC="black frequent sponsor nice claim rally hunt suit parent size stumble expire forest avocado mistake agree trend witness lounge shiver image smoke stool chicken"
            CONFIG_FOLDER=$CHAIN_DATA/config
            GENESIS=$CONFIG_FOLDER/genesis.json
            mkdir --parents "$CHAIN_DATA/data/cs.wal"

            echo "$VALIDATOR_MNEMONIC" | neutrond init --chain-id="$CHAIN_ID" --home "$CHAIN_DATA" --recover "$VALIDATOR_MONIKER"

            function dasel-genesis() {
              dasel put --type string --file "$GENESIS" --value "$2" "$1"   
            }             

            dasel-genesis '.app_state.staking.params.bond_denom' 'uosmo'
            dasel-genesis '.app_state.staking.params.unbonding_time' '960s'
            dasel  put --type json --file "$GENESIS" --value "[{},{},{}]" 'app_state.bank.denom_metadata'

            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.bank.denom_metadata.[0].denom_units'
            dasel-genesis '.app_state.bank.denom_metadata.[0].description' 'Registered denom uion for localneutron testing'
            dasel-genesis '.app_state.bank.denom_metadata.[0].denom_units.[0].denom' 'uion'
            dasel-genesis '.app_state.bank.denom_metadata.[0].denom_units.[0].exponent' 0
            dasel-genesis '.app_state.bank.denom_metadata.[0].base' 'uion'
            dasel-genesis '.app_state.bank.denom_metadata.[0].display' 'uion'
            dasel-genesis '.app_state.bank.denom_metadata.[0].name' 'uion'
            dasel-genesis '.app_state.bank.denom_metadata.[0].symbol' 'uion'

            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.bank.denom_metadata.[1].denom_units'
            dasel-genesis '.app_state.bank.denom_metadata.[1].description' 'Registered denom uosmo for localneutron testing'
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
              echo "$1" | neutrond keys add "$2" --recover --keyring-backend test --home "$CHAIN_DATA" --keyring-dir "$KEYRING_TEST"
              ACCOUNT=$(neutrond keys show --address "$2" --keyring-backend test --home "$CHAIN_DATA" )
              echo "===================================="
              echo "$ACCOUNT"
              neutrond add-genesis-account "$ACCOUNT" 100000000000000000uosmo,100000000000uion,100000000000stake,10000000000000ibc/3262D378E1636BE287EC355990D229DCEB828F0C60ED5049729575E235C60E8B --home "$CHAIN_DATA"
            }

            add-genesis-account "$VALIDATOR_MNEMONIC" "$VALIDATOR_MONIKER"
            add-genesis-account "$FAUCET_MNEMONIC" "faucet"
            add-genesis-account "$RELAYER_MNEMONIC" "relayer"
            add-genesis-account "${cosmosTools.cvm.mnemonic}" "cvm"
            add-genesis-account "${cosmosTools.pools.mnemonic}" "pools"

            neutrond gentx $VALIDATOR_MONIKER 500000000uosmo --keyring-backend=test --chain-id=$CHAIN_ID --home "$CHAIN_DATA" 
            neutrond collect-gentxs --home "$CHAIN_DATA"
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

            neutrond start --home "$CHAIN_DATA" --rpc.unsafe --rpc.laddr tcp://0.0.0.0:$PORT --pruning=nothing --grpc.address localhost:19090 --address "tcp://0.0.0.0:36658" --p2p.external-address 43421 --p2p.laddr "tcp://0.0.0.0:36656" --p2p.pex false --p2p.upnp  false  --p2p.seed_mode true --log_level trace --trace
          '';
        };

        neutrond-gen-fresh = pkgs.writeShellApplication {
          name = "neutrond-gen-fresh";
          runtimeInputs = [ neutrond-gen ];
          text = ''
            CHAIN_DATA="${devnet-root-directory}/.neutrond"
            rm --force --recursive "$CHAIN_DATA"
            neutrond-gen
          '';
        };       
      };
    };
}
