{ metadata, database, graphql-port }: {
  service = {
    image = "hasura/graphql-engine:v2.12.0-beta.1.cli-migrations-v3";
    restart = "always";
    environment = {
      HASURA_GRAPHQL_ENABLE_CONSOLE = "true";
      HASURA_GRAPHQL_DEV_MODE = "true";
      HASURA_GRAPHQL_ENABLED_LOG_TYPES =
        "startup, http-log, webhook-log, websocket-log, query-log, action-handler-log, data-connector-log";
      HASURA_GRAPHQL_METADATA_DATABASE_URL =
        "postgres://${database.user}:${database.password}@${database.host}:${
          toString database.port
        }/${database.name}";
    };
    volumes = [ "${metadata}/:/hasura-metadata" ];
    ports = [ "${toString graphql-port}:8080" ];
  };
}
