{
  service = {
    name = "subsquid-substrate-gateway";
    image = "subsquid/substrate-gateway:firesquid";
    restart = "always";
    environment = {
      RUST_LOG = "substrate_gateway=info,actix_server=info";
      # DEV_MODE = "true";
      # DB_NAME = database.name;
      # DB_HOST = database.host;
      # DB_USER = database.user;
      # DB_PASS = database.password;
      # DB_PORT = database.port;
      # HYDRA_INDEXER_STATUS_SERVICE = "http://${status}:8081/status";
    };
    command = [
        "--database-url" "postgres://postgres:postgres@db:5432/squid-archive"
        "--database-max-connections" "3" # max number of concurrent database connections
        "--evm-support" # uncomment for chains with Frontier EVM pallet
        # (e.g. Moonbeam/Moonriver or Astar/Shiden) 
    ];
    ports = [ "8888:8888" ];
  };
}
