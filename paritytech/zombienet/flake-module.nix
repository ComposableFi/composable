{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      paritytech-zombienet-src = pkgs.fetchFromGitHub {
        owner = "paritytech";
        repo = "zombienet";
        rev = "1d94ce94aea86a58a728ad200f6cdf6f3a721c2f";
        hash = "sha256-v5IpdaKivHtuxPbAyK0Z7kyyHwPl0/E9quLBa/9kg98=";
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
      };

      apps = rec {
        zombienet = {
          type = "app";
          program = self'.packages.zombienet;
        };
      };
    };
}
