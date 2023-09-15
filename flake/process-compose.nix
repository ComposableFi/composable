{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }:
    let
      devnet-root-directory = "/tmp/composable-devnet";
      validator-key = "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj";
      relay = "no"; # `no` not to restart
      chain-restart = "no"; # `no` not to restart
    in {

      packages = rec {
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

        devnet-xc-cosmos-fresh = pkgs.writeShellApplication {
          runtimeInputs = devnetTools.withBaseContainerTools;
          name = "devnet-xc-cosmos-fresh";
          text = ''
            rm --force --recursive ${devnet-root-directory}             
            mkdir --parents ${devnet-root-directory}
            ${pkgs.lib.meta.getExe self'.packages.devnet-xc-cosmos}
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
          debug = true;
          settings = {
            processes = {
              picasso = {
                command = self'.packages.zombienet-rococo-local-picasso-dev;
                availability = { restart = chain-restart; };
                log_location = "${devnet-root-directory}/picasso.log";
                readiness_probe = {
                  initial_delay_seconds = 32;
                  period_seconds = 8;
                  failure_threshold = 8;
                  timeout_seconds = 2;
                  exec.command = ''
                    curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://127.0.0.1:9988
                  '';
                };
              };
              composable = {
                command = self'.packages.zombienet-composable-westend-b;
                availability = { restart = chain-restart; };
                log_location = "${devnet-root-directory}/composable.log";
                readiness_probe = {
                  initial_delay_seconds = 32;
                  period_seconds = 8;
                  failure_threshold = 8;
                  timeout_seconds = 2;
                  exec.command = ''
                    curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://127.0.0.1:29988
                  '';
                };
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
          debug = true;
          settings = {
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
              };
              centauri-init = {
                command = self'.packages.centaurid-init;
                depends_on."centauri".condition = "process_healthy";
                log_location = "${devnet-root-directory}/centauri-init.log";
                availability = { restart = "on_failure"; };
              };

              osmosis = {
                command = self'.packages.osmosisd-gen;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = 36657;
                };
                log_location = "${devnet-root-directory}/osmosis.log";
                availability = { restart = chain-restart; };
              };
              osmosisd-xcvm-init = {
                command = self'.packages.osmosisd-xcvm-init;
                depends_on."osmosis".condition = "process_healthy";
                log_location =
                  "${devnet-root-directory}/osmosisd-xcvm-init.log";
                availability = { restart = "on_failure"; };
              };

              picasso = {
                command = self'.packages.zombienet-rococo-local-picasso-dev;
                availability = { restart = chain-restart; };
                log_location = "${devnet-root-directory}/picasso.log";
                readiness_probe = {
                  initial_delay_seconds = 32;
                  period_seconds = 8;
                  failure_threshold = 8;
                  timeout_seconds = 2;
                  exec.command = ''
                    curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://127.0.0.1:9988
                  '';
                };
              };
              composable = {
                command = self'.packages.zombienet-composable-westend-b;
                availability = { restart = chain-restart; };
                log_location = "${devnet-root-directory}/composable.log";
                readiness_probe = {
                  initial_delay_seconds = 32;
                  period_seconds = 8;
                  failure_threshold = 8;
                  timeout_seconds = 2;
                  exec.command = ''
                    curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "assets_listAssets"}' http://127.0.0.1:29988
                  '';
                };
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
                availability = { restart = "on_failure"; };
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
                availability = { restart = "on_failure"; };
              };

              picasso-centauri-ibc-connection-init = {
                command = self'.packages.picasso-centauri-ibc-connection-init;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-connection-init.log";
                depends_on = {
                  "picasso-centauri-ibc-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = "on_failure"; };
              };

              picasso-centauri-ibc-channels-init = {
                command = self'.packages.picasso-centauri-ibc-channels-init;
                log_location =
                  "${devnet-root-directory}/picasso-centauri-ibc-channels-init.log";
                depends_on = {
                  "picasso-centauri-ibc-connection-init".condition =
                    "process_completed_successfully";
                };
                availability = { restart = "on_failure"; };
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
                availability = { restart = "on_failure"; };
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
                availability = { restart = "on_failure"; };
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
                availability = { restart = "on_failure"; };
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

        devnet-xc-cosmos = {
          debug = true;
          settings = {
            processes = {
              centauri = {
                command = self'.packages.centaurid-gen;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = 26657;
                };
                log_location = "${devnet-root-directory}/centauri.log";
                availability = { restart = "on_failure"; };
              };
              centauri-init = {
                command = self'.packages.centaurid-init;
                depends_on."centauri".condition = "process_healthy";
                log_location = "${devnet-root-directory}/centauri-init.log";
                availability = { restart = "on_failure"; };
              };

              centauri-xcvm-init = {
                command = self'.packages.centaurid-xcvm-init;
                depends_on."centauri".condition = "process_healthy";
                log_location =
                  "${devnet-root-directory}/centauri-xcvm-init.log";
                availability = { restart = "on_failure"; };
              };

              centauri-xcvm-config = {
                command = self'.packages.centaurid-xcvm-config;
                depends_on."centauri-xcvm-init".condition =
                  "process_completed_successfully";
                depends_on."osmosis-xcvm-init".condition =
                  "process_completed_successfully";
                log_location =
                  "${devnet-root-directory}/centauri-xcvm-config.log";
                availability = { restart = "on_failure"; };
              };

              osmosis-xcvm-config = {
                command = self'.packages.osmosisd-xcvm-config;
                depends_on."centauri-xcvm-init".condition =
                  "process_completed_successfully";
                depends_on."osmosis-xcvm-init".condition =
                  "process_completed_successfully";
                log_location =
                  "${devnet-root-directory}/osmosis-xcvm-config.log";
                availability = { restart = "on_failure"; };
              };

              osmosis = {
                command = self'.packages.osmosisd-gen;
                readiness_probe.http_get = {
                  host = "127.0.0.1";
                  port = 36657;
                };
                log_location = "${devnet-root-directory}/osmosis.log";
              };
              osmosis-pools-init = {
                command = self'.packages.osmosisd-pools-init;
                depends_on."osmosis".condition = "process_healthy";
                log_location =
                  "${devnet-root-directory}/osmosisd-pools-init.log";
                availability = { restart = "on_failure"; };
              };
              osmosis-xcvm-init = {
                command = self'.packages.osmosisd-xcvm-init;
                depends_on."osmosis".condition = "process_healthy";
                log_location = "${devnet-root-directory}/osmosis-xcvm-init.log";
                availability = { restart = "on_failure"; };
              };

              osmosis-centauri-hermes-init = {
                command = self'.packages.osmosis-centauri-hermes-init;
                depends_on = {
                  "centauri-init".condition = "process_completed_successfully";
                  "osmosis".condition = "process_healthy";
                };
                log_location =
                  "${devnet-root-directory}/osmosis-centauri-hermes-init.log";
                availability = { restart = "on_failure"; };
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
            };
          };
        };
      };
    };
}
