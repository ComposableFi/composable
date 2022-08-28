{ database }: {
  service = {
    name = "hydra-indexer";
    image = "subsquid/hydra-indexer:5";
    network_mode = "host";
    restart = "unless-stopped";
    environment = {
      WORKERS_NUMBER = 5;
      DB_NAME = database.name;
      DB_HOST = database.host;
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_PORT = database.port;
      REDIS_URI = "redis://127.0.0.1:6379/0";
      FORCE_HEIGHT = "true";
      WS_PROVIDER_ENDPOINT_URI = "ws://127.0.0.1:9988";
    };
    command = [ "sh" "-c" "yarn db:bootstrap && yarn start:prod" ];
  };
}
