{ pkgs, host }: {
  ibc-relayer-config-picasso-kusama-to-centauri-0-0 = {
    type = "cosmos";
    name = "centauri";
    rpc_url = "http://${host}:${
        builtins.toString pkgs.networksLib.pica.devnet.RPCPORT
      }";
    grpc_url = "http://${host}:${
        builtins.toString pkgs.networksLib.pica.devnet.GRPCPORT
      }";
    websocket_url = "ws://${host}:26657/websocket";
    chain_id = pkgs.networksLib.pica.devnet.CHAIN_ID;
    client_id = "07-tendermint-0";
    connection_id = "connection-0";
    account_prefix = "centauri";
    fee_denom = "ppica";
    fee_amount = "10000000000000000";
    gas_limit = 9223372036854775806;
    store_prefix = "ibc";
    max_tx_size = 20000000;
    wasm_code_id =
      "0000000000000000000000000000000000000000000000000000000000000000";
    skip_optional_client_updates = false;
    channel_whitelist = [ ];
    skip_tokens_list = [ ];
    mnemonic =
      "bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort";
  };
}
