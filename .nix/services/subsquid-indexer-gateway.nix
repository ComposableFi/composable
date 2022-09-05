{ status, database, graphql-port }: {
  service = {
    name = "hydra-indexer-gateway";
    image = "subsquid/hydra-indexer-gateway:5";
    restart = "always";
    environment = {
      DEV_MODE = "true";
      DB_NAME = database.name;
      DB_HOST = database.host;
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_PORT = database.port;
      HYDRA_INDEXER_STATUS_SERVICE = "http://${status}:8081/status";
    };
    ports = [ "${toString graphql-port}:8080" ];
  };
}
