{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {
      docs-static = pkgs.buildNpmPackage {
        src = ./.;
        npmBuild = "npm run build";
        installPhase = ''
          mkdir -p $out
          cp -a ./build/. $out
        '';
      };

      docs-server = let PORT = 8008;
      in pkgs.writeShellApplication {
        name = "docs-server";
        runtimeInputs = [ pkgs.miniserve ];
        text = ''
          miniserve -p ${
            builtins.toString PORT
          } --spa --index index.html ${docs-static}
        '';
      };

    };

    apps = {
      docs-dev = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "docs-dev";
          runtimeInputs = [ pkgs.nodejs ];
          text = ''
            cd docs
            npm install
            npm run start
          '';
        };
      };
    };
  };
}
