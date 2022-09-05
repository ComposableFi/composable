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
          host = "127.0.0.1";
          user = "squid";
          password = "postgres";
          port = 1337;
        };
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
        mk-composable-container = container:
          container // {
            service = container.service // { networks = [ network-name ]; };
          };
      in
      {
        config = {
          project.name = "composable_firesquid";
          networks."${network-name}" = { };
          services = {
            "${db-container-name}" = mk-composable-container
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

            ingest = mk-composable-container {
              service = {
                # dependsOn = [ db-container-name ];
                restart = "on-failure";
                image = "subsquid/substrate-ingest:firesquid";
                command = [

                  # polkadot endpoints -- replace with your wss
                  "-e"
                  "ws://host.docker.internal:9988"
                  "-c"
                  "10" # allow up to 20 pending requests for the above endpoint (defa>
                  #  "--start-block", "1000000", # uncomment to specify a non-zero start blo>
                  "--prom-port"
                  "9090"
                  "--out"
                  "postgres://postgres:postgres@db:5432/squid-archive"
                ];
                ports = [ "9090:9090" ];
              };
            };
            
            gateway = mk-composable-container (import ./services/subsquid-substrate-gateway.nix);
            
            

          };
        };
      })
  ];
  inherit pkgs;
}

