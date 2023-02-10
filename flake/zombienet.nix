{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, zombieTools, ... }:
    let
      prelude = zombieTools.builder;
      relaychainBase = {
        chain = "rococo-local";
        default_command = pkgs.lib.meta.getExe self'.packages.polkadot-node;
        count = 3;
      };

      zombienet-rococo-local-composable-config = with prelude;
        { chain ? "dali-dev", ws_port ? null, rpc_port ? null
        , relay_ws_port ? null, relay_rpc_port ? null, rust_log_add ? null
        , para-id ? 2087 }:
        mkZombienet {
          relaychain = relaychainBase
            // (pkgs.lib.optionalAttrs (chain != null) {
              ws_port = relay_ws_port;
            });
          parachains = [
            ({
              command = pkgs.lib.meta.getExe self'.packages.composable-node;
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

    in with prelude; {
      packages = rec {
        default = devnet-dali;
        devnet-dali = zombienet-rococo-local-dali-dev;
        devnet-picasso = zombienet-rococo-local-picasso-dev;
        devnet-composable = zombienet-rococo-local-composable-dev;

        zombienet-dali-complete =
          mk-zombienet-all "devnet-dali-complete" "dali-dev";
        zombienet-picasso-complete =
          mk-zombienet-all "devnet-picasso-complete" "picasso-dev";

        zombienet-rococo-local-dali-dev =
          zombieTools.writeZombienetShellApplication
          "zombienet-rococo-local-dali-dev"
          (zombienet-rococo-local-composable-config { });

        zombienet-rococo-local-picasso-dev =
          zombieTools.writeZombienetShellApplication
          "zombienet-rococo-local-picasso-dev"
          (zombienet-rococo-local-composable-config { chain = "picasso-dev"; });

        zombienet-rococo-local-composable-dev =
          zombieTools.writeZombienetShellApplication
          "zombienet-rococo-local-composable-dev"
          (zombienet-rococo-local-composable-config {
            chain = "composable-dev";
          });

        zombienet-dali-centauri-a =
          zombieTools.writeZombienetShellApplication "zombienet-dali-centauri-a"
          (zombienet-rococo-local-composable-config {
            rust_log_add =
              "runtime::contracts=debug,ibc_transfer=trace,pallet_ibc=trace,grandpa-verifier=trace";
          });

        zombienet-dali-centauri-b =
          zombieTools.writeZombienetShellApplication "zombienet-dali-centauri-b"
          (zombienet-rococo-local-composable-config {
            ws_port = 29988;
            rpc_port = 32201;
            relay_ws_port = 29944;
            relay_rpc_port = 31445;
            rust_log_add =
              "runtime::contracts=debug,ibc_transfer=trace,pallet_ibc=trace,grandpa-verifier=trace";
          });
      };

      apps = rec {

        zombienet = {
          type = "app";
          program = self'.packages.zombienet;
        };
        zombienet-dali-complete = {
          type = "app";
          program = self'.packages.zombienet-dali-complete;
        };

        zombienet-dali-centauri-a = {
          type = "app";
          program = self'.packages.zombienet-dali-centauri-a;
        };

        zombienet-picasso-complete = {
          type = "app";
          program = self'.packages.zombienet-picasso-complete;
        };

        zombienet-log-follow = {
          program = pkgs.writeShellApplication rec {
            name = "zombienet-log-follow";
            text = ''
              docker exec -it composable-devnet-a-1   bash  -c 'LOG=$(find /tmp/ -name "zombie-*" | head --lines=1)/alice.log && tail --follow $LOG'
            '';
          };
          type = "app";
        };

        zombienet-rococo-local-dali-dev = {
          type = "app";
          program = self'.packages.zombienet-rococo-local-dali-dev;
        };
        zombienet-rococo-local-picasso-dev = {
          type = "app";
          program = self'.packages.zombienet-rococo-local-picasso-dev;
        };
      };
    };
}
