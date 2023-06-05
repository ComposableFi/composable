{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, lib, system, crane
    , systemCommonRust, subnix, ... }:
    let
      name = "banksyd";
      banksyd = pkgs.buildGoModule {
        name = name;
        doCheck = false;
        nativeBuildInputs = [ pkgs.patchelf ];
        excludedPackages = [ "interchaintest" "simd" ];
        ldflags = [ "-v -extldflags '-L${self'.packages.libwasmvm}/lib'" ];
        src = pkgs.fetchFromGitHub {
          owner = "notional-labs";
          repo = "composable-centauri";
          rev = "bypass";
          sha256 = "sha256-XLTBYWDvFwirss3JbbVCwYfG3qIGya35HulvFS7SYaM=";
        };
        dontFixup = true;
        vendorSha256 = "sha256-4K0MLZ8FUaAy4vkPONO694jolOzGHYACZRJAfU8Qn4s=";
      };
      CW20_BASE_WASM = pkgs.fetchurl {
        url =
          "https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm";
        hash = "sha256-nClak9UDPLdALVnN7e9yVKafnKUO7RAYDFO7sxwAXpI=";
      };
      WYNDEX_PAIR_WASM = pkgs.fetchurl {
        url =
          "https://github.com/wynddao/wynddex/releases/download/v2.1.0/wyndex_pair.wasm";
        hash = "sha256-GQh3SBVccriWhHNPe22VMGWJVqfJa7x3cWy67j6NFTg=";
      };

      WYNDEX_FACTORY_WASM = pkgs.fetchurl {
        url =
          "https://github.com/wynddao/wynddex/releases/download/v2.1.0/wyndex_factory.wasm";
        hash = "sha256-2ZYILTelKNsuqfOisXhrg4TPLwocaVNp6UN+6LN51SQ=";
      };
      banksyd-init = pkgs.writeShellApplication {
        name = "banksyd-init";
        runtimeInputs = [ banksyd pkgs.jq pkgs.yq ];

        text = ''
          BANKSY="/tmp/banksy-dev"
          CHAIN_ID="centauri-dev"
          KEYRING_TEST="$BANKSY/keyring-test"
          banksyd query bank balances banksy1cyyzpxplxdzkeea7kwsydadg87357qna4p6c6f
          banksyd tx 08-wasm push-wasm ${CW20_BASE_WASM} --from alice --keyring-backend test --gas 902152622 --fees 920166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes
          sleep 5
          banksyd tx 08-wasm push-wasm ${WYNDEX_PAIR_WASM} --from alice --keyring-backend test --gas 902152622 --fees 920166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes
          sleep 5
          banksyd tx 08-wasm push-wasm ${WYNDEX_FACTORY_WASM} --from alice --keyring-backend test --gas 902152622 --fees 920166ppica --keyring-dir "$KEYRING_TEST" --chain-id "$CHAIN_ID" --yes
          sleep 5
          banksyd  query 08-wasm all-wasm-code --chain-id "$CHAIN_ID" --home "$BANKSY" --output json --node tcp://localhost:26657
        '';
      };

      banksyd-gen = pkgs.writeShellApplication {
        name = "banksyd-gen";
        runtimeInputs = [ banksyd pkgs.jq pkgs.yq ];
        text = ''
          BANKSY="/tmp/banksy-dev"
          CHAIN_ID="centauri-dev"
          KEYRING_TEST="$BANKSY/keyring-test"
          rm --force --recursive "$BANKSY"
          mkdir --parents "$BANKSY"
          mkdir --parents "$BANKSY/config/gentx"
          mkdir --parents "$KEYRING_TEST"
          banksyd init "$CHAIN_ID" --chain-id "$CHAIN_ID" --default-denom ppica --home "$BANKSY"           
          jq -r  '.consensus_params.block.max_gas |= "-1" '  > "$BANKSY/config/genesis-update.json"  < "$BANKSY/config/genesis.json"
          sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$BANKSY/config/client.toml"
          mv --force "$BANKSY/config/genesis-update.json" "$BANKSY/config/genesis.json"
          sed -i 's/keyring-backend = "os"/keyring-backend = "test"/' "$BANKSY/config/client.toml"
          sed -i 's/output = "text"/output = "json"/' "$BANKSY/config/client.toml"
          sed -i "s/cors_allowed_origins = \[\]/cors_allowed_origins = \[\"\*\"\]/" "$BANKSY/config/config.toml"
          sed -i   "s/enable = false/enable = true/" "$BANKSY/config/app.toml"
          sed -i   "s/swagger = false/swagger = true/" "$BANKSY/config/app.toml"           
          echo "document prefer nurse marriage flavor cheese west when knee drink sorry minimum thunder tilt cherry behave cute stove elder couch badge gown coral expire" | banksyd keys add alice --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true    
          echo "bleak slush nose opinion document sample embark couple cabbage soccer cage slow father witness canyon ring distance hub denial topic great beyond actress problem" | banksyd keys add bob --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "coffee hospital claim ability wrap load display submit lecture solid secret law base barrel miss tattoo desert want wall bar ketchup sauce real unknown" | banksyd keys add charlie --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort" | banksyd keys add validator --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "notice oak worry limit wrap speak medal online prefer cluster roof addict wrist behave treat actual wasp year salad speed social layer crew genius" | banksyd keys add test1 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "quality vacuum heart guard buzz spike sight swarm shove special gym robust assume sudden deposit grid alcohol choice devote leader tilt noodle tide penalty" | banksyd keys add test2 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          echo "symbol force gallery make bulk round subway violin worry mixture penalty kingdom boring survey tool fringe patrol sausage hard admit remember broken alien absorb" | banksyd keys add test3 --recover --keyring-backend test --keyring-dir "$KEYRING_TEST" || true
          banksyd --keyring-backend test add-genesis-account banksy1cyyzpxplxdzkeea7kwsydadg87357qna4p6c6f "1000000000000000ppica" --keyring-backend test --home "$BANKSY"
          banksyd --keyring-backend test add-genesis-account banksy18s5lynnmx37hq4wlrw9gdn68sg2uxp5r22xlq4 "1000000000000000ppica" --keyring-backend test --home "$BANKSY"
          banksyd --keyring-backend test add-genesis-account banksy1qwexv7c6sm95lwhzn9027vyu2ccneaqa0fzz6y "1000000000000000ppica" --keyring-backend test --home "$BANKSY"
          banksyd --keyring-backend test add-genesis-account banksy1xtf3wlewqpnzgu20460fjjuc7vrkmysm5xr9e3 "1000000000000000ppica" --keyring-backend test --home "$BANKSY"
          banksyd --keyring-backend test add-genesis-account banksy1zr4ng42laatyh9zx238n20r74spcrlct5lzfrk "1000000000000000ppica" --keyring-backend test --home "$BANKSY"
          banksyd --keyring-backend test add-genesis-account banksy1makf5hslxqxzl29uyeyyddf89ff7edxydnt89v "1000000000000000ppica" --keyring-backend test --home "$BANKSY"
          banksyd --keyring-backend test add-genesis-account banksy12smx2wdlyttvyzvzg54y2vnqwq2qjate74jwmt "1000000000000000ppica" --keyring-backend test --home "$BANKSY"
          banksyd --keyring-backend test --keyring-dir "$KEYRING_TEST" --home "$BANKSY" gentx validator "250000000000000ppica" --chain-id="$CHAIN_ID" --amount="250000000000000ppica"
          banksyd collect-gentxs --home "$BANKSY"  --gentx-dir "$BANKSY/config/gentx"
          banksyd start --rpc.laddr tcp://0.0.0.0:26657 --pruning=nothing  --minimum-gas-prices=0.0001ppica --trace --log_level debug --home "$BANKSY" --db_dir "$BANKSY/data" --log_format json --trace           
        '';
      };
    in {
      packages = rec {
      } // (pkgs.lib.optionalAttrs (system == "x86_64-linux") {
        inherit banksyd banksyd-gen banksyd-init;
      });
    };
}
