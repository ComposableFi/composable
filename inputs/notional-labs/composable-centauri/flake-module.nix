{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, subnix, devnetTools, cosmosTools, ... }:
    let
      devnet-root-directory = cosmosTools.devnet-root-directory;
      validator-mnemonic = cosmosTools.validators.mnemonic;
      validator = cosmosTools.validators.centauri;
      gov = {
        account = "centauri10d07y265gmmuvt4z0w9aw880jnsr700j7g7ejq";
        voting_period = "20s";
        max_deposit_period = "10s";
      };
      native_denom = "ppica";
      name = "centaurid";
      centaurid = pkgs.writeShellApplication {
        name = "centaurid";
        text = ''
          ${self.inputs.cosmos.packages.${system}.centauri}/bin/centaurid "$@"
        '';
      };

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
        #buildInputs = [ self'.packages.ics10-grandpa-cw ];
        installPhase = ''
          mkdir --parents $out
          cp ${code-file} $out/ics10_grandpa_cw.wasm.json
        '';
      };
      centaurid-init = pkgs.writeShellApplication {
        name = "centaurid-init";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq self'.packages.xc-cw-contracts ];

        text = ''
          CENTAURI_DATA="${devnet-root-directory}/.centaurid"
          CHAIN_ID="centauri-dev"
          KEYRING_TEST="$CENTAURI_DATA/keyring-test"
          BLOCK_SECONDS=5
          centaurid tx gov submit-proposal ${ics10-grandpa-cw-proposal}/ics10_grandpa_cw.wasm.json --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json
          sleep $BLOCK_SECONDS
          centaurid query auth module-account gov --chain-id "$CHAIN_ID" --node tcp://localhost:26657 --home "$CENTAURI_DATA" | jq '.account.base_account.address' --raw-output
          PROPOSAL_ID=1          
          centaurid tx gov vote $PROPOSAL_ID yes --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json          
          sleep 20          
          centaurid query gov proposal $PROPOSAL_ID --chain-id "$CHAIN_ID" --node tcp://localhost:26657 --home "$CENTAURI_DATA" |
          jq '.status'
          sleep $BLOCK_SECONDS         
          centaurid query 08-wasm all-wasm-code --chain-id "$CHAIN_ID" --home "$CENTAURI_DATA" --output json --node tcp://localhost:26657 | jq '.code_ids[0]' --raw-output | tee "$CENTAURI_DATA/code_id"
          centaurid tx wasm store ${self'.packages.xc-cw-contracts}/lib/cw_xc_gateway.wasm --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json          
          sleep $BLOCK_SECONDS
          centaurid tx wasm store ${self'.packages.xc-cw-contracts}/lib/cw_xc_interpreter.wasm --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json          
          sleep $BLOCK_SECONDS
          centaurid tx wasm store ${self'.packages.wyndex-pair} --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json          
          sleep $BLOCK_SECONDS
          centaurid tx wasm store ${self'.packages.wyndex-factory} --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json          
          sleep $BLOCK_SECONDS
          centaurid tx wasm store ${self'.packages.cw20_base} --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json          
          sleep $BLOCK_SECONDS
          centaurid tx wasm store ${self'.packages.cw4_stake} --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json          
          centaurid query wasm list-code --chain-id "$CHAIN_ID" --home "$CENTAURI_DATA" --output json --node tcp://localhost:26657
        '';
      };

      centaurid-dev = pkgs.writeShellApplication {
        name = "centaurid-dev";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq ];

        text = ''
          #CENTAURI_DATA="${devnet-root-directory}/.centaurid"
          CHAIN_ID="centauri-dev"
          #KEYRING_TEST="$CENTAURI_DATA/keyring-test"
          centaurid query ibc connection connections --chain-id "$CHAIN_ID"
          #--from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST"  --yes --home "$CENTAURI_DATA" --output json
        '';
      };

      centaurid-gen-fresh = pkgs.writeShellApplication {
        name = "centaurid-gen-fresh";
        runtimeInputs = [ centaurid-gen ];
        text = ''
          rm --force --recursive ${devnet-root-directory} 
          centaurid-gen
        '';
      };

      centaurid-gen = pkgs.writeShellApplication {
        name = "centaurid-gen";
        runtimeInputs = devnetTools.withBaseContainerTools
          ++ [ centaurid pkgs.jq ];
        text = ''
          CENTAURI_DATA="${devnet-root-directory}/.centaurid"
          CHAIN_ID="centauri-dev"
          KEYRING_TEST="$CENTAURI_DATA/keyring-test"
          REUSE=true
          export REUSE
          if [[ $REUSE == false ]]; then
            rm --force --recursive "$CENTAURI_DATA" 
          fi

          mkdir --parents "$CENTAURI_DATA"
          mkdir --parents "$CENTAURI_DATA/config/gentx"
          mkdir --parents "$KEYRING_TEST"
          echo "${validator-mnemonic}" | centaurid init "$CHAIN_ID" --chain-id "$CHAIN_ID" --default-denom ${native_denom} --home "$CENTAURI_DATA"  --recover           

          function jq-genesis() {
            jq -r  "$1"  > "$CENTAURI_DATA/config/genesis-update.json"  < "$CENTAURI_DATA/config/genesis.json"
            mv --force "$CENTAURI_DATA/config/genesis-update.json" "$CENTAURI_DATA/config/genesis.json"
          }

          jq-genesis '.consensus_params.block.max_gas |= "-1"'  
          jq-genesis '.app_state.gov.params.voting_period |= "${gov.voting_period}"'  
          jq-genesis '.app_state.gov.params.max_deposit_period |= "${gov.max_deposit_period}"'  

          jq-genesis '.app_state.transmiddleware.token_infos[0].ibc_denom |= "ibc/632DBFDB06584976F1351A66E873BF0F7A19FAA083425FEC9890C90993E5F0A4"'            
          jq-genesis '.app_state.transmiddleware.token_infos[0].channel_id |= "channel-0"'  
          jq-genesis '.app_state.transmiddleware.token_infos[0].native_denom |= "ppica"'
          jq-genesis '.app_state.transmiddleware.token_infos[0].asset_id |= "1"'

          sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$CENTAURI_DATA/config/client.toml"
          sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$CENTAURI_DATA/config/client.toml"            
          sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$CENTAURI_DATA/config/client.toml"
          sed -i 's/output = "text"/output = "json"/' "$CENTAURI_DATA/config/client.toml"
          sed -i "s/cors_allowed_origins = \[\]/cors_allowed_origins = \[\"\*\"\]/" "$CENTAURI_DATA/config/config.toml"
          sed -i "s/swagger = false/swagger = true/" "$CENTAURI_DATA/config/app.toml"           
          sed -i "s/rpc-max-body-bytes = 1000000/rpc-max-body-bytes = 10000000/" "$CENTAURI_DATA/config/app.toml"
          sed -i "s/max_body_bytes = 1000000/max_body_bytes = 10000000/" "$CENTAURI_DATA/config/config.toml"
          sed -i "s/max_header_bytes = 1048576/max_header_bytes = 10485760/" "$CENTAURI_DATA/config/config.toml"
          sed -i "s/max_tx_bytes = 1048576/max_tx_bytes = 10485760/" "$CENTAURI_DATA/config/config.toml"

          echo "document prefer nurse marriage flavor cheese west when knee drink sorry minimum thunder tilt cherry behave cute stove elder couch badge gown coral expire" | centaurid keys add alice --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true    
          echo "bleak slush nose opinion document sample embark couple cabbage soccer cage slow father witness canyon ring distance hub denial topic great beyond actress problem" | centaurid keys add bob --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "coffee hospital claim ability wrap load display submit lecture solid secret law base barrel miss tattoo desert want wall bar ketchup sauce real unknown" | centaurid keys add charlie --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "VALIDATOR:"
          echo "${validator-mnemonic}" | centaurid keys add ${cosmosTools.validators.moniker} --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius" | centaurid keys add test1 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty" | centaurid keys add test2 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb" | centaurid keys add test3 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "black frequent sponsor nice claim rally hunt suit parent size stumble expire forest avocado mistake agree trend witness lounge shiver image smoke stool chicken" | centaurid keys add relayer --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          function add-genesis-account () {
            centaurid --keyring-backend test add-genesis-account "$1" "1000000000000000000000ppica" --keyring-backend test --home "$CENTAURI_DATA"          
          }

          add-genesis-account centauri1qvdeu4x34rapp3wc8fym5g4wu343mswxxgc6wf
          add-genesis-account centauri1zr4ng42laatyh9zx238n20r74spcrlct6jsqaw
          add-genesis-account centauri1makf5hslxqxzl29uyeyyddf89ff7edxyr7ewm5
          add-genesis-account ${validator}
          add-genesis-account centauri1cyyzpxplxdzkeea7kwsydadg87357qnamvg3y3
          add-genesis-account centauri18s5lynnmx37hq4wlrw9gdn68sg2uxp5ry85k7d
          add-genesis-account centauri1qwexv7c6sm95lwhzn9027vyu2ccneaqapystyu
          centaurid --keyring-backend test --keyring-dir "$KEYRING_TEST" --home "$CENTAURI_DATA" gentx ${cosmosTools.validators.moniker} "250000000000000ppica" --chain-id="$CHAIN_ID" --amount="250000000000000ppica"
          centaurid collect-gentxs --home "$CENTAURI_DATA"  --gentx-dir "$CENTAURI_DATA/config/gentx"
          centaurid start --rpc.unsafe --rpc.laddr tcp://0.0.0.0:26657 --pruning=nothing --minimum-gas-prices=0ppica --log_level debug --home "$CENTAURI_DATA" --db_dir "$CENTAURI_DATA/data" --trace --with-tendermint true --transport socket --trace-store $CENTAURI_DATA/kvstore.log --grpc.address localhost:9090 --grpc.enable true --grpc-web.enable false --api.enable true --cpu-profile $CENTAURI_DATA/cpu-profile.log --p2p.pex false --p2p.upnp  false
        '';
      };
    in {
      packages = rec {
        inherit centaurid centaurid-gen centaurid-init centaurid-dev
          centaurid-gen-fresh ics10-grandpa-cw-proposal;
      };
    };
}
