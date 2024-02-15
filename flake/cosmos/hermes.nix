{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , bashTools, ... }:
    let
      networks = pkgs.networksLib;
      cosmos = self.inputs.cosmos.lib {
        inherit pkgs;
        cosmwasm-check = self.input.cosmos.packages.${system}.cosmwasm-check;
      };
      devnet-root-directory = "/tmp/composable-devnet";
      log = "trace";
      centauri-osmosis = {
        devnet = {
          TELEMETRY_PORT = 30142;
          REST_PORT = 31066;
        };
      };
      hermes-config = (cosmos.evalHermesModule {
        modules = [{

          config.hermes.global.log_level = "trace";
          config.hermes.mode.clients = {
            misbehaviour = true;
            refresh = true;
          };
          config.hermes.mode.packets = {
            enabled = true;
            clear_interval = 100;
            clear_on_start = false;
            tx_confirmation = true;
          };
          config.hermes.rest = {
            enabled = false;
            host = "0.0.0.0";
            port = centauri-osmosis.devnet.REST_PORT;
          };
          config.hermes.telemetry = {
            enabled = false;
            host = "0.0.0.0";
            port = centauri-osmosis.devnet.TELEMETRY_PORT;
          };
          config.hermes.chains = [
            {
              id = networks.pica.devnet.CHAIN_ID;
              rpc_addr = "http://127.0.0.1:${
                  builtins.toString networks.pica.devnet.CONSENSUS_RPC_PORT
                }";
              grpc_addr = "http://127.0.0.1:${
                  builtins.toString networks.pica.devnet.GRPCPORT
                }";
              event_source = {
                mode = "pull";
                interval = "1s";
              };
              rpc_timeout = "30s";
              account_prefix = "centauri";
              key_name = networks.pica.devnet.CHAIN_ID;
              store_prefix = "ibc";
              default_gas = 100000000;
              max_gas = 40000000000;
              gas_price = {
                price = 0.1;
                denom = "ppica";
              };
              gas_multiplier = 2.0;
              max_msg_num = 5;
              max_tx_size = 4097152;
              clock_drift = "10s";
              max_block_time = "30s";
              trusting_period = "640s";
              trust_threshold = {
                numerator = "1";
                denominator = "3";
              };
              type = "CosmosSdk";
              address_type = { derivation = "cosmos"; };
              trusted_node = true;
              key_store_type = "Test";
              # default is allow * *
              # packet_filter = {
              #   policy = "allow";
              #   list = [[
              #     "transfer"
              #     "channel-*"
              #   ]];
              # };
            }
            {
              id = networks.osmosis.devnet.CHAIN_ID;
              rpc_addr = "http://127.0.0.1:${
                  builtins.toString networks.osmosis.devnet.CONSENSUS_RPC_PORT
                }";
              grpc_addr = "http://127.0.0.1:${
                  builtins.toString networks.osmosis.devnet.GRPCPORT
                }";
              event_source = {
                mode = "pull";
                interval = "1s";
              };
              rpc_timeout = "20s";
              account_prefix = networks.osmosis.devnet.ACCOUNT_PREFIX;
              key_name = "osmosis-dev";
              store_prefix = "ibc";
              key_store_type = "Test";
              default_gas = 10000000;
              max_gas = 2000000000;
              gas_price = {
                price = 0.1;
                denom = "uosmo";
              };
              gas_multiplier = 2.0;
              max_msg_num = 10;
              max_tx_size = 4097152;
              clock_drift = "20s";
              max_block_time = "30s";
              trusting_period = "640s";
              trust_threshold = {
                numerator = "1";
                denominator = "3";
              };
              type = "CosmosSdk";
              address_type = { derivation = "cosmos"; };
              trusted_node = true;
            }
            # {
            #   id = networks.neutron.devnet.CHAIN_ID;
            #   rpc_addr = "http://127.0.0.1:${
            #       builtins.toString networks.neutron.devnet.CONSENSUS_RPC_PORT
            #     }";
            #   grpc_addr = "http://127.0.0.1:${
            #       builtins.toString networks.neutron.devnet.GRPCPORT
            #     }";
            #   event_source = {
            #     mode = "pull";
            #     interval = "1s";
            #   };
            #   ccv_consumer_chain = true;
            #   rpc_timeout = "20s";
            #   account_prefix = networks.neutron.devnet.ACCOUNT_PREFIX;
            #   key_name = networks.neutron.devnet.CHAIN_ID;
            #   store_prefix = "ibc";
            #   key_store_type = "Test";
            #   default_gas = 10000000;
            #   max_gas = 30000000;
            #   gas_price = {
            #     price = 2.5e-3;
            #     denom = networks.neutron.devnet.FEE;
            #   };
            #   gas_multiplier = 2.0;
            #   max_msg_num = 30;
            #   max_tx_size = 4097152;
            #   clock_drift = "10s";
            #   max_block_time = "30s";
            #   trusting_period = "14days";
            #   trust_threshold = {
            #     numerator = "1";
            #     denominator = "3";
            #   };
            #   type = "CosmosSdk";
            #   address_type = { derivation = "cosmos"; };
            #   trusted_node = true;
            # }
            {
              id = networks.cosmos-hub.devnet.CHAIN_ID;
              rpc_addr = "http://127.0.0.1:${
                  builtins.toString
                  networks.cosmos-hub.devnet.CONSENSUS_RPC_PORT
                }";
              grpc_addr = "http://127.0.0.1:${
                  builtins.toString networks.cosmos-hub.devnet.GRPCPORT
                }";
              event_source = {
                mode = "pull";
                interval = "1s";
              };
              rpc_timeout = "10s";
              account_prefix = "cosmos";
              key_name = networks.cosmos-hub.devnet.CHAIN_ID;
              store_prefix = "ibc";
              default_gas = 100000;
              max_gas = 3000000;
              gas_price = {
                price = 2.5e-3;
                denom = "uatom";
              };
              gas_multiplier = 2.0;
              max_msg_num = 30;
              max_tx_size = 2097152;
              clock_drift = "5s";
              max_block_time = "10s";
              trusting_period = "14days";
              trust_threshold = {
                numerator = "1";
                denominator = "3";
              };
              address_type = { derivation = "cosmos"; };
            }
          ];
        }];
      }).config.hermes.toml;

    in {
      packages = rec {
        hermes = self.inputs.cosmos.packages.${system}.hermes;
        osmosis-centauri-hermes-init = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "osmosis-centauri-hermes-init";
          text = ''
            ${bashTools.export pkgs.networksLib.devnet.mnemonics}
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/osmosis-centauri"            
            HOME=${devnet-root-directory}/osmosis-centauri
            export HOME
            MNEMONIC_FILE="$HOME/.hermes/mnemonics/relayer.txt"
            export MNEMONIC_FILE
            echo "$HOME/.hermes/mnemonics/"
            mkdir --parents "$HOME/.hermes/mnemonics/"
            cp --dereference --no-preserve=mode,ownership --force ${
              builtins.toFile "hermes-config.toml" hermes-config
            } "$HOME/.hermes/config.toml"
            echo "$RLY_MNEMONIC_3" > "$MNEMONIC_FILE"
            hermes keys add --chain ${networks.pica.devnet.CHAIN_ID} --mnemonic-file "$MNEMONIC_FILE" --key-name ${networks.pica.devnet.CHAIN_ID} --overwrite
            hermes keys add --chain osmosis-dev --mnemonic-file "$MNEMONIC_FILE" --key-name osmosis-dev --overwrite
            export RUST_LOG
            hermes create channel --a-chain ${networks.pica.devnet.CHAIN_ID} --b-chain osmosis-dev --a-port transfer --b-port transfer --new-client-connection --yes
          '';
        };

        neutron-centauri-hermes-init = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "neutron-centauri-hermes-init";
          text = ''
            ${bashTools.export pkgs.networksLib.devnet.mnemonics}
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/neutron-centauri"            
            HOME=${devnet-root-directory}/neutron-centauri
            export HOME
            MNEMONIC_FILE="$HOME/.hermes/mnemonics/relayer.txt"
            export MNEMONIC_FILE
            echo "$HOME/.hermes/mnemonics/"
            mkdir --parents "$HOME/.hermes/mnemonics/"
            cp --dereference --no-preserve=mode,ownership --force ${
              builtins.toFile "hermes-config.toml" hermes-config
            } "$HOME/.hermes/config.toml"
            echo "$RLY_MNEMONIC_4" > "$MNEMONIC_FILE"
            hermes keys add --chain ${networks.pica.devnet.CHAIN_ID} --mnemonic-file "$MNEMONIC_FILE" --key-name ${networks.pica.devnet.CHAIN_ID} --overwrite
            hermes keys add --chain ${networks.neutron.devnet.CHAIN_ID} --mnemonic-file "$MNEMONIC_FILE" --key-name ${networks.neutron.devnet.CHAIN_ID} --overwrite
            export RUST_LOG
            hermes create channel --a-chain ${networks.pica.devnet.CHAIN_ID} --b-chain ${networks.neutron.devnet.CHAIN_ID} --a-port transfer --b-port transfer --new-client-connection --yes
          '';
        };

        neutron-cosmos-hub-hermes-init = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "neutron-cosmos-hub-hermes-init";
          text = ''
            ${bashTools.export pkgs.networksLib.devnet.mnemonics}
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/neutron-cosmos-hub"            
            HOME=${devnet-root-directory}/neutron-cosmos-hub
            export HOME
            MNEMONIC_FILE="$HOME/.hermes/mnemonics/relayer.txt"
            export MNEMONIC_FILE
            echo "$HOME/.hermes/mnemonics/"
            mkdir --parents "$HOME/.hermes/mnemonics/"
            cp --dereference --no-preserve=mode,ownership --force ${
              builtins.toFile "hermes-config.toml" hermes-config
            } "$HOME/.hermes/config.toml"
            echo "$RLY_MNEMONIC_1" > "$MNEMONIC_FILE"
            hermes keys add --chain ${networks.cosmos-hub.devnet.CHAIN_ID} --mnemonic-file "$MNEMONIC_FILE" --key-name ${networks.cosmos-hub.devnet.CHAIN_ID} --overwrite
            hermes keys add --chain ${networks.neutron.devnet.CHAIN_ID} --mnemonic-file "$MNEMONIC_FILE" --key-name ${networks.neutron.devnet.CHAIN_ID} --overwrite
            export RUST_LOG
            hermes create channel --a-chain ${networks.cosmos-hub.devnet.CHAIN_ID} --b-chain ${networks.neutron.devnet.CHAIN_ID} --a-port transfer --b-port transfer --new-client-connection --yes
          '';
        };

        centauri-cosmos-hub-hermes-init = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "centauri-cosmos-hub-hermes-init";
          text = ''
            ${bashTools.export pkgs.networksLib.devnet.mnemonics}
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/centauri-cosmos-hub"            
            HOME=${devnet-root-directory}/centauri-cosmos-hub
            export HOME
            MNEMONIC_FILE="$HOME/.hermes/mnemonics/relayer.txt"
            export MNEMONIC_FILE
            echo "$HOME/.hermes/mnemonics/"
            mkdir --parents "$HOME/.hermes/mnemonics/"
            cp --dereference --no-preserve=mode,ownership --force ${
              builtins.toFile "hermes-config.toml" hermes-config
            } "$HOME/.hermes/config.toml"
            echo "$RLY_MNEMONIC_2" > "$MNEMONIC_FILE"
            hermes keys add --chain ${networks.cosmos-hub.devnet.CHAIN_ID} --mnemonic-file "$MNEMONIC_FILE" --key-name ${networks.cosmos-hub.devnet.CHAIN_ID} --overwrite
            hermes keys add --chain ${networks.pica.devnet.CHAIN_ID} --mnemonic-file "$MNEMONIC_FILE" --key-name ${networks.pica.devnet.CHAIN_ID} --overwrite
            export RUST_LOG
            hermes create channel --a-chain ${networks.cosmos-hub.devnet.CHAIN_ID} --b-chain ${networks.pica.devnet.CHAIN_ID} --a-port transfer --b-port transfer --new-client-connection --yes
          '';
        };

        osmosis-cosmos-hub-hermes-init = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "osmosis-cosmos-hub-hermes-init";
          text = ''
            ${bashTools.export pkgs.networksLib.devnet.mnemonics}
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/osmosis-cosmos-hub"            
            HOME=${devnet-root-directory}/osmosis-cosmos-hub
            export HOME
            MNEMONIC_FILE="$HOME/.hermes/mnemonics/relayer.txt"
            export MNEMONIC_FILE
            echo "$HOME/.hermes/mnemonics/"
            mkdir --parents "$HOME/.hermes/mnemonics/"
            cp --dereference --no-preserve=mode,ownership --force ${
              builtins.toFile "hermes-config.toml" hermes-config
            } "$HOME/.hermes/config.toml"
            echo "$RLY_MNEMONIC_4" > "$MNEMONIC_FILE"
            hermes keys add --chain ${networks.cosmos-hub.devnet.CHAIN_ID} --mnemonic-file "$MNEMONIC_FILE" --key-name ${networks.cosmos-hub.devnet.CHAIN_ID} --overwrite
            hermes keys add --chain ${networks.osmosis.devnet.CHAIN_ID} --mnemonic-file "$MNEMONIC_FILE" --key-name ${networks.osmosis.devnet.CHAIN_ID} --overwrite
            export RUST_LOG
            hermes create channel --a-chain ${networks.cosmos-hub.devnet.CHAIN_ID} --b-chain ${networks.osmosis.devnet.CHAIN_ID} --a-port transfer --b-port transfer --new-client-connection --yes
          '';
        };

        osmosis-centauri-hermes-relay = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "osmosis-centauri-hermes-relay";
          text = ''
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/osmosis-centauri"            
            HOME=${devnet-root-directory}/osmosis-centauri
            export HOME
            export RUST_LOG
            hermes start
          '';
        };

        centauri-neutron-hermes-relay = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "centauri-neutron-hermes-relay";
          text = ''
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/centauri-neutron"            
            HOME=${devnet-root-directory}/centauri-neutron
            export HOME
            export RUST_LOG
            hermes start
          '';
        };

        centauri-cosmos-hub-hermes-relay = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "centauri-cosmos-hub-hermes-relay";
          text = ''
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/centauri-cosmos-hub"            
            HOME=${devnet-root-directory}/centauri-cosmos-hub
            export HOME
            export RUST_LOG
            hermes start
          '';
        };

        osmosis-cosmos-hub-hermes-relay = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "osmosis-cosmos-hub-hermes-relay";
          text = ''
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/osmosis-cosmos-hub"            
            HOME=${devnet-root-directory}/osmosis-cosmos-hub
            export HOME
            export RUST_LOG
            hermes start
          '';
        };

        neutron-cosmos-hub-hermes-relay = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ hermes ];
          name = "neutron-cosmos-hub-hermes-relay";
          text = ''
            RUST_LOG=${log}
            mkdir --parents "${devnet-root-directory}/neutron-cosmos-hub"            
            HOME=${devnet-root-directory}/neutron-cosmos-hub
            export HOME
            export RUST_LOG
            hermes start
          '';
        };
      };
    };
}
