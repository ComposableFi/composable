{ redis, database, parachain, parachainPort }: {
  service = {
    name = "hydra-indexer";
    image = "subsquid/hydra-indexer:5";
    restart = "always";
    environment = {
      WORKERS_NUMBER = 5;
      DB_NAME = database.name;
      DB_HOST = database.host;
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_PORT = database.port;
      REDIS_URI = "redis://${redis}:6379/0";
      FORCE_HEIGHT = "true";
      WS_PROVIDER_ENDPOINT_URI = "ws://${parachain}:${toString parachainPort}";
    };
    command = [ "sh" "-c" "yarn db:bootstrap && yarn start:prod" ];
  };
}
