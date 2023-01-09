{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      paritytech-zombienet-src = pkgs.fetchFromGitHub {
        owner = "paritytech";
        repo = "zombienet";
        rev = "81a88811674772c1a03ea0a6081a7331a1210d64";
        hash = "sha256-59lorg8GHkMH41SnijMFQyuJEsNWVuizxtoQzGP2osE=";
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
            printf '${builtins.toJSON config}' > /tmp/${name}.json
            cd ${paritytech-zombienet}            
            npm run zombie spawn /tmp/${name}.json
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
      };
    };
}
