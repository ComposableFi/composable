{ database }: {
  service = {
    name = "hydra-indexer-gateway";
    image = "subsquid/hydra-indexer-gateway:5";
    network_mode = "host";
    restart = "unless-stopped";
    environment = {
      DEV_MODE = "true";
      DB_NAME = database.name;
      DB_HOST = database.host;
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_PORT = database.port;
      HYDRA_INDEXER_STATUS_SERVICE = "http://127.0.0.1:8081/status";
    };
  };
}
