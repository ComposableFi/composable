{ ... }: {
  perSystem = { self', pkgs, systemCommonRust, subnix, lib, ... }: {
    # 1. run hyperspae in process compose
    # 2. run centaurid in process compose
    # 3. run zombienet in process compose
    # 4. run run hyperspace with some configuraiton
    # 5. build hyperspace wasm
    # 6. upload hyperspace wasm
    # 7. run hyperspace with connected configuraiton


#     type = "cosmos"
# name = "centauri"
# rpc_url = "http://127.0.0.1:80"
# grpc_url = "http://127.0.0.1:9090"
# websocket_url = "wss://ws-banksy.notional.ventures/"
# chain_id = "banksy-testnet-1"
# client_id = "07-tendermint-32"
# connection_id = "connection-0"
# account_prefix = "banksy"
# fee_denom = "ubanksy"
# fee_amount = "15000"
# gas_limit = 9223372036854775806
# store_prefix = "ibc"
# max_tx_size = 20000000
# wasm_code_id = "714a9a70fd46af31c8cf3bcb3972edf4f428a2647c4014071221bdc9ad9547bb"
# channel_whitelist = []

# mnemonic = "<..>
    process-compose.devnet-cosmos = {
      settings = {
        processes = {
          centauri = {
            command =self'.packages.centaurid-gen;
          };
          picasso = {
            command =self'.packages.zombienet-rococo-local-picasso-dev;
          };
          hyperspace = {
            command =self'.packages.hyperspace-composable-rococo-picasso-rococo;
          };
        };
      };
    };
  };
}
