{ self, withSystem, inputs, lib, options, flake-parts-lib, specialArgs, config
}: {
  flake = {
    homeConfigurations = let user = "vscode";
    in (withSystem "x86_64-linux"
      ({ config, self', inputs', pkgs, devnetTools, this, subnix, ... }: {
        "${user}" = let
          codespace = with pkgs; [ cachix acl direnv ];
          env-vars = subnix.subattrs;

        in self.inputs.home-manager.lib.homeManagerConfiguration {
          inherit pkgs;
          modules = [
            ({ config, ... }: {
              home = let sessionVarsStr = config.lib.shell.exportAll env-vars;
              in {
                username = user;
                sessionVariables = env-vars;
                homeDirectory = "/home/${user}";
                stateVersion = "22.11";
                packages = with pkgs;
                  with self'.packages;
                  [
                    clang
                    nodejs
                    python3
                    yarn
                    sad
                    git
                    git-lfs
                    subwasm
                    zombienet

                  ] ++ (with self'.packages; [ rust-nightly ]) ++ codespace;
              };
              programs =
                let sessionVarsStr = config.lib.shell.exportAll env-vars;
                in {
                  home-manager.enable = true;
                  bash = {
                    enable = true;
                    enableCompletion = true;
                    sessionVariables = env-vars;
                    bashrcExtra = ''
                      ${sessionVarsStr}
                    '';
                    initExtra = ''
                      ${sessionVarsStr}
                    '';
                    profileExtra = ''
                      ${sessionVarsStr}
                    '';
                  };
                  direnv = {
                    enable = true;
                    nix-direnv = { enable = true; };
                  };
                };
            })

          ];
        };
      })) // { };
  };
}
