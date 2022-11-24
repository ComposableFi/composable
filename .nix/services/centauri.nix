{  }: {
  service = {
    name = "hyperspace";
    image = "composablefi/composable-centauri:d8fdb31227a221a794f86c5bba4a8127cf8e71d5";
    restart = "always";
    # environment = {
    #   DEV_MODE = "true";
    #   DB_NAME = database.name;
    #   DB_HOST = database.host;
    #   DB_USER = database.user;
    #   DB_PASS = database.password;
    #   DB_PORT = database.port;
    # };
    # ports = [ "${toString graphql-port}:8080" ];
  };
}
