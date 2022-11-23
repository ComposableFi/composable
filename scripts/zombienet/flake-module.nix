{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      paritytech-zombienet-src = pkgs.fetchFromGitHub {
        owner = "paritytech";
        repo = "zombienet";
        rev = "9cf8e598f0b8e8b88bc1b0e677acb7ba322c3a1a";
        hash = "sha256-+tyVQa+BYdFphSLbinMFZlhV/fPG8R+/mwij36WwEEM=";
      };

      # will be builder of it https://app.clickup.com/t/3u4b2ad
      all-dev-local-config = ./all-dev-local.toml;

    in {
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
          # npm build fails with https://github.com/serokell/nix-npm-buildpackage/pull/62 (also i have updaed to this PR...)
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
      };
    };
}
