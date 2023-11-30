{ self, ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools
    , cosmosTools, bashTools, ... }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      validator-key = cosmosTools.validators.neutron;

    in {
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
          runtimeInputs = devnetTools.withBaseContainerTools ++ [ neutrond ];
          text = ''
              ${bashTools.export pkgs.networksLib.neutron.devnet}
              ${bashTools.export pkgs.networksLib.devnet.mnemonics}

              if test "''${1-fresh}" == "fresh"; then
                if pgrep "^neutrond$"; then
                  echo "Neutrond is running, please stop it first"
                  killall "$BINARY"
                fi
                rm -rf "$CHAIN_DATA"                
              fi

              mkdir --parents "$CHAIN_DATA"

              $BINARY init test --home "$CHAIN_DATA" --chain-id="$CHAIN_ID"

              echo "Adding genesis accounts..."
              echo "$VAL_MNEMONIC_2"
              echo "$VAL_MNEMONIC_2" | $BINARY keys add val2 --home "$CHAIN_DATA" --recover --keyring-backend=test
              echo "Adding genesis accounts..1"
              echo "$VAL_MNEMONIC_1" | $BINARY keys add val1 --home "$CHAIN_DATA" --recover --keyring-backend=test
              echo "Adding genesis accounts..2"
              echo "$DEMO_MNEMONIC_1" | $BINARY keys add demowallet1 --home "$CHAIN_DATA" --recover --keyring-backend=test
              echo "Adding genesis accounts..3"
              echo "$DEMO_MNEMONIC_2" | $BINARY keys add demowallet2 --home "$CHAIN_DATA" --recover --keyring-backend=test
              echo "Adding genesis accounts..4"
              echo "$DEMO_MNEMONIC_3" | $BINARY keys add demowallet3 --home "$CHAIN_DATA" --recover --keyring-backend=test
              echo "Adding genesis accounts..5"
              echo "$RLY_MNEMONIC_1" | $BINARY keys add rly1 --home "$CHAIN_DATA" --recover --keyring-backend=test
              echo "Adding genesis accounts..6"
              echo "$RLY_MNEMONIC_2" | $BINARY keys add rly2 --home "$CHAIN_DATA" --recover --keyring-backend=test

              $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DATA" keys show val1 --keyring-backend test -a --home "$CHAIN_DATA")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DATA"
              $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DATA" keys show val2 --keyring-backend test -a --home "$CHAIN_DATA")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DATA"
              $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DATA" keys show demowallet1 --keyring-backend test -a --home "$CHAIN_DATA")" "100000000000000$STAKEDENOM,100000000000000$IBCATOMDENOM,100000000000000$IBCUSDCDENOM"  --home "$CHAIN_DATA"
              $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DATA" keys show demowallet2 --keyring-backend test -a --home "$CHAIN_DATA")" "100000000000000$STAKEDENOM,100000000000000$IBCATOMDENOM,100000000000000$IBCUSDCDENOM"  --home "$CHAIN_DATA"
              $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DATA" keys show demowallet3 --keyring-backend test -a --home "$CHAIN_DATA")" "100000000000000$STAKEDENOM,100000000000000$IBCATOMDENOM,100000000000000$IBCUSDCDENOM"  --home "$CHAIN_DATA"
              $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DATA" keys show rly1 --keyring-backend test -a --home "$CHAIN_DATA")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DATA"
              $BINARY add-genesis-account "$($BINARY --home "$CHAIN_DATA" keys show rly2 --keyring-backend test -a --home "$CHAIN_DATA")" "100000000000000$STAKEDENOM"  --home "$CHAIN_DATA"

              sed -i -e 's/timeout_commit = "5s"/timeout_commit = "1s"/g' "$CHAIN_DATA/config/config.toml"
              sed -i -e 's/timeout_propose = "3s"/timeout_propose = "1s"/g' "$CHAIN_DATA/config/config.toml"
              sed -i -e 's/index_all_keys = false/index_all_keys = true/g' "$CHAIN_DATA/config/config.toml"
              sed -i -e 's/enable = false/enable = true/g' "$CHAIN_DATA/config/app.toml"
              sed -i -e 's/swagger = false/swagger = true/g' "$CHAIN_DATA/config/app.toml"
              sed -i -e "s/minimum-gas-prices = \"\"/minimum-gas-prices = \"0.0025$STAKEDENOM,0.0025ibc\/27394FB092D2ECCD56123C74F36E4C1F926001CEADA9CA97EA622B25F41E5EB2\"/g" "$CHAIN_DATA/config/app.toml"
              sed -i -e 's/enabled = false/enabled = true/g' "$CHAIN_DATA/config/app.toml"
              sed -i -e 's/prometheus-retention-time = 0/prometheus-retention-time = 1000/g' "$CHAIN_DATA/config/app.toml"

              sed -i -e 's#"tcp://0.0.0.0:26656"#"tcp://0.0.0.0:'"$P2PPORT"'"#g' "$CHAIN_DATA/config/config.toml"
              sed -i -e 's#"tcp://127.0.0.1:26657"#"tcp://0.0.0.0:'"$RPCPORT"'"#g' "$CHAIN_DATA/config/config.toml"
              sed -i -e 's#"tcp://localhost:1317"#"tcp://0.0.0.0:'"$RESTPORT"'"#g' "$CHAIN_DATA/config/app.toml"
              sed -i -e 's#"tcp://0.0.0.0:1317"#"tcp://0.0.0.0:'"$RESTPORT"'"#g' "$CHAIN_DATA/config/app.toml"
  
              GENESIS_FILE="$CHAIN_DATA/config/genesis.json"

              sed -i -e "s/\"denom\": \"stake\",/\"denom\": \"$STAKEDENOM\",/g" "$GENESIS_FILE"
              sed -i -e "s/\"mint_denom\": \"stake\",/\"mint_denom\": \"$STAKEDENOM\",/g" "$GENESIS_FILE"
              sed -i -e "s/\"bond_denom\": \"stake\"/\"bond_denom\": \"$STAKEDENOM\"/g" "$GENESIS_FILE"
              sed -i -e 's/enabled-unsafe-cors = false/enabled-unsafe-cors = true/g' "$CHAIN_DATA/config/app.toml"              
          '';
        };   
      };
    };
}
