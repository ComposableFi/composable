{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, ... }:
    let
      devnet-root-directory = "/tmp/composable-devnet";
      validator-key = "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj";
    in {
      packages = rec {
        default = pkgs.writeShellApplication {
          name = "devnet-xc-fresh-background";
          text = ''
            rm --force --recursive /tmp/composable-devnet             
            mkdir --parents /tmp/composable-devnet
            nix run .#devnet-xc-background --accept-flake-config
          '';
        };

        osmosisd = pkgs.writeShellApplication {
          name = "osmosisd";
          text = ''
            ${self.inputs.cosmos.packages.${system}.osmosis}/bin/osmosisd "$@"
          '';
        };

        composable-ready = pkgs.writeShellApplication {
          name = "composable-ready";
          runtimeInputs = [ pkgs.curl pkgs.dasel ];
          text = ''
            curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://127.0.0.1:32201      
          '';
        };

        devnet-xc-run-fresh = pkgs.writeShellApplication {
          name = "devnet-xc-run-fresh";
          text = ''
            rm --force --recursive /tmp/composable-devnet             
            mkdir --parents /tmp/composable-devnet
            nix run .#devnet-xc --accept-flake-config
          '';
        };

        picasso-centauri-ibc-init = pkgs.writeShellApplication {
          name = "picasso-centauri-ibc-init";
          text = ''
            mkdir --parents "/tmp/composable-devnet/picasso-centauri-ibc"
            HOME="/tmp/composable-devnet/picasso-centauri-ibc"
            export HOME
            RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
            export RUST_LOG
            cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-chain-2} "/tmp/composable-devnet/picasso-centauri-ibc/config-chain-2.toml"  
            cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-chain-3} "/tmp/composable-devnet/picasso-centauri-ibc/config-chain-3.toml"  
            cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-core} "/tmp/composable-devnet/picasso-centauri-ibc/config-core.toml"                
            CODE_ID=$(cat ${devnet-root-directory}/.centaurid/code_id)
            echo "$CODE_ID"
            sed -i "s/wasm_code_id = \"0000000000000000000000000000000000000000000000000000000000000000\"/wasm_code_id = \"$CODE_ID\"/" "/tmp/composable-devnet/picasso-centauri-ibc/config-chain-2.toml"
            ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-clients --config-a "/tmp/composable-devnet/picasso-centauri-ibc/config-chain-3.toml" --config-b /tmp/composable-devnet/picasso-centauri-ibc/config-chain-2.toml --config-core /tmp/composable-devnet/picasso-centauri-ibc/config-core.toml --delay-period 10
          '';
        };

        composable-picasso-ibc-init = pkgs.writeShellApplication {
          name = "composable-picasso-ibc-init";
          text = ''
            sleep 60
            mkdir --parents "/tmp/composable-devnet/composable-picasso-ibc"
            HOME="/tmp/composable-devnet/composable-picasso-ibc"
            export HOME
            RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug,jsonrpsee_client_transport::ws=info,soketto=info,tracing::span=info,mio::poll=info,trie=info,jsonrpsee_core::client::async_client=info"
            export RUST_LOG
            cp --dereference --no-preserve=mode,ownership --force ${self'.packages.ibc-composable-picasso-config-2} "/tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml"  
            cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-chain-b} "/tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml"  
            cp --dereference --no-preserve=mode,ownership --force ${self'.packages.hyperspace-config-core} "/tmp/composable-devnet/composable-picasso-ibc/config-core.toml"                
            ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-clients --config-a "/tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml" --config-b /tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml --config-core /tmp/composable-devnet/composable-picasso-ibc/config-core.toml --delay-period 10
          '';
        };

        osmosisd-gen = pkgs.writeShellApplication {
          name = "osmosisd-gen";
          runtimeInputs = [ osmosisd pkgs.jq pkgs.yq pkgs.dasel ];
          text = ''
            HOME=${devnet-root-directory}
            export HOME
            OSMOSIS_DATA="$HOME/.osmosisd"             
            CHAIN_ID="osmosis-dev"
            REUSE=true
            export REUSE
            if [[ $REUSE == false ]]; then
              rm --force --recursive "$OSMOSIS_DATA" 
            fi

            VALIDATOR_MONIKER="validator"
            VALIDATOR_MNEMONIC="bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort"
            FAUCET_MNEMONIC="increase bread alpha rigid glide amused approve oblige print asset idea enact lawn proof unfold jeans rabbit audit return chuckle valve rather cactus great"
            RELAYER_MNEMONIC="black frequent sponsor nice claim rally hunt suit parent size stumble expire forest avocado mistake agree trend witness lounge shiver image smoke stool chicken"
            CONFIG_FOLDER=$OSMOSIS_DATA/config
            GENESIS=$CONFIG_FOLDER/genesis.json
            mkdir --parents "$OSMOSIS_DATA/data/cs.wal"

            echo "$VALIDATOR_MNEMONIC" | osmosisd init --chain-id="$CHAIN_ID" --home "$OSMOSIS_DATA" --recover "$VALIDATOR_MONIKER"

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
            dasel-genesis '.app_state.concentratedliquidity.params.is_permissionless_pool_creation_enabled' true

            function add-genesis-account() {
              echo "$1" | osmosisd keys add "$2" --recover --keyring-backend test --home "$OSMOSIS_DATA" 
              ACCOUNT=$(osmosisd keys show --address "$2" --keyring-backend test --home "$OSMOSIS_DATA" )
              echo "===================================="
              echo "$ACCOUNT"
              osmosisd add-genesis-account "$ACCOUNT" 100000000000uosmo,100000000000uion,100000000000stake --home "$OSMOSIS_DATA"
            }

            add-genesis-account "$VALIDATOR_MNEMONIC" "$VALIDATOR_MONIKER"
            add-genesis-account "$FAUCET_MNEMONIC" "faucet"
            add-genesis-account "$RELAYER_MNEMONIC" "relayer"

            osmosisd gentx $VALIDATOR_MONIKER 500000000uosmo --keyring-backend=test --chain-id=$CHAIN_ID --home "$OSMOSIS_DATA" 
            osmosisd collect-gentxs --home "$OSMOSIS_DATA"
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "" '.p2p.seeds'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:36657" '.rpc.laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "0.0.0.0:16060" '.rpc.pprof_laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://127.0.0.1:36658" '.proxy_app'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value ":36660" '.instrumentation.prometheus_listen_addr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:36656" '.p2p.laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://localhost:36657" '.node'

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

            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "tcp://localhost:36657" '.node'
            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "$CHAIN_ID" '.chain-id'
            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "test" '.keyring-backend'
            dasel put --type string --file $CONFIG_FOLDER/client.toml --value "json" '.output'


            osmosisd start --home "$OSMOSIS_DATA" --rpc.unsafe --rpc.laddr tcp://0.0.0.0:36657 --pruning=nothing --grpc.address localhost:19090   --address "tcp://0.0.0.0:36658" --p2p.external-address 43421 --p2p.laddr "tcp://0.0.0.0:36656" --p2p.pex false --p2p.upnp  false  --p2p.seed_mode true
          '';
        };

        osmosisd-init = pkgs.writeShellApplication {
          name = "osmosisd-init";
          runtimeInputs = [ osmosisd pkgs.jq pkgs.yq pkgs.dasel ];
          text = ''
            HOME=${devnet-root-directory}
            export HOME
            OSMOSIS_DATA="$HOME/.osmosisd"             
            CHAIN_ID="osmosis-dev"

            set +e
            osmosisd tx wasm store  "${self'.packages.xcvm-contracts}/lib/cw_xc_gateway.wasm" --chain-id="$CHAIN_ID"  --node "tcp://localhost:36657" --output json --yes --gas 25000000 --fees 920000166uatom --dry-run --log_level info --keyring-backend test  --home "$OSMOSIS_DATA" --from validator
          '';
        };
      };
      process-compose = rec {
        devnet-xc-background = devnet-xc // { tui = false; };
        devnet-xc = {
          debug = true;
          settings = {
            processes = {
              centauri = {
                command = self'.packages.centaurid-gen;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = 26657;
                };
                log_location = "/tmp/composable-devnet/centauri.log";
              };
              centauri-init = {
                command = self'.packages.centaurid-init;
                depends_on."centauri".condition = "process_healthy";
                log_location = "/tmp/composable-devnet/centauri-init.log";
              };
              osmosis = {
                command = self'.packages.osmosisd-gen;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = 36657;
                };
                log_location = "/tmp/composable-devnet/osmosis.log";
              };
              osmosis-init = {
                command = self'.packages.osmosisd-init;
                depends_on."osmosis".condition = "process_healthy";
                log_location = "/tmp/composable-devnet/osmosis-init.log";
                availability = { restart = "on_failure"; };
              };

              picasso = {
                command = self'.packages.zombienet-rococo-local-picasso-dev;
                availability = { restart = "on_failure"; };
                log_location = "/tmp/composable-devnet/picasso.log";
              };
              composable = {
                command = self'.packages.zombienet-composable-centauri-b;
                availability = { restart = "on_failure"; };
                log_location = "/tmp/composable-devnet/composable.log";
                #readiness_probe.exec = self'.packages.composable-ready;
              };
              osmosis-centauri-hermes-init = {
                command = self'.packages.osmosis-centauri-hermes-init;
                depends_on = {
                  "centauri-init".condition = "process_completed_successfully";
                  "osmosis".condition = "process_healthy";
                };
                log_location =
                  "/tmp/composable-devnet/osmosis-centauri-hermes-init.log";
                availability = { restart = "on_failure"; };
              };

              osmosis-centauri-hermes-relay = {
                command = self'.packages.osmosis-centauri-hermes-relay;
                depends_on = {
                  "osmosis-centauri-hermes-init".condition =
                    "process_completed_successfully";
                };
                log_location =
                  "/tmp/composable-devnet/osmosis-centauri-hermes-relay.log";
                availability = { restart = "on_failure"; };
              };

              picasso-centauri-ibc-init = {
                command = self'.packages.picasso-centauri-ibc-init;
                log_location =
                  "/tmp/composable-devnet/picasso-centauri-ibc-init.log";
                depends_on = {
                  "centauri-init".condition = "process_completed_successfully";
                  "centauri".condition = "process_healthy";
                };
                availability = { restart = "on_failure"; };
              };

              composable-picasso-ibc-init = {
                command = self'.packages.composable-picasso-ibc-init;
                log_location =
                  "/tmp/composable-devnet/composable-picasso-ibc-init.log";
                depends_on = {
                  "picasso-centauri-ibc-channels-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = "on_failure"; };
              };

              picasso-centauri-ibc-connection-init = {
                command = ''
                  HOME="/tmp/composable-devnet/picasso-centauri-ibc"
                  export HOME                
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG      
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-connection --config-a /tmp/composable-devnet/picasso-centauri-ibc/config-chain-3.toml --config-b /tmp/composable-devnet/picasso-centauri-ibc/config-chain-2.toml --config-core /tmp/composable-devnet/picasso-centauri-ibc/config-core.toml --delay-period 10
                '';
                log_location =
                  "/tmp/composable-devnet/picasso-centauri-ibc-connection-init.log";
                depends_on = {
                  "picasso-centauri-ibc-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = "on_failure"; };
              };

              composable-picasso-ibc-connection-init = {
                command = ''
                  HOME="/tmp/composable-devnet/composable-picasso-ibc"
                  export HOME                
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG      
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-connection --config-a /tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml --config-b /tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml --config-core /tmp/composable-devnet/composable-picasso-ibc/config-core.toml --delay-period 10
                '';
                log_location =
                  "/tmp/composable-devnet/composable-picasso-ibc-connection-init.log";
                depends_on = {
                  "composable-picasso-ibc-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = "on_failure"; };
              };

              picasso-centauri-ibc-channels-init = {
                command = ''
                  HOME="/tmp/composable-devnet/picasso-centauri-ibc"
                  export HOME       
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-channel --config-a /tmp/composable-devnet/picasso-centauri-ibc/config-chain-3.toml --config-b /tmp/composable-devnet/picasso-centauri-ibc/config-chain-2.toml --config-core /tmp/composable-devnet/picasso-centauri-ibc/config-core.toml --delay-period 10 --port-id transfer --version ics20-1 --order unordered
                '';
                log_location =
                  "/tmp/composable-devnet/picasso-centauri-ibc-channels-init.log";
                depends_on = {
                  "picasso-centauri-ibc-connection-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = "on_failure"; };
              };
              composable-picasso-ibc-channels-init = {
                command = ''
                  HOME="/tmp/composable-devnet/composable-picasso-ibc"
                  export HOME       
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-channel --config-a /tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml --config-b /tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml --config-core /tmp/composable-devnet/composable-picasso-ibc/config-core.toml --delay-period 10 --port-id transfer --version ics20-1 --order unordered
                '';
                log_location =
                  "/tmp/composable-devnet/composable-picasso-ibc-channels-init.log";
                depends_on = {
                  "composable-picasso-ibc-connection-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = "on_failure"; };
              };
              picasso-centauri-ibc-relay = {
                command = ''
                  HOME="/tmp/composable-devnet/picasso-centauri-ibc"
                  export HOME
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace relay --config-a /tmp/composable-devnet/picasso-centauri-ibc/config-chain-3.toml --config-b /tmp/composable-devnet/picasso-centauri-ibc/config-chain-2.toml --config-core /tmp/composable-devnet/picasso-centauri-ibc/config-core.toml --delay-period 10
                '';
                log_location =
                  "/tmp/composable-devnet/picasso-centauri-ibc-relay.log";
                depends_on = {
                  "picasso-centauri-ibc-channels-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = "on_failure"; };
              };
              composable-picasso-ibc-relay = {
                command = ''
                  HOME="/tmp/composable-devnet/composable-picasso-ibc"
                  export HOME
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG
                  sed -i "s/private_key = \"\/\/Alice\"/private_key = \"\/\/Bob\"/" "/tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml"
                  sed -i "s/private_key = \"\/\/Alice\"/private_key = \"\/\/Bob\"/" "/tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml"
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace relay --config-a /tmp/composable-devnet/composable-picasso-ibc/config-chain-a.toml --config-b /tmp/composable-devnet/composable-picasso-ibc/config-chain-b.toml --config-core /tmp/composable-devnet/composable-picasso-ibc/config-core.toml --delay-period 10
                '';
                log_location =
                  "/tmp/composable-devnet/composable-picasso-ibc-relay.log";
                depends_on = {
                  "composable-picasso-ibc-channels-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = "on_failure"; };
              };
            };
          };
        };
      };
    };
}
