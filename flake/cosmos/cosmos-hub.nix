{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, bashTools, ... }:
    let devnet = pkgs.networksLib.cosmos-hub.devnet;
    in {

      packages = rec {
        gaiad = pkgs.writeShellApplication {
          name = "gaiad";
          runtimeInputs = devnetTools.withBaseContainerTools;
          text = ''
            ${self.inputs.cosmos.packages.${system}.gaia14}/bin/gaiad "$@"
          '';
        };

        cosmos-hub-start = pkgs.writeShellApplication {
          name = "cosmos-hub-start";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ gaiad pkgs.jq ];
          text = ''
            ${bashTools.export devnet}
              $BINARY start --log_level debug --log_format json --home "$CHAIN_DIR"  --pruning=nothing --trace  --p2p.pex false --p2p.upnp false --p2p.seed_mode true --log_level trace 2>&1 | tee "$CHAIN_DIR/$CHAIN_ID.log"
          '';
        };

        cosmos-hub-gen = pkgs.writeShellApplication {
          name = "cosmos-hub-gen";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ gaiad pkgs.jq ];
          text = ''
            ${bashTools.export devnet}
            ${bashTools.export pkgs.networksLib.devnet.mnemonics}
            if test "''${1-fresh}" == "fresh"; then
              if pgrep "^gaiad$"; then
                killall "$BINARY"
              fi
              rm -rf "$CHAIN_DATA"                
            fi
            mkdir --parents "$CHAIN_DATA"

            $BINARY init test --home "$CHAIN_DIR" --chain-id="$CHAIN_ID"

            echo "$VAL_MNEMONIC_1" | $BINARY keys add val1 --home "$CHAIN_DIR" --recover --keyring-backend=test
            echo "$VAL_MNEMONIC_2" | $BINARY keys add val2 --home "$CHAIN_DIR" --recover --keyring-backend=test
            echo "$DEMO_MNEMONIC_1" | $BINARY keys add demowallet1 --home "$CHAIN_DIR" --recover --keyring-backend=test
            echo "$DEMO_MNEMONIC_2" | $BINARY keys add demowallet2 --home "$CHAIN_DIR" --recover --keyring-backend=test
            echo "$DEMO_MNEMONIC_3" | $BINARY keys add demowallet3 --home "$CHAIN_DIR" --recover --keyring-backend=test
            echo "$RLY_MNEMONIC_1" | $BINARY keys add rly1 --home "$CHAIN_DIR" --recover --keyring-backend=test
            echo "$RLY_MNEMONIC_2" | $BINARY keys add rly2 --home "$CHAIN_DIR" --recover --keyring-backend=test
            echo "$RLY_MNEMONIC_4" | $BINARY keys add rly4 --home "$CHAIN_DIR" --recover --keyring-backend=test

            $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DIR" keys show val1 --keyring-backend test -a --home "$CHAIN_DIR")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DIR"
            $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DIR" keys show val2 --keyring-backend test -a --home "$CHAIN_DIR")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DIR"
            $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DIR" keys show demowallet1 --keyring-backend test -a --home "$CHAIN_DIR")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DIR"
            $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DIR" keys show demowallet2 --keyring-backend test -a --home "$CHAIN_DIR")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DIR"
            $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DIR" keys show demowallet3 --keyring-backend test -a --home "$CHAIN_DIR")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DIR"
            $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DIR" keys show rly1 --keyring-backend test -a --home "$CHAIN_DIR")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DIR"
            $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DIR" keys show rly2 --keyring-backend test -a --home "$CHAIN_DIR")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DIR"
            $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DIR" keys show rly4 --keyring-backend test -a --home "$CHAIN_DIR")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DIR"

            sed -i -e 's/timeout_commit = "5s"/timeout_commit = "1s"/g' "$CHAIN_DIR/config/config.toml"
            sed -i -e 's/timeout_propose = "3s"/timeout_propose = "1s"/g' "$CHAIN_DIR/config/config.toml"
            sed -i -e 's/index_all_keys = false/index_all_keys = true/g' "$CHAIN_DIR/config/config.toml"
            sed -i -e 's/enable = false/enable = true/g' "$CHAIN_DIR/config/app.toml"
            sed -i -e 's/swagger = false/swagger = true/g' "$CHAIN_DIR/config/app.toml"
            sed -i -e "s/minimum-gas-prices = \"\"/minimum-gas-prices = \"0.0025$STAKEDENOM\"/g" "$CHAIN_DIR/config/app.toml"
            sed -i -e 's/enabled = false/enabled = true/g' "$CHAIN_DIR/config/app.toml"
            sed -i -e 's/prometheus-retention-time = 0/prometheus-retention-time = 1000/g' "$CHAIN_DIR/config/app.toml"

            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "0.0.0.0:$GRPCPORT" '.grpc.address'
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "0.0.0.0:$GRPCWEB" '.grpc-web.address'
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "0.0.0.0:$ROSETTA_PORT" '.rosetta.address'
            dasel put --type string --file "$CONFIG_FOLDER/app.toml" --value "tcp://0.0.0.0:$RESTPORT" '.api.address'

            dasel put --type string --file "$CONFIG_FOLDER/client.toml" --value "tcp://localhost:$GRPCPORT" '.node'

            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:$CONSENSUS_GRPC_PORT" '.rpc.grpc_laddr'
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:$P2PPORT" '.p2p.laddr'            
            dasel put --type string --file "$CONFIG_FOLDER/config.toml" --value "tcp://0.0.0.0:$CONSENSUS_RPC_PORT" '.rpc.laddr'

            GENESIS_FILE="$CHAIN_DIR/config/genesis.json"

            sed -i -e "s/\"denom\": \"stake\",/\"denom\": \"$STAKEDENOM\",/g" "$GENESIS_FILE"
            sed -i -e "s/\"mint_denom\": \"stake\",/\"mint_denom\": \"$STAKEDENOM\",/g" "$GENESIS_FILE"
            sed -i -e "s/\"bond_denom\": \"stake\"/\"bond_denom\": \"$STAKEDENOM\"/g" "$GENESIS_FILE"
            sed -i -e 's/enabled-unsafe-cors = false/enabled-unsafe-cors = true/g' "$CHAIN_DIR/config/app.toml"
            $BINARY gentx val1 "7000000000$STAKEDENOM" --home "$CHAIN_DIR" --chain-id "$CHAIN_ID" --keyring-backend test
            $BINARY collect-gentxs --home "$CHAIN_DIR"

            sed -i -e 's/\"allow_messages\":.*/\"allow_messages\": [\"\/cosmos.bank.v1beta1.MsgSend\", \"\/cosmos.staking.v1beta1.MsgDelegate\", \"\/cosmos.staking.v1beta1.MsgUndelegate\"]/g' "$CHAIN_DIR/config/genesis.json"
          '';
        };
      };
    };
}
