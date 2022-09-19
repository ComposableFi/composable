{ pkgs, metadata, database, graphql-port }:
let
  context = let
    files = pkgs.linkFarm "context" [
      {
        name = "metadata";
        path = metadata;
      }
      {
        name = "Dockerfile";
        path = pkgs.writeText "Dockerfile" ''
          FROM hasura/graphql-engine:v2.12.0-beta.1.cli-migrations-v3
          COPY metadata /hasura-metadata
        '';
      }
    ];
  in pkgs.stdenv.mkDerivation {
    name = "context";
    phases = [ "installPhase" ];
    installPhase = ''
      mkdir $out
      cp -rL ${files}/* $out
    '';
  };
in {
  service = {
    build = { context = "${context}"; };
    restart = "always";
    environment = {
      HASURA_GRAPHQL_ENABLE_CONSOLE = "true";
      HASURA_GRAPHQL_DEV_MODE = "true";
      HASURA_GRAPHQL_CORS_DOMAIN = "*";
      HASURA_GRAPHQL_ENABLED_LOG_TYPES =
        "startup, http-log, webhook-log, websocket-log, query-log, action-handler-log, data-connector-log";
      HASURA_GRAPHQL_METADATA_DATABASE_URL =
        "postgres://${database.user}:${database.password}@${database.host}:${
          toString database.port
        }/${database.name}";
    };
    ports = [ "${toString graphql-port}:8080" ];
  };
}
