{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      build = pkgs.callPackage ./../../paritytech/zombienet/default.nix { };
      paritytech-zombienet = self'.packages.paritytech-zombienet;

      writeZombienetShellApplication = name: config:
        pkgs.writeShellApplication rec {
          inherit name;
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet ];
          text = ''
            printf '${builtins.toJSON config}' > /tmp/${name}.json
            cd ${paritytech-zombienet}            
            npm run zombie spawn /tmp/${name}.json
          '';
        };

      zombienet-rococo-local-composable-config = with build;
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
        with build;
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
        in writeZombienetShellApplication name config;

    in with build; {
      _module.args.zombieTools = rec {
        inherit zombienet-rococo-local-composable-config
          writeZombienetShellApplication zombienet-to-ops;
      };

      packages = rec {
        zombienet-rococo-local-dali-dev =
          writeZombienetShellApplication "zombienet-rococo-local-dali-dev"
          (zombienet-rococo-local-composable-config { });

        zombienet-rococo-local-picasso-dev =
          writeZombienetShellApplication "zombienet-rococo-local-dali-dev"
          (zombienet-rococo-local-composable-config { chain = "picasso-dev"; });

        devnet-dali-centauri-a =
          writeZombienetShellApplication "devnet-dali-centauri-a"
          (zombienet-rococo-local-composable-config { });

        devnet-dali-centauri-b =
          writeZombienetShellApplication "devnet-dali-centauri-b"
          (zombienet-rococo-local-composable-config {
            rpc_port = 32201;
            ws_port = 29988;
            relay_ws_port = 29944;
          });

        devnet-dali-complete =
          mk-zombienet-all "devnet-dali-complete" "dali-dev";
        devnet-picasso-complete =
          mk-zombienet-all "devnet-picasso-complete" "picasso-dev";
      };

      apps = rec {
        zombienet-rococo-local-dali-dev = {
          type = "app";
          program = self'.packages.zombienet-rococo-local-dali-dev;
        };

        zombienet-all-dev-local = {
          type = "app";
          program = self'.packages.zombienet-all-dev-local;
        };

        devnet-dali-complete = {
          type = "app";
          program = self'.packages.devnet-dali-complete;
        };

        devnet-picasso-complete = {
          type = "app";
          program = self'.packages.devnet-picasso-complete;
        };

        devnet-dali-centauri-b = {
          type = "app";
          program = self'.packages.devnet-dali-centauri-b;
        };
      };
    };
}
