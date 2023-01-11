{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      paritytech-zombienet-src = pkgs.fetchFromGitHub {
        owner = "dzmitry-lahoda-forks";
        repo = "zombienet";
        rev = "b9f089eb55a7cb1d4c12575b8323cb9b9fab4a60";
        hash = "sha256-EC6XKbcI+Is0RGlmC8WGjPqiFh9Ulf3bXDoVihtYqsU=";
      };

      all-dev-local-config = ./zombienet/all-dev-local.toml;
      build = pkgs.callPackage ./zombienet/default.nix { };
      npmDeps = pkgs.callPackage ../../.nix/npm.nix { };
      all-dev-local-config = ./all-dev-local.toml;
      runtimeDeps = with pkgs;
        [ coreutils bash procps git git-lfs ]
        ++ lib.optional stdenv.isLinux glibc.bin;
    in with build; {
      packages = rec {
        default = devnet-dali;
        devnet-dali = zombienet-rococo-local-dali-dev;
        paritytech-zombienet = pkgs.stdenv.mkDerivation {
          name = "zombienet";
          src = "${paritytech-zombienet-src}/javascript";
          buildInputs = with pkgs; [ nodejs ];
          nativeBuildInputs = with pkgs; [
            yarn
            nodejs
            python3
            pkg-config
            vips
            nodePackages.node-gyp-build
            nodePackages.node-gyp
            nodePackages.typescript
            coreutils
          ];
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
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet ] ++ runtimeDeps;
          text = ''
            cd ${paritytech-zombienet}
            npm run zombie
          '';
        };

        zombienet-rococo-local-dali-dev-statemine = pkgs.writeShellApplication {
          name = "zombienet-rococo-local-dali-dev-statemine";
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet ] ++ runtimeDeps;
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
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet pkgs.bash ]
            ++ runtimeDeps;
          text = ''
            realpath bash
            bash --version
            export DEBUG="zombie*"
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
