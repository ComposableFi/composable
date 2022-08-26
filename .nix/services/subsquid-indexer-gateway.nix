{
  service = {
    name = "hydra-indexer-gateway";
    image = "subsquid/hydra-indexer-gateway:5";
    network_mode = "host";
    restart = "unless-stopped";
    environment = {
      DEV_MODE = "true";
      DB_NAME = "indexer";
      DB_HOST = "localhost";
      DB_USER = "postgres";
      DB_PASS = "postgres";
      DB_PORT = 5432;
      HYDRA_INDEXER_STATUS_SERVICE =
        "http://localhost:8081/status";
    };
  };
}