{ pkgs, packages, ... }:
pkgs.arion.build {
  modules = [
    ({ pkgs, ... }:
      let
        default-db = {
          name = "xcvm";
          host = "127.0.0.1";
          user = "xcvm";
          password = "xcvm";
          port = 1337;
        };
        juno-db-name = "juno";
        juno-db = default-db // { name = juno-db-name; };
        squid-db-name = "squid";
        squid-db = default-db // { name = squid-db-name; };
      in {
        config.project.name = "Composable Finance XCVM devnet";
        config.services = {

          # ============ COMMON ===============
          db = import ./services/postgres.nix {
            database = default-db;
            version = "14";
            init-scripts = pkgs.writeTextFile {
              name = "init";
              text = ''
                CREATE DATABASE ${juno-db-name} WITH OWNER ${default-db.user};
                CREATE DATABASE ${squid-db-name} WITH OWNER ${default-db.user};
              '';
              executable = false;
              destination = "/init.sql";
            };
          };

          # ============== COSMOS ===============
          junod = import ./services/junod.nix;
          juno-subql-indexer = import ./services/juno-subql.nix {
            inherit pkgs;
            database = juno-db;
          };

          # ============== POLKADOT ==============
          devnet-dali = import ./services/devnet-dali.nix {
            inherit pkgs;
            inherit packages;
          };
          subsquid-indexer =
            import ./services/subsquid-indexer.nix { database = squid-db; };
          subsquid-indexer-gateway =
            import ./services/subsquid-indexer-gateway.nix {
              database = squid-db;
            };
          subsquid-indexer-status-service =
            import ./services/subsquid-indexer-status-service.nix;
          subsquid-redis = import ./services/subsquid-redis.nix;
          subsquid-processor = import ./services/subsquid-processor.nix {
            inherit pkgs;
            inherit packages;
            database = squid-db;
          };
        };
      })
  ];
  inherit pkgs;
}
