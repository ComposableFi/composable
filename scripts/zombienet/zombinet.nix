{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      paritytech-zombienet-src = pkgs.fetchFromGitHub {
        owner = "paritytech";
        repo = "zombienet";
        rev = "9cf8e598f0b8e8b88bc1b0e677acb7ba322c3a1a";
        hash = "sha256-ZqCHgkr5lVsGFg/Yvx6QY/zSiIafwSec+oiioOWTZMg=";
      };

    in {
      packages = rec {
        paritytech-zombienet = pkgs.buildNpmPackage {
          src = "${paritytech-zombienet-src}/javascript";
          npmBuild = """
          npm run build
          npm run package
          """;
          installPhase = ''
            mkdir -p $out
            cp -a ./build/. $out
          '';
        };
      };

      # apps = {
      #   zombienet = {
      #     type = "app";
      #     program = packages.paritytech-zombienet;
      #   };
      # };
    };
}
