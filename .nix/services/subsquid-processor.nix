{ pkgs, database, packages, ... }: {
  image = {
    contents = [ pkgs.coreutils pkgs.python3 packages.subsquid-processor ];
    enableRecommendedContents = true;
  };
  service = {
    restart = "always";
    network_mode = "host";
    command = [
      "sh"
      "-c"
      ''
        ${packages.subsquid-processor}/bin/run-subsquid-processor
      ''
    ];
    environment = {
      DB_NAME = database.name;
      DB_HOST = database.host;
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_PORT = database.port;
    };
  };
}
