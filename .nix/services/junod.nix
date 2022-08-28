{
  service = {
    name = "junod-testing-local";
    image = "ghcr.io/cosmoscontracts/juno:v9.0.0";
    environment = {
      STAKE_TOKEN = "ujunox";
      UNSAFE_CORS = "true";
      USER = "juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y";
      GAS_LIMIT = 100000000;
    };
    network_mode = "host";
    command = [
      "sh"
      "-c"
      ''
        ./setup_junod.sh juno16g2rahf5846rxzp3fwlswy08fz8ccuwk03k57y
        mkdir -p /root/log
        junod start --rpc.laddr tcp://0.0.0.0:26657 --grpc.address 0.0.0.0:9099 --trace
      ''
    ];
  };
}

