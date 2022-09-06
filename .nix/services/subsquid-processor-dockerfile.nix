{ database, subsquidGraphqlContainerName, parachainEndpoint, graphqlPort
, gatewayContainerName, gatewayPort, ... }: {
  service = {
    build.context = "${../../subsquid}";
    depends_on = {
      "${database.name}" = { condition = "service_healthy"; };
      "${subsquidGraphqlContainerName}" = { condition = "service_started"; };
    };
    environment = {
      DB_NAME = database.name;
      DB_HOST = database.host;
      DB_USER = database.user;
      DB_PASS = database.password;
      DB_PORT = database.port;
      GQL_PORT = graphqlPort;
      # is actually the parachain URI, bug in ts source
      RELAYCHAIN_URI = parachainEndpoint;
      SUBSQUID_ARCHIVE_URI =
        "http://${gatewayContainerName}:${toString gatewayPort}/graphql";
    };
  };

  # Unfortunately, arion does not model this field yet.
  # We can add a PR that adds it right below this option 
  # https://github.com/hercules-ci/arion/blob/e5fb978143240f8d293e6e5acc9691acf472928d/src/nix/modules/service/docker-compose-service.nix#L66  
  out.service.build.dockerfile = "Dockerfile";
}
