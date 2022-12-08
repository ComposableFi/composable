{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      paritytech-zombienet-src = pkgs.fetchFromGitHub {
        owner = "paritytech";
        repo = "zombienet";
        rev = "30f3cffc494622f710f73297a68831e564bbf9f1";
        hash = "sha256-l4kCsTRXPKxg3UongXd/pWAdGTaV9k5pnlidU/2LWR4=";
      };

      build = pkgs.callPackage ./default.nix { };
      npmDeps = pkgs.callPackage ../../.nix/npm.nix { };
      all-dev-local-config = ./all-dev-local.toml;
    in with build; {
      packages = rec {
        paritytech-zombienet = pkgs.stdenv.mkDerivation {
          name = "zombienet";
          src = "${paritytech-zombienet-src}/javascript";
          buildInputs = with pkgs; [ nodejs ];
          nativeBuildInputs = npmDeps.nativeBuildInputs;
          buildPhase = ''
            mkdir home
            export HOME=$PWD/home
            npm install
            npm run build
          '';
          installPhase = ''
            mkdir --parents $out
            cp . $out --recursive
          '';
          # https://app.clickup.com/t/3w8y83f
          # npm build fails with https://github.com/serokell/nix-npm-buildpackage/pull/62 (also i have updated to this PR...)
          # cannot use fix because of https://github.com/serokell/nix-npm-buildpackage/pull/54#issuecomment-1254908364
          __noChroot = true;
        };

        zombienet = pkgs.writeShellApplication {
          name = "zombienet";
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet ];
          text = ''
            cd ${paritytech-zombienet}
            npm run zombie
          '';
        };

        zombienet-rococo-local-dali-dev-statemine = pkgs.writeShellApplication {
          name = "zombienet-rococo-local-dali-dev-statemine";
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet ];
          text = ''
            cd ${paritytech-zombienet}            
            npm run zombie spawn ${all-dev-local-config}
          '';
        };

        zombienet-rococo-local-dali-dev = let
          config = mkZombienet {
            relaychain = {
              chain = "rococo-local";
              default_command =
                pkgs.lib.meta.getExe self'.packages.polkadot-node;
              count = 3;
            };
            parachains = [{
              command = pkgs.lib.meta.getExe self'.packages.composable-node;
              chain = "dali-dev";
              id = 2087;
              collators = 3;
            }];
          };
        in pkgs.writeShellApplication rec {
          name = "zombienet-rococo-local-dali-dev";
          runtimeInputs = [ pkgs.nodejs pkgs.yq paritytech-zombienet ];
          text = ''
            printf '${builtins.toJSON config}' > ${name}.json
            CONFIG=$PWD
            ${pkgs.yq}/bin/yq  . ${name}.json --toml-output > ${name}.toml
            cat ${name}.toml      
            cd ${paritytech-zombienet}            
            npm run zombie spawn "$CONFIG"/${name}.toml
          '';
        };

        zombienet-all-dev-local = let
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
                chain = "dali-dev";
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
        in pkgs.writeShellApplication rec {
          name = "zombienet-all-dev-local";
          runtimeInputs = [ pkgs.nodejs pkgs.yq paritytech-zombienet ];
          text = ''
            printf '${builtins.toJSON config}' > ${name}.json
            CONFIG=$PWD
            ${pkgs.yq}/bin/yq  . ${name}.json --toml-output > ${name}.toml
            cat ${name}.toml      
            cd ${paritytech-zombienet}            
            npm run zombie spawn "$CONFIG"/${name}.toml
          '';
        };

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

        zombienet-all-dev-local = {
          type = "app";
          program = self'.packages.zombienet-all-dev-local;
        };        
      };
    };
}
