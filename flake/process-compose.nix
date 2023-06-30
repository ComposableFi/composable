{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, ... }:
    let devnet-root-directory = "/tmp/composable-dev/";
    in {
      packages = rec {
        osmosisd = pkgs.writeShellApplication {
          name = "osmosisd";
          text = ''
            ${self.inputs.cosmos.packages.${system}.osmosis}/bin/osmosisd "$@"
          '';
        };
        osmosisd-gen = pkgs.writeShellApplication {
          name = "osmosisd-gen";
          runtimeInputs = [ osmosisd pkgs.jq pkgs.yq pkgs.dasel ];
          text = ''
            OSMOSIS_DATA="${devnet-root-directory}osmosis-dev"
            rm --force --recursive "$OSMOSIS_DATA" 
            CHAIN_ID="osmosis"
            VALIDATOR_MONIKER="validator"
            VALIDATOR_MNEMONIC="bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort"
            FAUCET_MNEMONIC="increase bread alpha rigid glide amused approve oblige print asset idea enact lawn proof unfold jeans rabbit audit return chuckle valve rather cactus great"
            RELAYER_MNEMONIC="black frequent sponsor nice claim rally hunt suit parent size stumble expire forest avocado mistake agree trend witness lounge shiver image smoke stool chicken"
            CONFIG_FOLDER=$OSMOSIS_DATA/config
            KEYRING_TEST="$OSMOSIS_DATA/keyring-test"
            GENESIS=$CONFIG_FOLDER/genesis.json
            mkdir --parents $OSMOSIS_DATA

            echo "$VALIDATOR_MNEMONIC" | osmosisd init --chain-id="$CHAIN_ID" --home "$OSMOSIS_DATA" --recover "$VALIDATOR_MONIKER"

            function dasel-genesis() {
              dasel put --type string --file "$GENESIS" --value "$2" "$1"   
            }             

            dasel-genesis '.app_state.staking.params.bond_denom' 'uosmo'
            dasel-genesis '.app_state.staking.params.unbonding_time' '60s'
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
            dasel-genesis '.app_state.epochs.epochs.[1].duration' "30s"
            dasel  put --type json --file "$GENESIS" --value "[{},{},{}]" '.app_state.poolincentives.lockable_durations'
            dasel-genesis '.app_state.poolincentives.lockable_durations.[0]' "60s"
            dasel-genesis '.app_state.poolincentives.lockable_durations.[1]' "90s"
            dasel-genesis '.app_state.poolincentives.lockable_durations.[2]' "120s"
            dasel-genesis '.app_state.poolincentives.params.minted_denom' "uosmo"
            dasel  put --type json --file "$GENESIS" --value "[{},{},{},{}]" '.app_state.incentives.lockable_durations'
            dasel-genesis '.app_state.incentives.lockable_durations.[0]' "1s"
            dasel-genesis '.app_state.incentives.lockable_durations.[1]' "60s"
            dasel-genesis '.app_state.incentives.lockable_durations.[2]' "90s"
            dasel-genesis '.app_state.incentives.lockable_durations.[3]' "120s"
            dasel-genesis '.app_state.incentives.params.distr_epoch_identifier' "hour"
            dasel-genesis '.app_state.mint.params.mint_denom' "uosmo"
            dasel-genesis '.app_state.mint.params.epoch_identifier' "day"
            dasel-genesis '.app_state.poolmanager.params.pool_creation_fee.[0].denom' "uosmo"
            
            dasel  put --type json --file "$GENESIS" --value "[{}]" '.app_state.gamm.params.pool_creation_fee'
            dasel-genesis '.app_state.gamm.params.pool_creation_fee.[0].denom' "uosmo"
            dasel-genesis '.app_state.gamm.params.pool_creation_fee.[0].amount' "10000000"
            dasel-genesis '.app_state.txfees.basedenom' "uosmo"
            dasel-genesis '.app_state.wasm.params.code_upload_access.permission' "Everybody"
            dasel-genesis '.app_state.concentratedliquidity.params.is_permissionless_pool_creation_enabled' true
            
            function add-genesis-account() {
              echo "$1" | osmosisd keys add "$2" --recover --keyring-backend=test --home "$OSMOSIS_DATA" --keyring-dir "$KEYRING_TEST"
              ACCOUNT=$(osmosisd keys show --address "$2" --keyring-backend test --home "$OSMOSIS_DATA" --keyring-dir "$KEYRING_TEST")
              osmosisd add-genesis-account "$ACCOUNT" 100000000000uosmo,100000000000uion,100000000000stake --home "$OSMOSIS_DATA"
            }

            add-genesis-account "$VALIDATOR_MNEMONIC" "$VALIDATOR_MONIKER"
            add-genesis-account "$FAUCET_MNEMONIC" "faucet"
            add-genesis-account "$RELAYER_MNEMONIC" "relayer"

            osmosisd gentx $VALIDATOR_MONIKER 500000000uosmo --keyring-backend=test --chain-id=$CHAIN_ID --home "$OSMOSIS_DATA" --keyring-dir "$KEYRING_TEST"
            osmosisd collect-gentxs --home "$OSMOSIS_DATA"
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "" '.p2p.seeds'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:46657" '.rpc.laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "0.0.0.0:16060" '.rpc.pprof_laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://127.0.0.1:36658" '.proxy_app'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value ":36660" '.instrumentation.prometheus_listen_addr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:36656" '.p2p.prometheus_listen_addr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://localhost:36657" '.node'
            
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "0.0.0.0:19090" '.grpc.address'
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "0.0.0.0:19091" '.grpc-web.address'
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "tcp://0.0.0.0:11317" '.api.address'

            osmosisd start --home "$OSMOSIS_DATA"
          '';
        };
      };
      process-compose.devnet-xc = {
        settings = {
          processes = {
            centauri = {
              command = self'.packages.centaurid-gen;
              readiness_probe.http_get = {
                host = "127.0.0.1";
                port = 26657;
              };
            };
            centauri-init = {
              command = self'.packages.centaurid-init;
              depends_on."centauri".condition = "process_healthy";
            };
            picasso = {
              command = self'.packages.zombienet-rococo-local-picasso-dev;
              availability = { restart = "on_failure"; };
              log_location = "/tmp/composable-devnet/zombienet.log";
            };

            hyperspace-client = {
              command = ''
                sleep 20
                COMPOSABLE_DATA=/tmp/composable-devnet/
                HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
                RUST_LOG="hyperspace=trace,hyperspace_parachain=trace,hyperspace_cosmos=trace"
                export RUST_LOG
                mkdir --parents "$COMPOSABLE_DATA"
                mkdir --parents "$HYPERSPACE_DATA"

                cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-chain-2} $HYPERSPACE_DATA/config-chain-2.toml  
                cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-chain-3} $HYPERSPACE_DATA/config-chain-3.toml  
                cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-core} $HYPERSPACE_DATA/config-core.toml                
                CODE_ID=$(cat /tmp/centauri-dev/code_id)
                sed -i "s/wasm_code_id = \"0000000000000000000000000000000000000000000000000000000000000000\"/wasm_code_id = \"$CODE_ID\"/" "$HYPERSPACE_DATA/config-chain-2.toml"
                ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-clients --config-a $HYPERSPACE_DATA/config-chain-3.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10
              '';
              log_location = "/tmp/composable-devnet/hyperspace/clients.log";
              depends_on = {
                "centauri-init".condition = "process_completed_successfully";
                "centauri".condition = "process_healthy";
              };
              availability = { restart = "on_failure"; };
            };
            hyperspace-connection = {
              command = ''
                COMPOSABLE_DATA=/tmp/composable-devnet/
                HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
                RUST_LOG="hyperspace=trace,hyperspace_parachain=trace,hyperspace_cosmos=trace"
                export RUST_LOG      
                ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-connection --config-a $HYPERSPACE_DATA/config-chain-3.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10
              '';
              log_location = "/tmp/composable-devnet/hyperspace/connection.log";
              depends_on = {
                "hyperspace-client".condition =
                  "process_completed_successfully";
              };
              availability = { restart = "on_failure"; };
            };
            hyperspace-channels = {
              command = ''
                COMPOSABLE_DATA=/tmp/composable-devnet/
                HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
                RUST_LOG="hyperspace=trace,hyperspace_parachain=trace,hyperspace_cosmos=trace"
                export RUST_LOG
                ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-channel --config-a $HYPERSPACE_DATA/config-chain-3.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10 --port-id transfer --version ics20-1 --order unordered
              '';
              log_location = "/tmp/composable-devnet/hyperspace/channels.log";
              depends_on = {
                "hyperspace-connection".condition =
                  "process_completed_successfully";
              };
              availability = { restart = "on_failure"; };
            };
            hyperspace-relay = {
              command = ''
                COMPOSABLE_DATA=/tmp/composable-devnet/
                HYPERSPACE_DATA="$COMPOSABLE_DATA/hyperspace"
                RUST_LOG="hyperspace=trace,hyperspace_parachain=trace,hyperspace_cosmos=trace"
                export RUST_LOG
                ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace relay --config-a $HYPERSPACE_DATA/config-chain-3.toml --config-b $HYPERSPACE_DATA/config-chain-2.toml --config-core $HYPERSPACE_DATA/config-core.toml --delay-period 10
              '';
              log_location = "/tmp/composable-devnet/hyperspace/relay.log";
              depends_on = {
                "hyperspace-channels".condition =
                  "process_completed_successfully";
              };
              availability = { restart = "on_failure"; };
            };
          };
        };
      };
    };
}
