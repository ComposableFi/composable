{ pkgs, archive, relay, database, packages, ... }: {
  image = {
    contents = [ pkgs.coreutils pkgs.python3 packages.subsquid-processor ];
    enableRecommendedContents = true;
  };
  service = {
    restart = "always";
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
      SUBSQUID_ARCHIVE_URI = "http://${archive}:8080/v1/graphql";
      RELAYCHAIN_URI = "ws://${relay}:9988";
    };
  };
}
