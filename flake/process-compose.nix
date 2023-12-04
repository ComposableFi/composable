{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }:
    let
      networks = pkgs.networksLib;
      devnet-root-directory = "/tmp/composable-devnet";
      validator-key = "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj";
      relay = "no"; # `no` not to restart, `on_failure` for
      chain-restart = "no"; # `no` not to restart, `on_failure` for
      parachain-startup = {
        initial_delay_seconds = 32;
        period_seconds = 8;
        failure_threshold = 16;
        timeout_seconds = 4;
      };
    in {

      packages = rec {

        process-compose-stop = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ pkgs.process-compose ];
          name = "process-compose-stop";
          text = ''
            pkill -SIGTERM process-compose
          '';
        };

        devnet-xc-fresh-background = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools;
          name = "devnet-xc-fresh-background";
          text = ''
            rm --force --recursive ${devnet-root-directory}             
            mkdir --parents ${devnet-root-directory}
            ${pkgs.lib.meta.getExe self'.packages.devnet-xc-background}
          '';
        };

        devnet-xc-dotsama-fresh-background = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools;
          name = "devnet-xc-dotsama-fresh-background";
          text = ''
            rm --force --recursive ${devnet-root-directory}             
            mkdir --parents ${devnet-root-directory}
            ${pkgs.lib.meta.getExe self'.packages.devnet-xc-dotsama-background}
          '';
        };

        devnet-xc-dotsama-fresh = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools;
          name = "devnet-xc-dotsama-fresh";
          text = ''
            rm --force --recursive ${devnet-root-directory}             
            mkdir --parents ${devnet-root-directory}
            ${pkgs.lib.meta.getExe self'.packages.devnet-xc-dotsama}
          '';
        };

        prune-run-unsafe = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools
            ++ [ pkgs.process-compose ];
          name = "prune-run-unsafe";
          text = ''
            process-compose-stop() {
              for i in $(process-compose process list)
              do
                process-compose process stop "$i"
              done
            }
            process-compose-stop
            pkill composable centaurid osmosisd hyperspace hermes
          '';
        };

        devnet-xc-clean = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools;
          name = "devnet-xc-clean";
          text = ''
            rm --force --recursive ${devnet-root-directory}             
            mkdir --parents ${devnet-root-directory}            
          '';
        };

        devnet-cosmos-fresh = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools;
          name = "devnet-cosmos-fresh";
          text = ''
            rm --force --recursive ${devnet-root-directory}             
            mkdir --parents ${devnet-root-directory}
            ${pkgs.lib.meta.getExe self'.packages.devnet-cosmos}
          '';
        };

        devnet-cosmos-fresh-background = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools;
          name = "devnet-cosmos-fresh-background";
          text = ''
            rm --force --recursive ${devnet-root-directory}             
            mkdir --parents ${devnet-root-directory}
            ${pkgs.lib.meta.getExe self'.packages.devnet-cosmos-background}
          '';
        };

        devnet-xc-fresh = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools;
          name = "devnet-xc-fresh";
          text = ''
            rm --force --recursive ${devnet-root-directory}             
            mkdir --parents ${devnet-root-directory}
            ${pkgs.lib.meta.getExe self'.packages.devnet-xc}
          '';
        };
      };
      process-compose = rec {
        devnet-xc-background = devnet-xc // { tui = false; };
        devnet-xc-dotsama-background = devnet-xc-dotsama // { tui = false; };
        devnet-xc-dotsama = {
          settings = {
            log_level = "trace";
            log_location = "/tmp/composable-devnet/pc.log";
            processes = {
              picasso = {
                command = self'.packages.zombienet-rococo-local-picasso-dev;
                availability = { restart = chain-restart; };
                log_location = "${devnet-root-directory}/picasso.log";
                readiness_probe = {
                  exec.command = ''
                    curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://localhost:9988
                  '';
                } // parachain-startup;
              };
              composable = {
                command = self'.packages.zombienet-composable-westend-b;
                availability = { restart = chain-restart; };
                log_location = "${devnet-root-directory}/composable.log";
                readiness_probe = {
                  exec.command = ''
                    curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://localhost:29988
                  '';
                } // parachain-startup;
              };

              composable-picasso-ibc-init = {
                command = self'.packages.composable-picasso-ibc-init;
                log_location =
                  "${devnet-root-directory}/composable-picasso-ibc-init.log";
                depends_on = {
                  "composable".condition = "process_healthy";
                  "picasso".condition = "process_healthy";
                };
                availability = { restart = relay; };
              };
              composable-picasso-ibc-connection-init = {
                command = ''
                  HOME="${devnet-root-directory}/composable-picasso-ibc"
                  export HOME                
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG      
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-connection --config-a ${devnet-root-directory}/composable-picasso-ibc/config-chain-a.toml --config-b ${devnet-root-directory}/composable-picasso-ibc/config-chain-b.toml --config-core ${devnet-root-directory}/composable-picasso-ibc/config-core.toml --delay-period 10
                '';
                log_location =
                  "${devnet-root-directory}/composable-picasso-ibc-connection-init.log";
                depends_on = {
                  "composable-picasso-ibc-init".condition =
                    "process_completed_successfully";
                  "composable".condition = "process_healthy";
                };
                availability = { restart = relay; };
              };

              composable-picasso-ibc-channels-init = {
                command = ''
                  HOME="${devnet-root-directory}/composable-picasso-ibc"
                  export HOME       
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-channel --config-a ${devnet-root-directory}/composable-picasso-ibc/config-chain-a.toml --config-b ${devnet-root-directory}/composable-picasso-ibc/config-chain-b.toml --config-core ${devnet-root-directory}/composable-picasso-ibc/config-core.toml --delay-period 10 --port-id transfer --version ics20-1 --order unordered
                '';
                log_location =
                  "${devnet-root-directory}/composable-picasso-ibc-channels-init.log";
                depends_on = {
                  "composable-picasso-ibc-connection-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };
              composable-picasso-ibc-relay = {
                command = self'.packages.composable-picasso-ibc-relay;
                log_location =
                  "${devnet-root-directory}/composable-picasso-ibc-relay.log";
                depends_on = {
                  "composable-picasso-ibc-channels-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };
            };
          };
        };

        devnet-xc = {
          settings = {
            log_level = "trace";
            log_location = "/tmp/composable-devnet/pc.log";
            processes = {
              eth-gen = {
                command = self'.packages.eth-gen;
                log_location = "${devnet-root-directory}/eth-gen.log";
                availability = { restart = chain-restart; };
                namespace = "ethereum";
              };
              eth-consensus-gen = {
                command = self'.packages.eth-consensus-gen;
                log_location = "${devnet-root-directory}/eth-consensus-gen.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-gen".condition = "process_completed_successfully";
                };
                namespace = "ethereum";
              };
              eth-executor-gen = {
                command = self'.packages.eth-executor-gen;
                log_location = "${devnet-root-directory}/eth-executor-gen.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-gen".condition = "process_completed_successfully";
                };
                namespace = "ethereum";
              };
              eth-executor = {
                command = self'.packages.eth-executor;
                log_location = "${devnet-root-directory}/eth-executor.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-executor-gen".condition =
                    "process_completed_successfully";
                };
                readiness_probe = {
                  exec.command = ''
                    test -f ${devnet-root-directory}/eth/jwtsecret
                  '';
                } // parachain-startup;
                namespace = "ethereum";
              };
              eth-consensus = {
                command = self'.packages.eth-consensus;
                log_location = "${devnet-root-directory}/eth-consensus.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-consensus-gen".condition =
                    "process_completed_successfully";
                  "eth-executor".condition = "process_healthy";
                };
                namespace = "ethereum";
              };
              eth-validator = {
                command = self'.packages.eth-validator;
                log_location = "${devnet-root-directory}/eth-validator.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-consensus-gen".condition =
                    "process_completed_successfully";
                };
                namespace = "ethereum";
              };
              centauri = {
                command = pkgs.writeShellApplication {
                  runtimeInputs = devnetTools.withBaseContainerTools;
                  name = "centauri";
                  text = ''
                    ${pkgs.lib.meta.getExe self'.packages.centaurid-gen} reuse 0
                  '';
                };
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = 26657;
                };
                log_location = "${devnet-root-directory}/centauri.log";
                availability = { restart = chain-restart; };
                namespace = "cosmos";
              };

              centauri-init = {
                command = self'.packages.centaurid-init;
                depends_on."centauri".condition = "process_healthy";
                log_location = "${devnet-root-directory}/centauri-init.log";
                availability = { restart = chain-restart; };
                namespace = "cosmos";
              };

              osmosis = {
                command = self'.packages.osmosisd-gen;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = pkgs.networksLib.osmosis.devnet.PORT;
                };
                log_location = "${devnet-root-directory}/osmosis.log";
                availability = { restart = chain-restart; };
                namespace = "cosmos";
              };
              osmosisd-cvm-init = {
                command = self'.packages.osmosisd-cvm-init;
                depends_on."osmosis".condition = "process_healthy";
                log_location = "${devnet-root-directory}/osmosisd-cvm-init.log";
                availability = { restart = chain-restart; };
                namespace = "cosmos";
              };

              picasso = {
                command = self'.packages.zombienet-rococo-local-picasso-dev;
                availability = { restart = chain-restart; };
                log_location = "${devnet-root-directory}/picasso.log";
                readiness_probe = {
                  exec.command = ''
                    curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://localhost:9988
                  '';
                } // parachain-startup;
                namespace = "polkadot";
              };
              composable = {
                command = self'.packages.zombienet-composable-westend-b;
                availability = { restart = chain-restart; };
                log_location = "${devnet-root-directory}/composable.log";
                readiness_probe = {
                  exec.command = ''
                    curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://localhost:29988
                  '';
                } // parachain-startup;
                namespace = "polkadot";
              };
              osmosis-centauri-hermes-init = {
                command = self'.packages.osmosis-centauri-hermes-init;
                depends_on = {
                  "centauri-init".condition = "process_completed_successfully";
                  "picasso-centauri-ibc-channels-init".condition =
                    "process_completed_successfully";
                  "osmosis".condition = "process_healthy";
                };
                log_location =
                  "${devnet-root-directory}/osmosis-centauri-hermes-init.log";
                availability = { restart = relay; };
              };

              osmosis-centauri-hermes-relay = {
                command = self'.packages.osmosis-centauri-hermes-relay;
                depends_on = {
                  "osmosis-centauri-hermes-init".condition =
                    "process_completed_successfully";
                };
                log_location =
                  "${devnet-root-directory}/osmosis-centauri-hermes-relay.log";
                availability = { restart = relay; };
              };

              picasso-centauri-ibc-init = {
                command = self'.packages.picasso-centauri-ibc-init;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-init.log";
                depends_on = {
                  "centauri-init".condition = "process_completed_successfully";
                  "centauri".condition = "process_healthy";
                  "picasso".condition = "process_healthy";
                };
                availability = { restart = relay; };
              };

              picasso-centauri-ibc-connection-init = {
                command = self'.packages.picasso-centauri-ibc-connection-init;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-connection-init.log";
                depends_on = {
                  "picasso-centauri-ibc-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };

              picasso-centauri-ibc-channels-init = {
                command = self'.packages.picasso-centauri-ibc-channels-init;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-channels-init.log";
                depends_on = {
                  "picasso-centauri-ibc-connection-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };

              picasso-centauri-ibc-relay = {
                command = self'.packages.picasso-centauri-ibc-relay;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-relay.log";
                depends_on = {
                  "picasso-centauri-ibc-channels-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };

              composable-picasso-ibc-init = {
                command = self'.packages.composable-picasso-ibc-init;
                log_location =
                  "${devnet-root-directory}/composable-picasso-ibc-init.log";
                depends_on = {
                  "picasso-centauri-ibc-channels-init".condition =
                    "process_completed_successfully";
                  "composable".condition = "process_healthy";
                  "picasso".condition = "process_healthy";
                };
                availability = { restart = relay; };
              };
              composable-picasso-ibc-connection-init = {
                command = ''
                  HOME="${devnet-root-directory}/composable-picasso-ibc"
                  export HOME                
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG      
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-connection --config-a ${devnet-root-directory}/composable-picasso-ibc/config-chain-a.toml --config-b ${devnet-root-directory}/composable-picasso-ibc/config-chain-b.toml --config-core ${devnet-root-directory}/composable-picasso-ibc/config-core.toml --delay-period 10
                '';
                log_location =
                  "${devnet-root-directory}/composable-picasso-ibc-connection-init.log";
                depends_on = {
                  "composable-picasso-ibc-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };

              composable-picasso-ibc-channels-init = {
                command = ''
                  HOME="${devnet-root-directory}/composable-picasso-ibc"
                  export HOME       
                  RUST_LOG="hyperspace=info,hyperspace_parachain=debug,hyperspace_cosmos=debug"
                  export RUST_LOG
                  ${self'.packages.hyperspace-composable-rococo-picasso-rococo}/bin/hyperspace create-channel --config-a ${devnet-root-directory}/composable-picasso-ibc/config-chain-a.toml --config-b ${devnet-root-directory}/composable-picasso-ibc/config-chain-b.toml --config-core ${devnet-root-directory}/composable-picasso-ibc/config-core.toml --delay-period 10 --port-id transfer --version ics20-1 --order unordered
                '';
                log_location =
                  "${devnet-root-directory}/composable-picasso-ibc-channels-init.log";
                depends_on = {
                  "composable-picasso-ibc-connection-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };
              composable-picasso-ibc-relay = {
                command = self'.packages.composable-picasso-ibc-relay;
                log_location =
                  "${devnet-root-directory}/composable-picasso-ibc-relay.log";
                depends_on = {
                  "composable-picasso-ibc-channels-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };
            };
          };
        };

        devnet-xc-picasso-cosmos = {
          settings = {
            log_level = "trace";
            log_location = "/tmp/composable-devnet/pc.log";
            processes = {
              centauri = {
                command = pkgs.writeShellApplication {
                  runtimeInputs = devnetTools.withBaseContainerTools;
                  name = "centauri";
                  text = ''
                    ${pkgs.lib.meta.getExe self'.packages.centaurid-gen} reuse 0
                  '';
                };
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = 26657;
                };
                log_location = "${devnet-root-directory}/centauri.log";
                availability = { restart = chain-restart; };
                namespace = "cosmos";
              };
              centauri-init = {
                command = self'.packages.centaurid-init;
                depends_on."centauri".condition = "process_healthy";
                log_location = "${devnet-root-directory}/centauri-init.log";
                availability = { restart = chain-restart; };
                namespace = "cosmos";
              };

              osmosis = {
                command = self'.packages.osmosisd-gen;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = pkgs.networksLib.osmosis.devnet.PORT;
                };
                log_location = "${devnet-root-directory}/osmosis.log";
                availability = { restart = chain-restart; };
                namespace = "cosmos";
              };
              osmosisd-cvm-init = {
                command = self'.packages.osmosisd-cvm-init;
                depends_on."osmosis".condition = "process_healthy";
                log_location = "${devnet-root-directory}/osmosisd-cvm-init.log";
                availability = { restart = chain-restart; };
                namespace = "cosmos";
              };

              picasso = {
                command = self'.packages.zombienet-rococo-local-picasso-dev;
                availability = { restart = chain-restart; };
                log_location = "${devnet-root-directory}/picasso.log";
                readiness_probe = {
                  exec.command = ''
                    curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://localhost:9988
                  '';
                } // parachain-startup;
                namespace = "polkadot";
              };

              osmosis-centauri-hermes-init = {
                command = self'.packages.osmosis-centauri-hermes-init;
                depends_on = {
                  "centauri-init".condition = "process_completed_successfully";
                  "picasso-centauri-ibc-channels-init".condition =
                    "process_completed_successfully";
                  "osmosis".condition = "process_healthy";
                };
                log_location =
                  "${devnet-root-directory}/osmosis-centauri-hermes-init.log";
                availability = { restart = relay; };
              };

              osmosis-centauri-hermes-relay = {
                command = self'.packages.osmosis-centauri-hermes-relay;
                depends_on = {
                  "osmosis-centauri-hermes-init".condition =
                    "process_completed_successfully";
                };
                log_location =
                  "${devnet-root-directory}/osmosis-centauri-hermes-relay.log";
                availability = { restart = relay; };
              };

              picasso-centauri-ibc-init = {
                command = self'.packages.picasso-centauri-ibc-init;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-init.log";
                depends_on = {
                  "centauri-init".condition = "process_completed_successfully";
                  "centauri".condition = "process_healthy";
                  "picasso".condition = "process_healthy";
                };
                availability = { restart = relay; };
              };

              picasso-centauri-ibc-connection-init = {
                command = self'.packages.picasso-centauri-ibc-connection-init;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-connection-init.log";
                depends_on = {
                  "picasso-centauri-ibc-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };

              picasso-centauri-ibc-channels-init = {
                command = self'.packages.picasso-centauri-ibc-channels-init;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-channels-init.log";
                depends_on = {
                  "picasso-centauri-ibc-connection-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };

              picasso-centauri-ibc-relay = {
                command = self'.packages.picasso-centauri-ibc-relay;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-relay.log";
                depends_on = {
                  "picasso-centauri-ibc-channels-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = relay; };
              };

            };
          };
        };

        devnet-cosmos-background = devnet-cosmos // { tui = false; };

        devnet-cosmos = {
          settings = {
            log_level = "trace";
            log_location = "/tmp/composable-devnet/pc.log";
            processes = {
              centauri = {
                command = self'.packages.centaurid-gen;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = 26657;
                };
                log_location = "${devnet-root-directory}/centauri.log";
                availability = { restart = chain-restart; };
              };

              neutron-init = {
                command = self'.packages.neutron-gen;
                log_location = "${devnet-root-directory}/neutron-init.log";
                availability = { restart = chain-restart; };
              };

              neutron = {
                command = self'.packages.neutron-start;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = networks.neutron.devnet.PORT;
                };
                log_location = "${devnet-root-directory}/neutron-start.log";
                availability = { restart = chain-restart; };
                depends_on."neutron-init".condition =
                  "process_completed_successfully";
              };

              neutron-centauri-init = {
                command = self'.packages.neutron-centauri-hermes-init;
                log_location =
                  "${devnet-root-directory}/neutron-centauri-init.log";
                availability = { restart = relay; };
                depends_on."neutron".condition = "process_healthy";
                depends_on."osmosis-centauri-hermes-init".condition =
                  "process_completed_successfully";
                namespace = "ibc";
              };

              centauri-neutron-hermes-relay = {
                command = self'.packages.centauri-neutron-hermes-relay;
                log_location =
                  "${devnet-root-directory}/neutron-centauri-relay.log";
                availability = { restart = relay; };
                depends_on."neutron".condition = "process_healthy";
                depends_on."neutron-centauri-init".condition =
                  "process_completed_successfully";
                namespace = "ibc";
              };

              centauri-init = {
                command = self'.packages.centaurid-init;
                depends_on."centauri".condition = "process_healthy";
                log_location = "${devnet-root-directory}/centauri-init.log";
                availability = { restart = chain-restart; };
              };

              centauri-cvm-init = {
                command = self'.packages.centaurid-cvm-init;
                depends_on."centauri".condition = "process_healthy";
                log_location = "${devnet-root-directory}/centauri-cvm-init.log";
                availability = { restart = chain-restart; };
              };

              centauri-cvm-config = {
                command = self'.packages.centaurid-cvm-config;
                depends_on."centauri-cvm-init".condition =
                  "process_completed_successfully";
                depends_on."osmosis-cvm-init".condition =
                  "process_completed_successfully";
                log_location =
                  "${devnet-root-directory}/centauri-cvm-config.log";
                availability = { restart = chain-restart; };
              };

              osmosis-cvm-config = {
                command = self'.packages.osmosisd-cvm-config;
                depends_on."centauri-cvm-init".condition =
                  "process_completed_successfully";
                depends_on."osmosis-cvm-init".condition =
                  "process_completed_successfully";
                log_location =
                  "${devnet-root-directory}/osmosis-cvm-config.log";
                availability = { restart = chain-restart; };
              };

              osmosis = {
                command = self'.packages.osmosisd-gen;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = pkgs.networksLib.osmosis.devnet.PORT;
                };
                log_location = "${devnet-root-directory}/osmosis.log";
              };
              osmosis-pools-init = {
                command = self'.packages.osmosisd-pools-init;
                depends_on."osmosis".condition = "process_healthy";
                log_location =
                  "${devnet-root-directory}/osmosisd-pools-init.log";
                availability = { restart = chain-restart; };
              };
              osmosis-cvm-init = {
                command = self'.packages.osmosisd-cvm-init;
                depends_on."osmosis".condition = "process_healthy";
                log_location = "${devnet-root-directory}/osmosis-cvm-init.log";
                availability = { restart = chain-restart; };
                namespace = "osmosis";
              };

              osmosis-centauri-hermes-init = {
                command = self'.packages.osmosis-centauri-hermes-init;
                depends_on = {
                  "centauri-init".condition = "process_completed_successfully";
                  "osmosis".condition = "process_healthy";
                };
                namespace = "ibc";
                log_location =
                  "${devnet-root-directory}/osmosis-centauri-hermes-init.log";
                availability = { restart = relay; };
              };

              osmosis-centauri-hermes-relay = {
                command = self'.packages.osmosis-centauri-hermes-relay;
                depends_on = {
                  "osmosis-centauri-hermes-init".condition =
                    "process_completed_successfully";
                };
                log_location =
                  "${devnet-root-directory}/osmosis-centauri-hermes-relay.log";
                availability = { restart = relay; };
                namespace = "ibc";
              };

              mantis-simulate-solve = {
                command = self'.packages.mantis-simulate-solve;
                depends_on = {
                  "centauri-cvm-config".condition =
                    "process_completed_successfully";
                  "osmosis-centauri-hermes-init".condition =
                    "process_completed_successfully";
                };
                log_location =
                  "${devnet-root-directory}/mantis-simulate-solve.log";
                availability = { restart = relay; };
                namespace = "xapp";
              };

              xapp-osmosis-osmo-to-centauri = {
                command = self'.packages.xapp-osmosis-osmo-to-centauri;
                depends_on = {
                  "osmosis-centauri-hermes-init".condition =
                    "process_completed_successfully";
                };
                log_location =
                  "${devnet-root-directory}/xapp-osmosis-osmo-to-centauri.log";
                namespace = "xapp";
              };

              xapp-centauri-pica-to-osmosis = {
                command = self'.packages.xapp-centauri-pica-to-osmosis;
                depends_on = {
                  "osmosis-centauri-hermes-init".condition =
                    "process_completed_successfully";
                };
                log_location =
                  "${devnet-root-directory}/xapp-centauri-pica-to-osmosis.log";
                namespace = "xapp";
              };

              xapp-swap-centauri-osmo-to-osmosis-pica-and-back = {
                command =
                  self'.packages.xapp-swap-centauri-osmo-to-osmosis-pica-and-back;
                depends_on = {
                  "xapp-centauri-pica-to-osmosis".condition =
                    "process_completed_successfully";
                  "xapp-osmosis-osmo-to-centauri".condition =
                    "process_completed_successfully";
                };
                log_location =
                  "${devnet-root-directory}/xapp-swap-centauri-osmo-to-osmosis-pica-and-back.log";
                namespace = "xapp";
              };

              eth-gen = {
                command = self'.packages.eth-gen;
                log_location = "${devnet-root-directory}/eth-gen.log";
                availability = { restart = chain-restart; };
              };
              eth-consensus-gen = {
                command = self'.packages.eth-consensus-gen;
                log_location = "${devnet-root-directory}/eth-consensus-gen.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-gen".condition = "process_completed_successfully";
                };
              };
              eth-executor-gen = {
                command = self'.packages.eth-executor-gen;
                log_location = "${devnet-root-directory}/eth-executor-gen.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-gen".condition = "process_completed_successfully";

                };
              };
              eth-executor = {
                command = self'.packages.eth-executor;
                log_location = "${devnet-root-directory}/eth-executor.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-executor-gen".condition =
                    "process_completed_successfully";
                };
                readiness_probe = {
                  exec.command = ''
                    test -f ${devnet-root-directory}/eth/jwtsecret
                  '';
                } // parachain-startup;

              };
              eth-consensus = {
                command = self'.packages.eth-consensus;
                log_location = "${devnet-root-directory}/eth-consensus.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-consensus-gen".condition =
                    "process_completed_successfully";
                  "eth-executor".condition = "process_healthy";
                };
              };
              eth-validator = {
                command = self'.packages.eth-validator;
                log_location = "${devnet-root-directory}/eth-validator.log";
                availability = { restart = chain-restart; };
                depends_on = {
                  "eth-consensus-gen".condition =
                    "process_completed_successfully";
                };
              };
            };
          };
        };

      };
    };
}
