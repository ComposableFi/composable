{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, subnix, devnetTools, cosmosTools, bashTools, centauri
    , ... }:

    let
      log = " --log_level trace --trace ";
      devnet-root-directory = cosmosTools.devnet-root-directory;
      validator-mnemonic = cosmosTools.validators.mnemonic;
      gov = {
        account = "centauri10d07y265gmmuvt4z0w9aw880jnsr700j7g7ejq";
        voting_period = "20s";
        max_deposit_period = "10s";
      };
      native_denom = "ppica";
      name = "centaurid";
      cosmosLib = self.inputs.cosmos.lib {
        inherit pkgs;
        cosmwasm-check = self.inputs.cosmos.packages."${system}".cosmwasm-check;
      };

      centaurid = self.inputs.composable-cosmos.packages."${system}".centaurid;

      ibc-lightclients-wasm-v1-msg-push-new-wasm-code = code: {
        "messages" = [{
          "@type" = "/ibc.lightclients.wasm.v1.MsgPushNewWasmCode";
          "signer" = "${gov.account}";
          "code" = code;
        }];
        "deposit" = "5000000000000000ppica";
        "metadata" = "none";
        "title" = "none";
        "summary" = "none";
      };

      ics10-grandpa-cw-proposal = let
        code = builtins.readFile
          "${self'.packages.ics10-grandpa-cw}/lib/ics10_grandpa_cw.wasm.gz.txt";
        code-file = builtins.toFile "ics10_grandpa_cw.wasm.json"
          (builtins.toJSON
            (ibc-lightclients-wasm-v1-msg-push-new-wasm-code code));
      in pkgs.stdenv.mkDerivation {
        name = "ics10-grandpa-cw-proposal";
        dontUnpack = true;
        installPhase = ''
          mkdir --parents $out
          cp ${code-file} $out/ics10_grandpa_cw.wasm.json
        '';
      };
      centaurid-init = pkgs.writeShellApplication {
        name = "centaurid-init";
        runtimeInputs = devnetTools.withBaseContainerTools ++ [
          centaurid
          pkgs.jq
          self.inputs.cvm.packages."${system}".cw-cvm-executor
          self.inputs.cvm.packages."${system}".cw-cvm-outpost
        ];

        text = ''
          ${bashTools.export pkgs.networksLib.pica.devnet}
          VALIDATOR_KEY=$("$BINARY" keys show ${cosmosTools.validators.moniker} --keyring-backend=test --keyring-dir="$KEYRING_TEST" --output=json | jq .address -r )

          "$BINARY" tx gov submit-proposal ${ics10-grandpa-cw-proposal}/ics10_grandpa_cw.wasm.json --from="$VALIDATOR_KEY"  --keyring-backend test --gas 9021526220000 --fees 92000000166$FEE --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CHAIN_DATA" --output json
          sleep $BLOCK_SECONDS
          "$BINARY" query auth module-account gov --chain-id "$CHAIN_ID" --node tcp://localhost:$CONSENSUS_RPC_PORT --home "$CHAIN_DATA" | jq '.account.base_account.address' --raw-output
          PROPOSAL_ID=1
          "$BINARY" tx gov vote $PROPOSAL_ID yes --from "$VALIDATOR_KEY"  --keyring-backend test --gas 9021526220000 --fees 92000000166$FEE --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CHAIN_DATA" --output json
          sleep 20
          "$BINARY" query gov proposal $PROPOSAL_ID --chain-id "$CHAIN_ID" --node tcp://localhost:$CONSENSUS_RPC_PORT --home "$CHAIN_DATA" | jq '.status'
          sleep $BLOCK_SECONDS
          "$BINARY" query 08-wasm all-wasm-code --chain-id "$CHAIN_ID" --home "$CHAIN_DATA" --output json --node tcp://localhost:$CONSENSUS_RPC_PORT | jq '.code_ids[0]' --raw-output | tee "$CHAIN_DATA/code_id"
        '';
      };

      centaurid-cvm-config = pkgs.writeShellApplication {
        name = "centaurid-cvm-config";
        runtimeInputs = devnetTools.withBaseContainerTools ++ [
          centaurid
          pkgs.jq
          self.inputs.cvm.packages."${system}".cw-cvm-executor
          self.inputs.cvm.packages."${system}".cw-cvm-outpost
        ];

        text = ''
          KEY=${cosmosTools.cvm.centauri}
          ${bashTools.export pkgs.networksLib.pica.devnet}
          PORT=26657
          BLOCK_SECONDS=5
          FEE=ppica
          BINARY=centaurid

          CENTAURI_OUTPOST_CONTRACT_ADDRESS=$(cat $CHAIN_DATA/outpost_contract_address)
          CENTAURI_EXECUTOR_CODE_ID=$(cat $CHAIN_DATA/executor_code_id)
          OSMOSIS_OUTPOST_CONTRACT_ADDRESS=$(cat "$HOME/.osmosisd/outpost_contract_address")
          OSMOSIS_EXECUTOR_CODE_ID=$(cat "$HOME/.osmosisd/executor_code_id")
          NEUTRON_OUTPOST_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/outpost_contract_address")
          NEUTRON_EXECUTOR_CODE_ID=$(cat "$CHAIN_DATA/executor_code_id")

          FORCE_CONFIG=$(cat << EOF
              ${builtins.readFile ./../cvm.json}
          EOF
          )

          "$BINARY" tx wasm execute "$CENTAURI_OUTPOST_CONTRACT_ADDRESS" "$FORCE_CONFIG" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --yes --gas 25000000 --fees 920000166"$FEE" ${log} --keyring-backend test  --home "$CHAIN_DATA" --from APPLICATION1 --keyring-dir "$KEYRING_TEST" ${log}
          sleep $BLOCK_SECONDS
          "$BINARY" query wasm contract-state all "$CENTAURI_OUTPOST_CONTRACT_ADDRESS" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$PORT" --output json --home "$CHAIN_DATA"
        '';
      };

      centaurid-cvm-init = pkgs.writeShellApplication {
        name = "centaurid-cvm-init";
        runtimeInputs = devnetTools.withBaseContainerTools ++ [
          centaurid
          pkgs.jq
          self.inputs.cvm.packages."${system}".cw-cvm-executor
          self.inputs.cvm.packages."${system}".cw-cvm-outpost
        ];

        text = ''
          ${bashTools.export pkgs.networksLib.pica.devnet}
          KEY=${cosmosTools.cvm.centauri}

          if [[ $(curl "127.0.0.1:$CONSENSUS_RPC_PORT/block" | jq .result.block.header.height -r) -lt 5 ]]; then
           sleep 5
          fi

          function init_cvm() {
              local INSTANTIATE=$1
              "$BINARY" tx wasm store  "${
                self.inputs.cvm.packages."${system}".cw-cvm-outpost
              }/lib/cw_cvm_outpost.wasm" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --output json --yes --gas 25000000 --fees 920000166$FEE ${log} --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"
              GATEWAY_CODE_ID=1

              sleep $BLOCK_SECONDS
              "$BINARY" tx wasm store  "${
                self.inputs.cvm.packages."${system}".cw-cvm-executor
              }/lib/cw_cvm_executor.wasm" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --output json --yes --gas 25000000 --fees 920000166$FEE ${log} --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"
              EXECUTOR_CODE_ID=2
              sleep $BLOCK_SECONDS
              "$BINARY" tx wasm store  ${
                self.inputs.cosmos.packages.${system}.cw20-base
              }/lib/cw20_base.wasm --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --output json --yes --gas 25000000 --fees 920000166$FEE ${log} --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"

              sleep $BLOCK_SECONDS
              "$BINARY" tx wasm store  "${
                self.inputs.cvm.packages."${system}".cw-mantis-order
              }/lib/cw_mantis_order.wasm" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --output json --yes --gas 25000000 --fees 920000166$FEE ${log} --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST"
              ORDER_CODE_ID=4

              sleep $BLOCK_SECONDS
              "$BINARY" tx wasm instantiate2 $GATEWAY_CODE_ID "$INSTANTIATE" "2121" --label "composable_cvm_outpost" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --output json --yes --gas 25000000 --fees 920000166$FEE ${log} --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --admin "$KEY" --amount 1000000000000$FEE

              sleep $BLOCK_SECONDS
              OUTPOST_CONTRACT_ADDRESS=$("$BINARY" query wasm list-contract-by-code "$GATEWAY_CODE_ID" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --output json --home "$CHAIN_DATA" | dasel --read json '.contracts.[0]' --write yaml)
              echo "$OUTPOST_CONTRACT_ADDRESS" > "$CHAIN_DATA/outpost_contract_address"

              sleep $BLOCK_SECONDS
              echo "{\"cvm_address\": \"$OUTPOST_CONTRACT_ADDRESS\"}"
              "$BINARY" tx wasm instantiate2 $ORDER_CODE_ID "{\"cvm_address\": \"$OUTPOST_CONTRACT_ADDRESS\"}" "2121" --label "composable_mantis_order" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --output json --yes --gas 25000000 --fees 920000166$FEE ${log} --keyring-backend test  --home "$CHAIN_DATA" --from "$KEY" --keyring-dir "$KEYRING_TEST" --admin "$KEY" --amount 1000000000000$FEE


              echo "wait for next block"
              sleep $BLOCK_SECONDS
              ORDER_CONTRACT_ADDRESS=$("$BINARY" query wasm list-contract-by-code "$ORDER_CODE_ID" --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONSENSUS_RPC_PORT" --output json --home "$CHAIN_DATA" | dasel --read json '.contracts.[0]' --write yaml)
              echo "$ORDER_CONTRACT_ADDRESS" > "$CHAIN_DATA/ORDER_CONTRACT_ADDRESS"

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

      mantis-order-solve = pkgs.writeShellApplication {
        name = "mantis-order-solve";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq ];
        text = ''
          ${bashTools.export pkgs.networksLib.pica.devnet}          
          ORDER_CONTRACT_ADDRESS=$(cat "$CHAIN_DATA/ORDER_CONTRACT_ADDRESS")

          sleep $BLOCK_SECONDS
          "$BINARY" tx wasm execute "$ORDER_CONTRACT_ADDRESS" '{"order":{"msg":{"wants":{"denom":"ptest","amount":"10000"},"timeout":1000}}}' --output json --yes --gas 25000000 --fees "1000000000ppica" --amount 1234567890"$FEE" ${log} --from APPLICATION1  ${log}

          sleep $BLOCK_SECONDS
          "$BINARY" tx wasm execute "$ORDER_CONTRACT_ADDRESS" '{"order":{"msg":{"wants":{"denom":"ppica","amount":"10000"},"timeout":1000}}}' --output json --yes --gas 25000000 --fees "1000000000ptest" --amount "1234567890ptest" ${log} --from APPLICATION1  ${log}

          sleep $BLOCK_SECONDS
          "$BINARY" tx wasm execute "$ORDER_CONTRACT_ADDRESS" '{"solve":{"msg":{"routes" : [], "cows":[{"order_id":"2","cow_amount":"100000","given":"100000"},{"order_id":"3","cow_amount":"100000","given":"100000"}],"timeout":5}}}' --output json --yes --gas 25000000 --fees "1000000000ptest" --amount 1234567890"$FEE" ${log} --from APPLICATION1  ${log}

        '';
      };

      centaurid-gen-fresh = pkgs.writeShellApplication {
        name = "centaurid-gen-fresh";
        runtimeInputs = [ centaurid-gen ];
        text = ''
          centaurid-gen fresh
        '';
      };

      centaurid-gen = pkgs.writeShellApplication {
        name = "centaurid-gen";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq pkgs.dasel ];
        text = ''
          ${bashTools.export pkgs.networksLib.devnet.mnemonics}
          ${bashTools.export pkgs.networksLib.pica.devnet}

          if test "''${1-reuse}" == "fresh"; then
             echo "removing data dir"
             rm --force --recursive "$CHAIN_DATA"
          fi
          PICA_CHANNEL_ID=''${2-1}

          if [[ ! -d "$CHAIN_DATA" ]]; then
            mkdir --parents "$CHAIN_DATA"
            mkdir --parents "$CHAIN_DATA/config/gentx"
            mkdir --parents "$KEYRING_TEST"
            echo "${validator-mnemonic}" | centaurid init "$CHAIN_ID" --chain-id "$CHAIN_ID" --default-denom ${native_denom} --home "$CHAIN_DATA"  --recover

            function jq-genesis() {
              jq -r  "$1"  > "$CHAIN_DATA/config/genesis-update.json"  < "$CHAIN_DATA/config/genesis.json"
              mv --force "$CHAIN_DATA/config/genesis-update.json" "$CHAIN_DATA/config/genesis.json"
            }

            jq-genesis '.consensus_params.block.max_gas |= "-1"'
            jq-genesis '.app_state.gov.params.voting_period |= "${gov.voting_period}"'
            jq-genesis '.app_state.gov.params.max_deposit_period |= "${gov.max_deposit_period}"'

           function pica_setup() {
              jq-genesis '.app_state.transmiddleware.token_infos[0].ibc_denom |= "ibc/632DBFDB06584976F1351A66E873BF0F7A19FAA083425FEC9890C90993E5F0A4"'
              jq-genesis ".app_state.transmiddleware.token_infos[0].channel_id |= \"channel-$PICA_CHANNEL_ID\""
              jq-genesis '.app_state.transmiddleware.token_infos[0].native_denom |= "ppica"'
              jq-genesis '.app_state.transmiddleware.token_infos[0].asset_id |= "1"'
           }
           pica_setup

           function dasel-genesis() {
             dasel put --type string --file "$CHAIN_DATA/config/genesis.json" --value "$2" "$1"
           }


           register_asset () {
             dasel  put --type json --file "$CHAIN_DATA/config/genesis.json" --value "[{}]" ".app_state.bank.denom_metadata.[$1].denom_units"
             dasel-genesis ".app_state.bank.denom_metadata.[$1].description" "$2"
             dasel-genesis ".app_state.bank.denom_metadata.[$1].denom_units.[0].denom" "$2"
             dasel-genesis ".app_state.bank.denom_metadata.[$1].denom_units.[0].exponent" 0
             dasel-genesis ".app_state.bank.denom_metadata.[$1].base" "$2"
             dasel-genesis ".app_state.bank.denom_metadata.[$1].display" "$2"
             dasel-genesis ".app_state.bank.denom_metadata.[$1].name" "$2"
             dasel-genesis ".app_state.bank.denom_metadata.[$1].symbol" "$2"
           }

           dasel put --type json --file "$CHAIN_DATA/config/genesis.json" --value "[{},{}]" 'app_state.bank.denom_metadata'
           register_asset 0 "ptest"
           register_asset 1 "pdemo"

            dasel put --type=string --write=toml --file "$CHAIN_DATA/config/client.toml" --value "test" "keyring-backend"
            dasel put --type=string --write=toml --file "$CHAIN_DATA/config/client.toml" --value "json" "output"
            dasel put --type=string --write=toml --file="$CHAIN_DATA/config/client.toml" --value "$CHAIN_ID" '.chain-id'
            dasel put --type=string --write=toml --file="$CHAIN_DATA/config/client.toml" --value "sync" '.broadcast-mode'
            sed -i 's/minimum-gas-prices = "0stake"/minimum-gas-prices = "0stake"/' "$CHAIN_DATA/config/client.toml"
            
            sed -i "s/rpc-max-body-bytes = 1000000/rpc-max-body-bytes = 10000000/" "$CHAIN_DATA/config/app.toml"
            sed -i "s/swagger = false/swagger = true/" "$CHAIN_DATA/config/app.toml"
            dasel put --type string --file "$CHAIN_DATA/config/app.toml" --value "0.0.0.0:$GRPCPORT" '.grpc.address'

            dasel put --type string --file "$CHAIN_DATA/config/config.toml" --value "tcp://0.0.0.0:$CONSENSUS_GRPC_PORT" '.rpc.grpc_laddr'

            sed -i "s/cors_allowed_origins = \[\]/cors_allowed_origins = \[\"\*\"\]/" "$CHAIN_DATA/config/config.toml"
            sed -i "s/max_body_bytes = 1000000/max_body_bytes = 10000000/" "$CHAIN_DATA/config/config.toml"
            sed -i "s/max_header_bytes = 1048576/max_header_bytes = 10485760/" "$CHAIN_DATA/config/config.toml"
            sed -i "s/max_tx_bytes = 1048576/max_tx_bytes = 10485760/" "$CHAIN_DATA/config/config.toml"

            echo "document prefer nurse marriage flavor cheese west when knee drink sorry minimum thunder tilt cherry behave cute stove elder couch badge gown coral expire" | centaurid keys add alice --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "bleak slush nose opinion document sample embark couple cabbage soccer cage slow father witness canyon ring distance hub denial topic great beyond actress problem" | centaurid keys add bob --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "${validator-mnemonic}" | centaurid keys add ${cosmosTools.validators.moniker} --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius" | centaurid keys add test1 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty" | centaurid keys add test2 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "$RLY_MNEMONIC_1" | centaurid keys add relayer1 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "$RLY_MNEMONIC_2" | centaurid keys add relayer2 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "$RLY_MNEMONIC_3" | centaurid keys add relayer3 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "$RLY_MNEMONIC_4" | centaurid keys add relayer4 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "$APPLICATION1" | centaurid keys add APPLICATION1 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "$APPLICATION2" | centaurid keys add APPLICATION2 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true

            function add-genesis-account () {
              echo "adding $1"
              centaurid --keyring-backend test add-genesis-account "$1" "10000000000000000000000000000ppica,100000000000000000000000ptest,100000000000000000000000pdemo" --home "$CHAIN_DATA"
            }

            add-genesis-account "$("$BINARY" keys show relayer1 --keyring-backend test --keyring-dir "$KEYRING_TEST" --output json | jq .address -r )"
            add-genesis-account "$("$BINARY" keys show relayer2 --keyring-backend test --keyring-dir "$KEYRING_TEST" --output json | jq .address -r )"
            add-genesis-account "$("$BINARY" keys show relayer3 --keyring-backend test --keyring-dir "$KEYRING_TEST" --output json | jq .address -r )"
            add-genesis-account "$("$BINARY" keys show relayer4 --keyring-backend test --keyring-dir "$KEYRING_TEST" --output json | jq .address -r )"
            add-genesis-account "$("$BINARY" keys show APPLICATION1 --keyring-backend test --keyring-dir "$KEYRING_TEST" --output json | jq .address -r )"
            add-genesis-account "$("$BINARY" keys show APPLICATION2 --keyring-backend test --keyring-dir "$KEYRING_TEST" --output json | jq .address -r )"
            add-genesis-account "$("$BINARY" keys show ${cosmosTools.validators.moniker} --keyring-backend test --keyring-dir "$KEYRING_TEST" --output json | jq .address -r )"

            add-genesis-account centauri1zr4ng42laatyh9zx238n20r74spcrlct6jsqaw
            add-genesis-account ${cosmosTools.mantis.centauri}
            add-genesis-account centauri1cyyzpxplxdzkeea7kwsydadg87357qnamvg3y3
            add-genesis-account centauri18s5lynnmx37hq4wlrw9gdn68sg2uxp5ry85k7d
            centaurid --keyring-backend test --keyring-dir "$KEYRING_TEST" --home "$CHAIN_DATA" gentx ${cosmosTools.validators.moniker} "250000000000000ppica" --chain-id="$CHAIN_ID" --amount="250000000000000ppica"
            centaurid collect-gentxs --home "$CHAIN_DATA"  --gentx-dir "$CHAIN_DATA/config/gentx"
          else
            echo "WARNING: REUSING EXISTING DATA FOLDER"
          fi
          centaurid start --rpc.unsafe --rpc.laddr tcp://0.0.0.0:26657 --pruning=nothing --minimum-gas-prices=0.001ppica --home="$CHAIN_DATA" --db_dir="$CHAIN_DATA/data" ${log} --with-tendermint=true --transport=socket --trace-store=$CHAIN_DATA/kvstore.log --grpc.address=0.0.0.0:${
            builtins.toString pkgs.networksLib.pica.devnet.GRPCPORT
          } --grpc.enable=true --grpc-web.enable=false --api.enable=true --cpu-profile=$CHAIN_DATA/cpu-profile.log --p2p.pex=false --p2p.upnp=false
        '';
      };
    in {
      packages = rec {
        inherit centaurid centaurid-gen centaurid-init centaurid-gen-fresh
          ics10-grandpa-cw-proposal centaurid-cvm-init centaurid-cvm-config
          mantis-order-solve;

        centauri-exec = pkgs.writeShellApplication {
          name = "centaurid-cvm-config";
          runtimeInputs = devnetTools.withBaseContainerTools ++ [
            centaurid
            pkgs.jq
            self.inputs.cvm.packages."${system}".cw-cvm-executor
            self.inputs.cvm.packages."${system}".cw-cvm-outpost
          ];

          text = ''
            ${bashTools.export pkgs.networksLib.pica.devnet}
            OUTPOST_CONTRACT_ADDRESS=$(cat $CHAIN_DATA/outpost_contract_address)
            MSG=$1
            "$BINARY" tx wasm execute "$OUTPOST_CONTRACT_ADDRESS" "$MSG"  --chain-id="$CHAIN_ID"  --node=s"tcp://localhost:$CONSENSUS_RPC_PORT" --output=json --yes --gas=25000000 --fees=920000166"$FEE" ${log} --keyring-backend=test  --home="$CHAIN_DATA" --from=${cosmosTools.cvm.moniker} --keyring-dir="$KEYRING_TEST" ${log}
          '';
        };
        centauri-tx = pkgs.writeShellApplication {
          name = "centaurid-cvm-config";
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ centaurid pkgs.jq ];

          text = ''
            ${bashTools.export pkgs.networksLib.pica.devnet}
            "$BINARY" tx ibc-transfer transfer transfer channel-0 osmo1x99pkz8mk7msmptegg887wy46vrusl7kk0sudvaf2uh2k8qz7spsyy4mg8 9876543210ppica --chain-id="$CHAIN_ID"  --node "tcp://localhost:$CONENSUS_RPC_PORT" --output=json --yes --gas=25000000 --fees=920000166"$FEE" --keyring-backend=test  --home="$CHAIN_DATA" --from=${cosmosTools.cvm.moniker} --keyring-dir="$KEYRING_TEST" ${log}
          '';
        };
      };
    };
}
