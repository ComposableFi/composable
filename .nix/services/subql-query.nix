{ database, subql-node, subqlPort }: {
  service = {
    image = "onfinality/subql-query:v1.5.0";
    restart = "always";
    environment = {
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_DATABASE = database.name;
      DB_HOST = database.host;
      DB_PORT = database.port;
    };
    # as of today it is seems cors * by default
    command =
      [ "--name=cosmos" "--playground" "--indexer=http://${subql-node}:3000" ];
    ports = [ "${toString subqlPort}:3000" ];
  };
}
