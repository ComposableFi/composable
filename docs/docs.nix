{ self, ... }:
{
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      # we do not use pkgs or self', because we need to add
      # the npm-buildpackage overlay.
      pkgs' = import self.inputs.nixpkgs {
        inherit system;
        overlays = [
          self.inputs.npm-buildpackage.overlays.default
        ];
      };
    in
    {
      packages = rec {
        docs-static = pkgs'.buildNpmPackage {
          src = ./.;
          npmBuild = "npm run build";
          installPhase = ''
            mkdir -p $out
            cp -a ./build/. $out
          '';
        };

        docs-server = let PORT = 8008; in
          pkgs.writeShellApplication {
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
