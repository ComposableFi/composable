{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, cosmosLib, ... }:
    let
      devnet-root-directory = "/tmp/composable-devnet";
      validator-key = "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj";
      log = "debug";
      centauri-osmosis-ibc-hermes =
        (cosmosLib.evalHermesModule {
          modules = [
            {
              config.hermes.global.log_level = "trace";
              config.mode.clients.misbehaviour = false;
              config.mode.packets =
                {
                  enabled = true;
                  clear_interval = 0;
                  clear_on_start = false;
                  tx_confirmation = true;
                };
              config.rest = {
                enabled = true;
                host = "127.0.0.1";
                port = 30042;
              };
              config.telemetry = {
                enabled = true;
                host = "127.0.0.1";
                port = 30041;
              };
              config.hermes.chains = [
                {
                  id = 'centauri-1';
                  rpc_addr = 'http://127.0.0.1:26657';
                  grpc_addr = 'http://127.0.0.1:9090';
                  event_source = { mode = "pull"; interval = "1s"; };
                  rpc_timeout = "30s";
                  account_prefix = "centauri";
                  key_name = "centauri-1";
                  store_prefix = "ibc";
                  default_gas = 100000000;
                  max_gas = 40000000000;
                  gas_price = { price = 1; denom = "ppica"; };
                  gas_multiplier = 1.3;
                  max_msg_num = 5;
                  max_tx_size = 4097152;
                  clock_drift = "10s";
                  max_block_time = "30s";
                  trusting_period = "640s";
                  trust_threshold = { numerator = 1; denominator = 3; };
                  type = "CosmosSdk";
                  address_type = { derivation = "cosmos"; };
                  trusted_node = true;
                  key_store_type = "Test";

                }
              ];
            }
          ];
        }).config.hermes.toml;
      centauri-osmosis-ibc-hermes1 = ''
        [[chains]]
        id = 'osmosis-dev'
        rpc_addr = 'http://127.0.0.1:26757'
        grpc_addr = 'http://127.0.0.1:19090'
        #event_source = { mode = 'push', url = 'ws://127.0.0.1:36657/websocket', batch_delay = '1000ms' }
        event_source = { mode = 'pull', interval = '1s' }
        rpc_timeout = '20s'
        account_prefix = 'osmo'
        key_name = 'osmosis-dev'
        store_prefix = 'ibc'
        key_store_type = 'Test'
        default_gas = 10000000
        max_gas = 4000000000
        gas_price = { price = 1, denom = 'uosmo' }
        gas_multiplier = 1.1
        max_msg_num = 5
        max_tx_size = 4097152
        clock_drift = '10s'
        max_block_time = '30s'
        trusting_period = '640s'
        trust_threshold = { numerator = '1', denominator = '3' }
        type = 'CosmosSdk'
        address_type = { derivation = 'cosmos' }
        trusted_node = true
      '';
    in
    {
      packages = rec {
        hermes = self.inputs.cosmos.packages.${system}.hermes;
        osmosis-centauri-hermes-init = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "osmosis-centauri-hermes-init";
          text = ''
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}"            
            HOME=${devnet-root-directory}
            export HOME
            MNEMONIC_FILE="$HOME/.hermes/mnemonics/relayer.txt"
            export MNEMONIC_FILE
            echo "$HOME/.hermes/mnemonics/"
            mkdir --parents "$HOME/.hermes/mnemonics/"c
            echo "${centauri-osmosis-ibc-hermes}" > "$HOME/.hermes/config.toml"
            echo "black frequent sponsor nice claim rally hunt suit parent size stumble expire forest avocado mistake agree trend witness lounge shiver image smoke stool chicken" > "$MNEMONIC_FILE"
            hermes keys add --chain centauri-1 --mnemonic-file "$MNEMONIC_FILE" --key-name centauri-1 --overwrite
            hermes keys add --chain osmosis-dev --mnemonic-file "$MNEMONIC_FILE" --key-name osmosis-dev --overwrite
            export RUST_LOG
            hermes create channel --a-chain centauri-1 --b-chain osmosis-dev --a-port transfer --b-port transfer --new-client-connection --yes
          '';
        };


        osmosis-centauri-hermes-relay = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "osmosis-centauri-hermes-relay";
          text = ''
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}"            
            HOME=${devnet-root-directory}
            export HOME
            export RUST_LOG
            hermes start
          '';
        };
      };
    };
}
