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


        squid-archive-db = {
          name = "squid-archive";
          host = db-container-name;
          user = "postgres";
          password = "postgres";
          port = 5432;
        };
        
        relaychainPort = 9944;
        parachainPort = 9988;
        # composable-squid-db-name = "composable_squid";
        # composable-squid-db = default-db // {
        #   name = composable-squid-db-name;
        #   host = db-container-name;
        # };

        # indexer-db-name = "indexer";
        # indexer-db = default-db // {
        #   name = indexer-db-name;
        #   host = db-container-name;
        # };

        # frontend-picasso = import ./services/frontend-picasso.nix {
        #   inherit pkgs;
        #   inherit packages;
        # };

        network-name = "composable_devnet";
        mkComposableContainer = container:
          container // {
            service = container.service // { networks = [ network-name ]; };
          };
      in
      {
        config = {
          project.name = "composable_firesquid";
          networks."${network-name}" = { };
          services = {
            "${db-container-name}" = mkComposableContainer
              (import ./services/postgres.nix {
                inherit pkgs;
                database = squid-archive-db;
                version = "14";
                init-scripts = pkgs.writeTextFile {
                  name = "init";
                  text = ''
                  '';
                  executable = false;
                  destination = "/init.sql";
                };
              });

             "${dali-container-name}" = mkComposableContainer
              (import ./services/devnet-dali.nix {
                inherit pkgs;
                inherit packages;
                inherit parachainPort;
                inherit relaychainPort;
              });

            ingest = mkComposableContainer (import ./services/subsquid-substrate-ingest.nix {
                database = squid-archive-db;
                polkadotEndpoint = "ws://${dali-container-name}:${toString parachainPort}";
                prometheusPort = 9090;
            }); 
            
            gateway = mkComposableContainer (import ./services/subsquid-substrate-gateway.nix {
                database = squid-archive-db;
                port = 8888;
            });
            explorer = mkComposableContainer (import ./services/subsquid-substrate-explorer.nix {
                database = squid-archive-db;
                graphqlPort = 4010;
           });
          };
        };
      })
  ];
  inherit pkgs;
}

