{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      paritytech-zombienet-src = pkgs.fetchFromGitHub {
        owner = "paritytech";
        repo = "zombienet";
        rev = "9cf8e598f0b8e8b88bc1b0e677acb7ba322c3a1a";
        hash = "sha256-+tyVQa+BYdFphSLbinMFZlhV/fPG8R+/mwij36WwEEM=";
      };

      build = pkgs.callPackage ./default.nix { };
      all-dev-local-config = ./all-dev-local.toml;
    in with build; {
      packages = rec {
        paritytech-zombienet = pkgs.stdenv.mkDerivation {
          name = "zombienet";
          src = "${paritytech-zombienet-src}/javascript";
          buildInputs = with pkgs; [ nodejs ];
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

        zombienet-devnet-dali-complete = pkgs.writeShellApplication {
          name = "zombienet-devnet-dali-complete";
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
          name = "rococo-local-dali-dev";
          runtimeInputs = [ pkgs.nodejs pkgs.yq paritytech-zombienet ];
          text = ''
            printf '${builtins.toJSON config}' > ${name}.json

            ${pkgs.yq}/bin/yq  . ${name}.json --toml-output > ${name}.toml
            cat ${name}.toml      
            ${pkgs.lib.meta.getExe zombienet} spawn ${name}.toml
          '';
        };
      };

      apps = {

        zombienet = {
          type = "app";
          program = self'.packages.zombienet;
        };
        zombienet-devnet-dali-complete = {
          type = "app";
          program = self'.packages.zombienet-devnet-dali-complete;
        };
        zombienet-rococo-local-dali-dev = {
          type = "app";
          program = self'.packages.zombienet-rococo-local-dali-dev;
        };
      };
    };
}
