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
        junod-container-name = "junod";
        juno-indexer-container-name = "juno-indexer";
        subql-query-container-name = "subql-query";

        hasuraGraphqlPort = 8080;
        relaychainPort = 9944;
        parachainPort = 9988;
        subsquidParachainIndexerPort = 3000;
        subsquidIndexerGateway = 8081;
        subsquidIndexerStatusService = 60291;
        junoRpcPort = 26657;

        default-db = {
          name = "xcvm";
          host = "127.0.0.1";
          user = "xcvm";
          password = "xcvm";
          port = 1337;
        };

        juno-indexer-db-name = "juno";
        juno-indexer-db = default-db // {
          name = juno-indexer-db-name;
          host = db-container-name;
        };

        composable-indexer-db-name = "indexer";
        composable-indexer-db = default-db // {
          name = composable-indexer-db-name;
          host = db-container-name;
        };

        hasura-db-name = "hasura";
        hasura-db = default-db // {
          name = hasura-db-name;
          host = db-container-name;
        };

        network-name = "composable_xcvm_devnet";
        mk-composable-container = container:
          container // {
            service = container.service // { networks = [ network-name ]; };
          };
      in {
        config = {
          project.name = "composable_xcvm_devnet";
          networks."${network-name}" = { };
          services = {
            # ============ COMMON ===============
            "${db-container-name}" = mk-composable-container
              (import ./services/postgres.nix {
                inherit pkgs;
                database = default-db;
                version = "14";
                init-scripts = pkgs.writeTextFile {
                  name = "init";
                  text = ''
                    CREATE DATABASE ${juno-indexer-db-name} WITH OWNER ${default-db.user};
                    CREATE DATABASE ${composable-indexer-db-name} WITH OWNER ${default-db.user};
                    CREATE DATABASE ${hasura-db-name} WITH OWNER ${default-db.user};
                  '';
                  executable = false;
                  destination = "/init.sql";
                };
              });

            # ============== COSMOS ===============
            "${junod-container-name}" = mk-composable-container
              (import ./services/junod.nix { rpcPort = junoRpcPort; });
            "${juno-indexer-container-name}" = mk-composable-container
              (import ./services/juno-subql-indexer.nix {
                inherit pkgs;
                database = juno-indexer-db;
                juno = junod-container-name;
                junoPort = junoRpcPort;
              });
            "${subql-query-container-name}" = mk-composable-container
              (import ./services/subql-query.nix {
                database = juno-indexer-db;
                subql-node = juno-indexer-container-name;
                subqlPort = 3000;
              });
            hasura-aggregated = mk-composable-container
              (import ./services/hasura.nix {
                inherit pkgs;
                database = hasura-db;
                graphql-port = hasuraGraphqlPort;
                metadata = let
                  files = pkgs.linkFarm "metadata" [
                    {
                      name = "remote_schemas.yaml";
                      path = pkgs.writeText "remote_schemas.yaml" "[]";
                    }
                    {
                      name = "actions.graphql";
                      path = pkgs.writeText "actions.graphql" "";
                    }
                    {
                      name = "actions.yaml";
                      path = pkgs.writeText "actions.yaml" ''
                        actions: []
                        custom_types:
                          enums: []
                          input_objects: []
                          objects: []
                          scalars: []
                      '';
                    }
                    {
                      name = "allow_list.yaml";
                      path = pkgs.writeText "allow_list.yaml" "[]";
                    }
                    {
                      name = "cron_triggers.yaml";
                      path = pkgs.writeText "cron_triggers.yaml" "[]";
                    }
                    {
                      name = "query_collections.yaml";
                      path = pkgs.writeText "query_collections.yaml" "[]";
                    }
                    {
                      name = "version.yaml";
                      path = pkgs.writeText "version.yaml" "version: 3";
                    }
                    {
                      name = "rest_endpoints.yaml";
                      path = pkgs.writeText "rest_endpoints.yaml" "[]";
                    }
                    {
                      name = "databases";
                      path = pkgs.writeTextFile {
                        name = "databases.yaml";
                        text = ''
                          - name: cosmos
                            kind: postgres
                            configuration:
                              connection_info:
                                use_prepared_statements: false
                                database_url: postgres://${default-db.user}:${default-db.password}@${db-container-name}:${
                                  toString default-db.port
                                }/${juno-indexer-db-name}
                                isolation_level: read-committed
                            tables:
                              - table:
                                  schema: cosmos
                                  name: blocks
                              - table:
                                  schema: cosmos
                                  name: events
                              - table:
                                  schema: cosmos
                                  name: messages
                              - table:
                                  schema: cosmos
                                  name: transactions
                          - name: subsquid
                            kind: postgres
                            configuration:
                              connection_info:
                                use_prepared_statements: false
                                database_url: postgres://${default-db.user}:${default-db.password}@${db-container-name}:${
                                  toString default-db.port
                                }/${composable-indexer-db-name}
                                isolation_level: read-committed
                            tables:
                              - table:
                                  schema: public
                                  name: substrate_block
                              - table:
                                  schema: public
                                  name: substrate_extrinsic
                              - table:
                                  schema: public
                                  name: substrate_event
                        '';
                        executable = false;
                        destination = "/databases.yaml";
                      };
                    }
                  ];
                in pkgs.stdenv.mkDerivation {
                  # We can't use the above symlinked farm as the metadata store because it is mounted as a volume.
                  # Hence, it would be a folder full of dead link from the POV of the container.
                  name = "metadata";
                  phases = [ "installPhase" ];
                  installPhase = ''
                    mkdir $out
                    cp -rL ${files}/* $out
                  '';
                };
              });

            # ============== POLKADOT ==============
            "${dali-container-name}" = mk-composable-container
              (import ./services/devnet-dali.nix {
                inherit pkgs;
                inherit packages;
                inherit relaychainPort;
                inherit parachainPort;
              });
            subsquid-indexer = mk-composable-container
              (import ./services/subsquid-indexer.nix {
                database = composable-indexer-db;
                redis = redis-container-name;
                parachain = dali-container-name;
                inherit parachainPort;
                parachainIndexerPort = subsquidParachainIndexerPort;
              });
            "${subsquid-indexer-gateway-container-name}" =
              mk-composable-container
              (import ./services/subsquid-indexer-gateway.nix {
                database = composable-indexer-db;
                status = subsquid-status-container-name;
                graphql-port = subsquidIndexerGateway;
              });
            "${subsquid-status-container-name}" = mk-composable-container
              (import ./services/subsquid-indexer-status-service.nix {
                redis = redis-container-name;
                port = subsquidIndexerStatusService;
              });
            "${redis-container-name}" =
              mk-composable-container (import ./services/redis.nix);
          };
        };
      })
  ];
  inherit pkgs;
}
