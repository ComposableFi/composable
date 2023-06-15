{ self, ... }: {
  perSystem =
    { config
    , self'
    , inputs'
    , pkgs
    , lib
    , system
    , crane
    , systemCommonRust
    , subnix
    , ...
    }:
    let
      validator = "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n";
      validator-mnemonic = "bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort";
      gov = {
        account = "centauri10d07y265gmmuvt4z0w9aw880jnsr700j7g7ejq";
        voting_period     = "20s";
        max_deposit_period = "10s" ;               
      };
      name = "centaurid";
      centaurid = pkgs.buildGoModule {
        name = name;
        doCheck = false;
        nativeBuildInputs = [ pkgs.patchelf ];
        excludedPackages = [ "interchaintest" "simd" ];
        ldflags = [ "-v -extldflags '-L${self'.packages.libwasmvm}/lib'" ];
        src = pkgs.fetchFromGitHub {
          owner = "notional-labs";
          repo = "composable-centauri";
          rev = "762ad8efd57695ff4ee469aa6923fe7e81578c61";
          sha256 = "sha256-/wN246AZ4eO9Z2+q7iLLw33G+7cbUCCtetA2BFDRlj8=";
        };
        dontFixup = true;
        vendorSha256 = "sha256-xbHVzucr/5B9ePP3Dnhag9KaFKvS238sTgkoxsD3LJ8=";
      };

      ibc-lightclients-wasm-v1-msg-push-new-wasm-code = code: {
        "messages" = [
          {
            "@type" = "/ibc.lightclients.wasm.v1.MsgPushNewWasmCode";
            "signer" = "${gov.account}";
            "code" = code;
          }
        ];
        "deposit" = "5000000000000000ppica";
        "metadata" = "none";
        "title" = "none";
        "summary" = "none";
      };

      ics10-grandpa-cw-proposal =
        let
          code = builtins.readFile
            "${self'.packages.ics10-grandpa-cw}/lib/ics10_grandpa_cw.wasm.gz.txt";
          code-file = builtins.toFile
            "ics10_grandpa_cw.wasm.json"
            (builtins.toJSON (ibc-lightclients-wasm-v1-msg-push-new-wasm-code code));
        in
        pkgs.stdenv.mkDerivation
          {
            name = "ics10-grandpa-cw-proposal";
            dontUnpack = true;
            #buildInputs = [ self'.packages.ics10-grandpa-cw ];
            installPhase = ''
              mkdir --parents $out
              cp ${code-file} $out/ics10_grandpa_cw.wasm.json
            '';
          };

      CW20_BASE_WASM = pkgs.fetchurl
        {
          url =
            "https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm";
          hash = "sha256-nClak9UDPLdALVnN7e9yVKafnKUO7RAYDFO7sxwAXpI=";
        };
      WYNDEX_PAIR_WASM = pkgs.fetchurl
        {
          url =
            "https://github.com/wynddao/wynddex/releases/download/v2.1.0/wyndex_pair.wasm";
          hash = "sha256-GQh3SBVccriWhHNPe22VMGWJVqfJa7x3cWy67j6NFTg=";
        };

      WYNDEX_FACTORY_WASM = pkgs.fetchurl
        {
          url =
            "https://github.com/wynddao/wynddex/releases/download/v2.1.0/wyndex_factory.wasm";
          hash = "sha256-2ZYILTelKNsuqfOisXhrg4TPLwocaVNp6UN+6LN51SQ=";
        };
      centaurid-init = pkgs.writeShellApplication
        {
          name = "centaurid-init";
          runtimeInputs = [ centaurid pkgs.jq pkgs.yq ];

          text = ''
            CENTAURI_DATA="/tmp/centauri-dev"
            CHAIN_ID="centauri-dev"
            KEYRING_TEST="$CENTAURI_DATA/keyring-test"
            # centaurid query bank balances ${validator} --chain-id "$CHAIN_ID" --node tcp://localhost:26657 --home "$CENTAURI_DATA"
            # centaurid query bank balances centauri1cyyzpxplxdzkeea7kwsydadg87357qnamvg3y3 --chain-id "$CHAIN_ID" --node tcp://localhost:26657 --home "$CENTAURI_DATA"
            # centaurid tx bank send validator centauri1cyyzpxplxdzkeea7kwsydadg87357qnamvg3y3 1000000000ppica --from validator --keyring-backend test --gas 902152622 --fees 920166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --output json
            # sleep 5
            # centaurid query bank balances ${validator} --chain-id "$CHAIN_ID" --node tcp://localhost:26657 --home "$CENTAURI_DATA"
            # centaurid query bank balances centauri1cyyzpxplxdzkeea7kwsydadg87357qnamvg3y3 --chain-id "$CHAIN_ID" --node tcp://localhost:26657 --home "$CENTAURI_DATA"            
            
            echo "=============== SUBMIT PROPOSAL ========"
            centaurid tx gov submit-proposal ${ics10-grandpa-cw-proposal}/ics10_grandpa_cw.wasm.json --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json
            sleep 5          
            
            echo "=============== VOTE PROPOSAL ========="
            PROPOSAL_ID=1
            centaurid tx gov vote $PROPOSAL_ID yes --from "${validator}"  --keyring-backend test --gas 9021526220000 --fees 92000000166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes --home "$CENTAURI_DATA" --output json
            sleep 20
            echo "=============== GET PROPOSAL ========="
            centaurid query gov proposal $PROPOSAL_ID --chain-id "$CHAIN_ID" --node tcp://localhost:26657 --home "$CENTAURI_DATA" |
            jq '.status'
            sleep 5
            echo "=============== GET WASM ========="
            centaurid query 08-wasm all-wasm-code --chain-id "$CHAIN_ID" --home "$CENTAURI_DATA" --output json --node tcp://localhost:26657 | jq '.code_ids[0]' | tee "$CENTAURI_DATA/code_id"
          '';
        };

      centaurid-gen = pkgs.writeShellApplication
        {
          name = "centaurid-gen";
          runtimeInputs = [ centaurid pkgs.jq pkgs.yq ];
          text = ''
            CENTAURI_DATA="/tmp/centauri-dev"
            CHAIN_ID="centauri-dev"
            KEYRING_TEST="$CENTAURI_DATA/keyring-test"
            rm --force --recursive "$CENTAURI_DATA"
            
            mkdir --parents "$CENTAURI_DATA"
            mkdir --parents "$CENTAURI_DATA/config/gentx"
            mkdir --parents "$KEYRING_TEST"
            echo "${validator-mnemonic}" | centaurid init "$CHAIN_ID" --chain-id "$CHAIN_ID" --default-denom ppica --home "$CENTAURI_DATA"  --recover           
            
            function jq-genesis() {
              jq -r  "$1"  > "$CENTAURI_DATA/config/genesis-update.json"  < "$CENTAURI_DATA/config/genesis.json"
              mv --force "$CENTAURI_DATA/config/genesis-update.json" "$CENTAURI_DATA/config/genesis.json"
            }
            
            jq-genesis '.consensus_params.block.max_gas |= "-1"'  
            jq-genesis '.app_state.gov.params.voting_period |= "${gov.voting_period}"'  
            jq-genesis '.app_state.gov.params.max_deposit_period |= "${gov.max_deposit_period}"'  
            jq-genesis '.app_state.gov.params.min_deposit[0].amount |= "1"'  
            sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$CENTAURI_DATA/config/client.toml"
            sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$CENTAURI_DATA/config/client.toml"            
            sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$CENTAURI_DATA/config/client.toml"
            sed -i 's/output = "text"/output = "json"/' "$CENTAURI_DATA/config/client.toml"
            sed -i "s/cors_allowed_origins = \[\]/cors_allowed_origins = \[\"\*\"\]/" "$CENTAURI_DATA/config/config.toml"
            sed -i   "s/enable = false/enable = true/" "$CENTAURI_DATA/config/app.toml"
            #sed -i   "s/swagger = false/swagger = true/" "$CENTAURI_DATA/config/app.toml"           
            sed -i   "s/rpc-max-body-bytes = 1000000/rpc-max-body-bytes = 10000000/" "$CENTAURI_DATA/config/app.toml"
            sed -i   "s/max_body_bytes = 1000000/max_body_bytes = 10000000/" "$CENTAURI_DATA/config/config.toml"
            sed -i   "s/max_header_bytes = 1048576/max_header_bytes = 10485760/" "$CENTAURI_DATA/config/config.toml"
            sed -i   "s/max_tx_bytes = 1048576/max_tx_bytes = 10485760/" "$CENTAURI_DATA/config/config.toml"

            echo "document prefer nurse marriage flavor cheese west when knee drink sorry minimum thunder tilt cherry behave cute stove elder couch badge gown coral expire" | centaurid keys add alice --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true    
            echo "bleak slush nose opinion document sample embark couple cabbage soccer cage slow father witness canyon ring distance hub denial topic great beyond actress problem" | centaurid keys add bob --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "coffee hospital claim ability wrap load display submit lecture solid secret law base barrel miss tattoo desert want wall bar ketchup sauce real unknown" | centaurid keys add charlie --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "VALIDATOR:"
            echo "${validator-mnemonic}" | centaurid keys add validator --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius" | centaurid keys add test1 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty" | centaurid keys add test2 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            echo "symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb" | centaurid keys add test3 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
            function add-genesis-account () {
              centaurid --keyring-backend test add-genesis-account "$1" "1000000000000000000000ppica" --keyring-backend test --home "$CENTAURI_DATA"          
            }
            add-genesis-account centauri1zr4ng42laatyh9zx238n20r74spcrlct6jsqaw
            add-genesis-account centauri1makf5hslxqxzl29uyeyyddf89ff7edxyr7ewm5
            add-genesis-account ${validator}
            add-genesis-account centauri1cyyzpxplxdzkeea7kwsydadg87357qnamvg3y3
            add-genesis-account centauri18s5lynnmx37hq4wlrw9gdn68sg2uxp5ry85k7d
            add-genesis-account centauri1qwexv7c6sm95lwhzn9027vyu2ccneaqapystyu
            centaurid --keyring-backend test --keyring-dir "$KEYRING_TEST" --home "$CENTAURI_DATA" gentx validator "250000000000000ppica" --chain-id="$CHAIN_ID" --amount="250000000000000ppica"
            centaurid collect-gentxs --home "$CENTAURI_DATA"  --gentx-dir "$CENTAURI_DATA/config/gentx"
            centaurid start --rpc.laddr tcp://0.0.0.0:26657 --pruning=nothing  --minimum-gas-prices=0.0001ppica --log_level debug --home "$CENTAURI_DATA" --db_dir "$CENTAURI_DATA/data" --log_format json --trace --with-tendermint true --transport socket --trace-store $CENTAURI_DATA/kvstore.log  --grpc.enable true --grpc-web.enable true --api.enable true --cpu-profile $CENTAURI_DATA/cpu-profile.log
          '';
        };
    in
    {
      packages = rec { inherit centaurid centaurid-gen centaurid-init ics10-grandpa-cw-proposal; };
    };
}
