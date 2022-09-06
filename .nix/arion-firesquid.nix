{ pkgs, packages, ... }:
pkgs.arion.build {
  modules = [
    ({ pkgs, ... }:
      let

        # subsquid-status-container-name = "subsquid-status-service";
        # subsquid-indexer-gateway-container-name = "subsquid-indexer-gateway";

        dali-container-name = "dali-devnet";


        squid-archive-db = rec {
          name = "squid-archive-db";
          host = name;
          user = "squid-archive-user";
          password = "super-secret-squid-archive-pass";
          port = 5432;
        };

        squid-db = rec {
          name = "squid-db";
          host = name;
          user = "squid-user";
          password = "super-secret-squid-pass";
          port = 5433;
        };
        
        relaychainPort = 9944;
        parachainPort = 9988;

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
            "${squid-archive-db.name}" = mkComposableContainer
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

            "${squid-db.name}" = mkComposableContainer
              (import ./services/postgres.nix {
                inherit pkgs;
                database = squid-db;
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

            # note, this one currently seems broken.
            # explorer = mkComposableContainer (import ./services/subsquid-substrate-explorer.nix {
            #     database = squid-archive-db;
            #     graphqlPort = 4010;
            # });
          };
        };
      })
  ];
  inherit pkgs;
}

