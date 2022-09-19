{ pkgs, packages, ... }:
pkgs.arion.build {
  modules = [
    ({ pkgs, ... }:
      let
        dali-container-name = "dali-devnet";
        subsquidGraphqlContainerName = "subsquid-graphql";
        gatewayContainerName = "subsquid-gateway";

        # NOTE: Do not change this. It is hardcoded in the gateway source file.
        # cfr: services/subsquid-substrate-gateway
        gatewayPort = 8000;

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
        squidGraphqlPort = 4350;

        parachainEndpoint =
          "ws://${dali-container-name}:${toString parachainPort}";

        network-name = "composable_devnet";
        mkComposableContainer = container:
          container // {
            service = container.service // { networks = [ network-name ]; };
          };
      in {
        config = {
          project.name = "composable";
          networks."${network-name}" = { };
          services = {
            "${squid-archive-db.name}" = mkComposableContainer
              (import ../services/postgres.nix {
                inherit pkgs;
                database = squid-archive-db;
                version = "14";
                init-scripts = pkgs.writeTextFile {
                  name = "init";
                  text = "";
                  executable = false;
                  destination = "/init.sql";
                };
              });

            "${squid-db.name}" = mkComposableContainer
              (import ../services/postgres.nix {
                inherit pkgs;
                database = squid-db;
                version = "14";
                init-scripts = pkgs.writeTextFile {
                  name = "init";
                  text = "";
                  executable = false;
                  destination = "/init.sql";
                };
              });

            "${dali-container-name}" = mkComposableContainer
              (import ../services/devnet-dali.nix {
                inherit pkgs packages parachainPort relaychainPort;
              });

            ingest = mkComposableContainer
              (import ../services/subsquid-substrate-ingest.nix {
                database = squid-archive-db;
                polkadotEndpoint = parachainEndpoint;
                prometheusPort = 9090;
              });

            "${gatewayContainerName}" = mkComposableContainer
              (import ../services/subsquid-substrate-gateway.nix {
                database = squid-archive-db;
                port = gatewayPort;
              });

            # NOTE, this one currently seems broken. but it is an optional service anyways.
            # explorer = mkComposableContainer (import ../services/subsquid-substrate-explorer.nix {
            #     database = squid-archive-db;
            #     graphqlPort = 4010;
            # });

            "${subsquidGraphqlContainerName}" = mkComposableContainer
              (import ../services/subsquid-graphql.nix {
                inherit pkgs;
                database = squid-db;
                graphqlPort = squidGraphqlPort;
              });

            subsquid-processor = mkComposableContainer
              (import ../services/subsquid-processor-dockerfile.nix {
                inherit subsquidGraphqlContainerName gatewayContainerName
                  gatewayPort parachainEndpoint;
                database = squid-db;
                graphqlPort = squidGraphqlPort;
              });

            # NOTE: Ports are currently not configurable for frontend services
            frontend-picasso = mkComposableContainer
              (import ../services/frontend-picasso.nix {
                inherit pkgs packages;
              });

            frontend-pablo = mkComposableContainer
              (import ../services/frontend-pablo.nix { inherit pkgs packages; });
          };
        };
      })
  ];
  inherit pkgs;
}

