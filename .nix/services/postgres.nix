{ version, database, init-scripts }: {
  service = {
    name = "postgres";
    image = "postgres:${version}";
    network_mode = "host";
    environment = {
      POSTGRES_USER = database.user;
      POSTGRES_DB = database.name;
      POSTGRES_PASSWORD = database.password;
    };
    command = [ "-p" "${toString database.port}" ];
    volumes = [ "${init-scripts}:/docker-entrypoint-initdb.d/" ];
  };
}
