{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      paritytech-zombienet-src = pkgs.fetchFromGitHub {
        owner = "paritytech";
        repo = "zombienet";
        rev = "9cf8e598f0b8e8b88bc1b0e677acb7ba322c3a1a";
        hash = "sha256-+tyVQa+BYdFphSLbinMFZlhV/fPG8R+/mwij36WwEEM=";
      };

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
      };

      apps = {
        zombienet = {
          type = "app";
          program = self'.packages.zombienet;
        };
      };
    };
}
