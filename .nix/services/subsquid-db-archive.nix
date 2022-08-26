{
  service = {
    name = "postgres";
    image = "postgres:14";
    network_mode = "host";
    environment = {
      POSTGRES_USER = "postgres";
      POSTGRES_DB = "postgres";
      POSTGRES_PASSWORD = "postgres";
    };
  };
}
