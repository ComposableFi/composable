{ redis, port }: {
  service = {
    name = "hydra-indexer-status-service";
    image = "subsquid/hydra-indexer-status-service:5";
    restart = "always";
    environment = {
      REDIS_URI = "redis://${redis}:6379/0";
      PORT = port;
    };
  };
}
