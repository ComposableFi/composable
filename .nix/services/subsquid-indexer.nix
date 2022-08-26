{
  service = {
    name = "hydra-indexer";
    image = "subsquid/hydra-indexer:5";
    network_mode = "host";
    restart = "unless-stopped";
    environment = {
      WORKERS_NUMBER = 5;
      DB_NAME = "indexer";
      DB_HOST = "localhost";
      DB_USER = "postgres";
      DB_PASS = "postgres";
      DB_PORT = 5432;
      REDIS_URI = "redis://localhost:6379/0";
      FORCE_HEIGHT = "true";
      WS_PROVIDER_ENDPOINT_URI = "ws://127.0.0.1:9988";
    };
    command = [
      "sh"
      "-c"
      "yarn db:bootstrap && yarn start:prod"
    ];
  };
}