{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, zombieTools, ... }:
    let
      prelude = zombieTools.builder;
      zombienet-rococo-local-composable-config = with prelude;
        { chain ? null, ws_port ? null, rpc_port ? null, relay_ws_port ? null }:
        mkZombienet {
          relaychain = {
            chain = "rococo-local";
            default_command = pkgs.lib.meta.getExe self'.packages.polkadot-node;
            count = 3;
          } // (pkgs.lib.optionalAttrs (chain != null) {
            ws_port = relay_ws_port;
          });
          parachains = [
            ({
              command = pkgs.lib.meta.getExe self'.packages.composable-node;
              chain = "dali-dev";
              id = 2087;
              collators = 3;
            } // (pkgs.lib.optionalAttrs (chain != null) { inherit chain; })
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
            relaychain = {
              chain = "rococo-local";
              default_command =
                pkgs.lib.meta.getExe self'.packages.polkadot-node;
              count = 3;
            };
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
                collators = 0;
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
          "zombienet-rococo-local-dali-dev"
          (zombienet-rococo-local-composable-config { chain = "picasso-dev"; });
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

        zombienet-picasso-complete = {
          type = "app";
          program = self'.packages.zombienet-picasso-complete;
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
