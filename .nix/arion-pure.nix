{ pkgs, packages, ... }:
pkgs.arion.build {
  modules = [
    ({ pkgs, ... }:
      let
        db-container-name = "db";
        redis-container-name = "subsquid-redis";
        subsquid-status-container-name = "subsquid-status-service";
        subsquid-indexer-gateway-container-name = "subsquid-indexer-gateway";
        dali-container-name = "dali-devnet";

        default-db = {
          name = "postgres";
          host = "127.0.0.1";
          user = "squid";
          password = "squid";
          port = 1337;
        };
        composable-squid-db-name = "composable_squid";
        composable-squid-db = default-db // {
          name = composable-squid-db-name;
          host = db-container-name;
        };
        indexer-db-name = "indexer";
        indexer-db = default-db // {
          name = indexer-db-name;
          host = db-container-name;
        };
        network-name = "composable_devnet";
        mk-composable-container = container:
          container // {
            service = container.service // { networks = [ network-name ]; };
          };
      in {
        config = {
          project.name = "composable_devnet";
          networks."${network-name}" = { };
          services = {
            "${db-container-name}" = mk-composable-container
              (import ./services/postgres.nix {
                inherit pkgs;
                database = default-db;
                version = "14";
                init-scripts = pkgs.writeTextFile {
                  name = "init";
                  text = ''
                    CREATE DATABASE ${composable-squid-db-name} WITH OWNER ${default-db.user};
                    CREATE DATABASE ${indexer-db-name} WITH OWNER ${default-db.user};
                  '';
                  executable = false;
                  destination = "/init.sql";
                };
              });
            "${dali-container-name}" = mk-composable-container
              (import ./services/devnet-dali.nix {
                inherit pkgs;
                inherit packages;
                relaychain-port = 9944;
                parachain-port = 9988;
              });
            subsquid-indexer = mk-composable-container
              (import ./services/subsquid-indexer.nix {
                database = indexer-db;
                redis = redis-container-name;
                parachain = dali-container-name;
              });
            "${subsquid-indexer-gateway-container-name}" =
              mk-composable-container
              (import ./services/subsquid-indexer-gateway.nix {
                database = indexer-db;
                status = subsquid-status-container-name;
                graphql-port = 8080;
              });
            "${subsquid-status-container-name}" = mk-composable-container
              (import ./services/subsquid-indexer-status-service.nix {
                redis = redis-container-name;
              });
            "${redis-container-name}" =
              mk-composable-container (import ./services/redis.nix);
            subsquid-processor = mk-composable-container
              (import ./services/subsquid-processor.nix {
                inherit pkgs;
                inherit packages;
                database = composable-squid-db;
                redis = redis-container-name;
                relay = dali-container-name;
                archive = subsquid-indexer-gateway-container-name;
              });

            frontend-picasso = import ./services/frontend-picasso.nix {
              inherit pkgs;
              inherit packages;
            };
            frontend-pablo = import ./services/frontend-pablo.nix {
              inherit pkgs;
              inherit packages;
            };
          };
        };
      })
  ];
  inherit pkgs;
}

