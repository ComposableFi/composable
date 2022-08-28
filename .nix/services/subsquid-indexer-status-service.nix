{
  service = {
    name = "hydra-indexer-status-service";
    image = "subsquid/hydra-indexer-status-service:5";
    network_mode = "host";
    restart = "unless-stopped";
    environment = {
      REDIS_URL = "redis://127.0.0.1:6379/0";
      PORT = 8081;
    };
  };
}
