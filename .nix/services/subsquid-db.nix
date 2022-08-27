{
  service = {
    name = "postgres";
    image = "postgres:14";
    network_mode = "host";
    environment = {
      POSTGRES_USER = "postgres";
      POSTGRES_DB = "squid";
      POSTGRES_PASSWORD = "squid";
    };
    command = [ "-p" "23798" ];
  };
}
