{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, zombieTools, ... }:
    let
      prelude = zombieTools.builder;
      relaychainBase = {
        chain = "rococo-local";
        default_command =
          pkgs.lib.meta.getExe self'.packages.polkadot-node-from-dep;
        count = 3;
      };

      zombienet-rococo-local-composable-config = with prelude;
        { chain, ws_port ? null, rpc_port ? null, relay_ws_port ? null
        , relay_rpc_port ? null, rust_log_add ? null, para-id ? 2087
        , command ? self'.packages.composable-node, relaychain ? relaychainBase
        }:
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
          ];
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
                command = pkgs.lib.meta.getExe self'.packages.statemine-node;
                chain = "statemine-local";
                id = 1000;
                collators = 2;
                ws_port = 10008;
                rpc_port = 32220;
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

      picasso-dev-ops = (zombieTools.zombienet-to-ops picasso-dev-config) // {
        script = zombienet-rococo-local-picasso-dev;
        chain-spec = "picasso-dev";
      };

      picasso-dev-config = zombienet-rococo-local-composable-config {
        chain = "picasso-dev";
        command = self'.packages.composable-testfast-node;
      };

      zombienet-rococo-local-picasso-dev =
        zombieTools.writeZombienetShellApplication
        "zombienet-rococo-local-picasso-dev" picasso-dev-config;

    in with prelude; {
      _module.args.this = rec { inherit picasso-dev-ops; };

      packages = rec {
        devnet-picasso = zombienet-rococo-local-picasso-dev;
        devnet-composable = zombienet-rococo-local-composable-dev;

        livenet-composable = zombieTools.writeZombienetShellApplication
          "zombienet-rococo-local-composable-dev"
          (zombienet-rococo-local-composable-config {
            chain = "composable-dev";
            relaychain = {
              chain = "rococo-local";
              default_command =
                pkgs.lib.meta.getExe self'.packages.polkadot-live-runtine-node;
              count = 3;
            };
          });

        zombienet-picasso-complete =
          mk-zombienet-all "devnet-picasso-complete" "picasso-dev";

        inherit zombienet-rococo-local-picasso-dev;

        zombienet-rococo-local-composable-dev =
          zombieTools.writeZombienetShellApplication
          "zombienet-rococo-local-composable-dev"
          (zombienet-rococo-local-composable-config {
            chain = "composable-dev";
          });

        zombienet-picasso-centauri-a =
          zombieTools.writeZombienetShellApplication
          "zombienet-picasso-centauri-a"
          (zombienet-rococo-local-composable-config {
            rust_log_add =
              "runtime::contracts=debug,ibc_transfer=trace,pallet_ibc=trace,grandpa-verifier=trace";
            command = self'.packages.composable-node;
            chain = "picasso-dev";
          });

        zombienet-picasso-centauri-b =
          zombieTools.writeZombienetShellApplication
          "zombienet-picasso-centauri-b"
          (zombienet-rococo-local-composable-config {
            ws_port = 29988;
            rpc_port = 32201;
            relay_ws_port = 29944;
            relay_rpc_port = 31445;
            rust_log_add =
              "runtime::contracts=debug,ibc_transfer=trace,pallet_ibc=trace,grandpa-verifier=trace";
            command = self'.packages.composable-node;
            chain = "picasso-dev";
          });

        zombienet-composable-centauri-b =
          zombieTools.writeZombienetShellApplication
          "zombienet-composable-centauri-b"
          (zombienet-rococo-local-composable-config {
            ws_port = 29988;
            rpc_port = 32201;
            relay_ws_port = 29944;
            relay_rpc_port = 31445;
            rust_log_add =
              "runtime::contracts=debug,ibc_transfer=trace,pallet_ibc=trace,grandpa-verifier=trace";
            command = self'.packages.composable-node;
            chain = "composable-dev";
          });
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
