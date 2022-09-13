{ database, graphqlPort }: {
  service = {
    name = "subsquid-substrate-explorer";
    image = "subsquid/substrate-explorer:firesquid";
    restart = "always";
    environment = {
      DEV_MODE = "true";
      DB_TYPE = "postgres";
      DB_NAME = database.name;
      DB_HOST = database.host;
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_PORT = database.port;
    };
    ports = [ "${toString graphqlPort}:3000" ];
  };
}
