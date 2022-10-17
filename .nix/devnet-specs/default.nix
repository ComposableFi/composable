{ pkgs, price-feed, devnet, frontend, ... }: {
  modules = [
    (let
      price-feed-container-name = "price-feed";
      devnet-container-name = "devnet";
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

      rococoPort = 9944;
      daliAlicePort = 9988;
      daliBobPort = 9989;
      daliCharliePort = 9990;
      karuraPort = 9999;
      stateminePort = 10008;
      squidGraphqlPort = 4350;
      frontendPabloPort = 8001;
      frontendPicassoPort = 8002;
      priceFeedPort = 8003;

      daliEndpoint = "ws://${devnet-container-name}:${toString daliAlicePort}";

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

          "${devnet-container-name}" = mkComposableContainer
            (import ../services/devnet.nix {
              inherit pkgs;
              inherit devnet;
              ports = [
                {
                  host = rococoPort;
                  container = rococoPort;
                }
                {
                  host = daliAlicePort;
                  container = daliAlicePort;
                }
                {
                  host = daliBobPort;
                  container = daliBobPort;
                }
                {
                  host = daliCharliePort;
                  container = daliCharliePort;
                }
                {
                  host = karuraPort;
                  container = karuraPort;
                }
                {
                  host = stateminePort;
                  container = stateminePort;
                }
              ];
            });

          ingest = mkComposableContainer
            (import ../services/subsquid-substrate-ingest.nix {
              database = squid-archive-db;
              polkadotEndpoint = daliEndpoint;
              prometheusPort = 9090;
            });

          "${gatewayContainerName}" = mkComposableContainer
            (import ../services/subsquid-substrate-gateway.nix {
              database = squid-archive-db;
            });

          # NOTE, this one currently seems broken. but it is an optional service anyways.
          # explorer = mkComposableContainer (import ../services/subsquid-substrate-explorer.nix {
          #     database = squid-archive-db;
          #     graphqlPort = 4010;
          # });

          "${subsquidGraphqlContainerName}" = mkComposableContainer
            (import ../services/subsquid-graphql.nix {
              database = squid-db;
              graphqlPort = squidGraphqlPort;
            });

          subsquid-processor = mkComposableContainer
            (import ../services/subsquid-processor-dockerfile.nix {
              inherit subsquidGraphqlContainerName gatewayContainerName
                gatewayPort;
              parachainEndpoint = daliEndpoint;
              database = squid-db;
              graphqlPort = squidGraphqlPort;
            });

          # NOTE: Ports are currently not configurable for frontend services
          frontend-picasso = mkComposableContainer
            (import ../services/composable-frontend.nix {
              inherit pkgs frontend;
              app = "pablo";
              port = frontendPabloPort;
            });

          frontend-pablo = mkComposableContainer
            (import ../services/composable-frontend.nix {
              inherit pkgs frontend;
              app = "picasso";
              port = frontendPicassoPort;
            });

          "${price-feed-container-name}" = mkComposableContainer
            (import ../services/program.nix {
              inherit pkgs;
              program = price-feed;
              environment = { RUST_LOG = "debug"; };
              command = "${
                  pkgs.lib.meta.getExe price-feed
                } --quote-asset USDT --listening-address 0.0.0.0:${
                  toString priceFeedPort
                } --composable-node ws://${devnet-container-name}:${
                  toString daliAlicePort
                }";
              ports = [{
                host = priceFeedPort;
                container = priceFeedPort;
              }];
            });
        };
      };
    })
  ];
  inherit pkgs;
}

