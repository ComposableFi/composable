{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      prelude = pkgs.callPackage ./zombienet/default.nix { };
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

    in with prelude; {
      packages = rec {
        default = devnet-dali;
        devnet-dali = zombienet-rococo-local-dali-dev;

        zombienet-rococo-local-dali-dev-statemine = pkgs.writeShellApplication {
          name = "zombienet-rococo-local-dali-dev-statemine";
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet ] ++ runtimeDeps;
          text = ''
            cd ${paritytech-zombienet}            
            npm run zombie spawn ${all-dev-local-config}
          '';
        };
        zombienet-rococo-local-dali-dev =
          zombieTools.writeZombienetShellApplication "zombienet-rococo-local-dali-dev"
          (zombienet-rococo-local-composable-config { });

        zombienet-rococo-local-picasso-dev =
          zombieTools.writeZombienetShellApplication "zombienet-rococo-local-dali-dev"
          (zombienet-rococo-local-composable-config { chain = "picasso-dev"; });
      };

      apps = rec {

        zombienet = {
          type = "app";
          program = self'.packages.zombienet;
        };
        zombienet-rococo-local-dali-dev-statemine = {
          type = "app";
          program = self'.packages.zombienet-rococo-local-dali-dev-statemine;
        };
        zombienet-rococo-local-dali-dev = {
          type = "app";
          program = self'.packages.zombienet-rococo-local-dali-dev;
        };
      };
    };
}
