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


# { self, ... }: {
#   perSystem = { config, self', inputs', pkgs, system, ... }: {
#     packages = {
#       subsquid-processor = let
#         processor = pkgs.buildNpmPackage {
#           extraNodeModulesArgs = {
#             buildInputs = with pkgs; [
#               pkg-config
#               python3
#               nodePackages.node-gyp-build
#               nodePackages.node-gyp
#             ];
#             extraEnvVars = { npm_config_nodedir = "${pkgs.nodejs}"; };
#           };
#           src = ./.;
#           npmBuild = "npm run build";
#           preInstall = ''
#             mkdir $out
#             mv lib $out/
#           '';
#           dontNpmPrune = true;
#         };
#       in (pkgs.writeShellApplication {
#         name = "run-subsquid-processor";
#         text = ''
#           cd ${processor}
#           ${pkgs.nodejs}/bin/npx sqd db migrate
#           ${pkgs.nodejs}/bin/node lib/processor.js
#         '';
#       });
#     };
#   };
# }
