{
  service = {
    name = "junod-testing-local";
    # NOTE: the do not release git hash tags, so not clear how to share client and docker image
    image = "ghcr.io/cosmoscontracts/juno:v9.0.0";
    environment = {
      STAKE_TOKEN = "ujunox";
      UNSAFE_CORS = "true";
      USER = "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y";
      GAS_LIMIT = 100000000;
    };
    # TODO: mount proper genesis here as per
    # "clip hire initial neck maid actor venue client foam budget lock catalog sweet steak waste crater broccoli pipe steak sister coyote moment obvious choose" > junod keys add alice --recover
    # `"wasm":{"codes":[],"contracts":[],"gen_msgs":[],"params":{"code_upload_access":{"address":"","permission":"Everybody"},`
    #network_mode 
    command = ''
      ./setup_and_run.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
    '';
    #network_mode = "host";
    # these ports are open by default
    ports = [
      "1317:1317" # rest openapi
      "26656:26656" # p2p
      "26657:26657" # rpc json-rpc
    ];
  };
}

