{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, zombieTools, ... }:
    let
      prelude = zombieTools.builder;
      relaychainBase = {
        chain = "rococo-local";
        default_command =
          pkgs.lib.meta.getExe self'.packages.polkadot-fast-runtime;
        count = 2;
      };

      overrideZombienet = with prelude;
        { chain, ws_port ? null, rpc_port ? null, relay_ws_port ? null
        , relay_rpc_port ? null, rust_log_add ? null, para-id ? 2087
        , command ? self'.packages.composable-node, relaychain ? relaychainBase
        , parachains ? [ ] }:
        mkZombienet {
          relaychain = relaychain
            // (pkgs.lib.optionalAttrs (relay_ws_port != null) {
              ws_port = relay_ws_port;
            });
          parachains = [
            ({
              command = pkgs.lib.meta.getExe command;
              inherit chain;
              id = para-id;
              collators = 2;
            } // (pkgs.lib.optionalAttrs (chain != null) { inherit chain; })
              // (pkgs.lib.optionalAttrs (rust_log_add != null) {
                inherit rust_log_add;
              })
              // (pkgs.lib.optionalAttrs (ws_port != null) { inherit ws_port; })
              // (pkgs.lib.optionalAttrs (rpc_port != null) {
                inherit rpc_port;
              }))
          ] ++ parachains;
        };

      asset-hub-kusama-local = {
        command = pkgs.lib.meta.getExe self'.packages.polkadot-parachain;
        chain = "statemine-local";
        id = 1000;
        collators = 1;
        ws_port = 10008;
        rpc_port = 32220;
        genesis = {
          runtime = {
            parachainInfo = { parachainId = 1000; };
            balances = {
              balances = {
                "0" = {
                  "0" = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
                  "1" = 17476266491902;
                };
              };
            };
            assets = {
              assets = {
                "0" = {
                  "0" = "1984";
                  "1" = "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
                  "2" = true;
                  "3" = 123456789;
                };
              };
              metadata = {
                "0" = {
                  "0" = "1984";
                  "1" = [ 85 83 68 84 ];
                  "2" = [ 85 83 68 84 ];
                  "3" = 6;
                };
              };
            };
          };
        };
      };

      mk-zombienet-all = name: chain:
        with prelude;
        let
          config = mkZombienet {
            relaychain = relaychainBase;
            parachains = [
              {
                command = pkgs.lib.meta.getExe self'.packages.composable-node;
                inherit chain;
                id = 2087;
                collators = 3;
              }

              {
                command = pkgs.lib.meta.getExe self'.packages.acala-node;
                chain = "karura-dev";
                id = 2000;
                collators = 1;
                ws_port = 9999;
                rpc_port = 32210;
              }
            ];
          };
        in zombieTools.writeZombienetShellApplication name config;

      picasso-dev-config = overrideZombienet {
        chain = "picasso-dev";
        command = self'.packages.composable-testfast-node;
        parachains = [ asset-hub-kusama-local ];
      };

      zombienet-rococo-local-picasso-dev = zombieTools.writeZombienet {
        name = "zombienet-rococo-local-picasso-dev";
        config = picasso-dev-config;
        dir = "/tmp/composable-devnet/picasso-rococo";
      };

    in with prelude; {

      packages = rec {
        devnet-picasso = zombienet-rococo-local-picasso-dev;
        devnet-composable = zombienet-westend-local-composable-dev;

        inherit zombienet-rococo-local-picasso-dev;

        zombienet-westend-local-composable-dev = zombieTools.writeZombienet {
          dir = "/tmp/composable-devnet/composable-westend";
          name = "zombienet-westend-local-composable-dev";
          config = (overrideZombienet {
            chain = "composable-dev";
            relaychain = {
              # can build with `fast-runtime`, both relay, relay as part of para, and relay runtime
              chain = "westend-local";
              default_command =
                pkgs.lib.meta.getExe self'.packages.polkadot-fast-runtime;
              count = 3;
              genesis = {
                # whatever stakin here to add and handle PoS instead of PoA
                runtime = {
                  balances = {
                    balances = {
                      "0" = {
                        "0" =
                          "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY";
                        "1" = 17476266491902;
                      };
                    };
                  };
                };
              };
            };
            parachains = [{
              command = pkgs.lib.meta.getExe self'.packages.polkadot-parachain;
              chain = "statemint-local";
              id = 1000;
              collators = 2;
              ws_port = 10018;
              rpc_port = 32240;
            }];
          });
        };

        zombienet-picasso-centauri-a =
          zombieTools.writeZombienetShellApplication
          "zombienet-picasso-centauri-a" (overrideZombienet {
            rust_log_add =
              "runtime::contracts=debug,ibc_transfer=trace,pallet_ibc=trace,grandpa-verifier=debug";
            command = self'.packages.composable-testfast-node;
            chain = "picasso-dev";
            parachains = [ asset-hub-kusama-local ];
          });

        zombienet-picasso-centauri-b =
          zombieTools.writeZombienetShellApplication
          "zombienet-picasso-centauri-b" (overrideZombienet {
            ws_port = 29988;
            rpc_port = 32201;
            relay_ws_port = 29944;
            relay_rpc_port = 31445;
            rust_log_add =
              "runtime::contracts=debug,ibc_transfer=trace,pallet_ibc=trace,grandpa-verifier=debug";
            command = self'.packages.composable-testfast-node;
            chain = "picasso-dev";
          });

        zombienet-composable-westend-b = zombieTools.writeZombienet {
          name = "zombienet-composable-westend-b";
          dir = "/tmp/composable-devnet/composable-westend-b";
          config = (overrideZombienet {
            ws_port = 29988;
            rpc_port = 32201;
            relay_ws_port = 29944;
            relay_rpc_port = 31445;
            rust_log_add =
              "runtime::contracts=debug,ibc_transfer=trace,pallet_ibc=trace,grandpa-verifier=debug";
            command = self'.packages.composable-testfast-node;
            chain = "composable-dev";
            parachains = [{
              command = pkgs.lib.meta.getExe self'.packages.polkadot-parachain;
              chain = "statemint-local";
              id = 1000;
              collators = 2;
              ws_port = 30008;
              rpc_port = 32240;
            }];
          });
        };

      };

      apps = rec {
        zombienet-log-follow = {
          program = pkgs.writeShellApplication rec {
            name = "zombienet-log-follow";
            text = ''
              CONTAINER="''${1:-composable-devnet-a-1}"
              docker exec -it "$CONTAINER"   bash  -c 'LOG=$(find /tmp/ -name "zombie-*" | head --lines=1)/alice.log && tail --follow $LOG'
            '';
          };
          type = "app";
        };

        zombienet-log-cat = {
          program = pkgs.writeShellApplication rec {
            name = "zombienet-log-follow";
            text = ''
              CONTAINER="''${1:-composable-devnet-a-1}"
              docker exec -it "$CONTAINER"   bash  -c 'LOG=$(find /tmp/ -name "zombie-*" | head --lines=1)/alice.log && cat $LOG'
            '';
          };
          type = "app";
        };
      };
    };
}
