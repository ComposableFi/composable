{
  service = {
    image = "redis:6.0-alpine";
    network_mode = "host";
    restart = "always";
  };
}
