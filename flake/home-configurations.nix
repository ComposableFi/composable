{ self, withSystem, inputs, lib, options, flake-parts-lib, specialArgs, config
}: {
  flake = {
    homeConfigurations = let user = "vscode";
    in (withSystem "x86_64-linux"
      ({ config, self', inputs', pkgs, devnetTools, this, subnix, ... }: {
        vscode = let codespace = with pkgs; [ cachix acl direnv ];
        in self.inputs.home-manager.lib.homeManagerConfiguration {
          inherit pkgs;
          modules = [{
            home = {
              username = user;
              sessionVariables = subnix.subattrs;
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
            programs = {
              home-manager.enable = true;
              direnv = {
                enable = true;
                nix-direnv = { enable = true; };
              };
            };
          }];
        };
      })) // { };
  };
}
