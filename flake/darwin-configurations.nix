{ self, withSystem, inputs, lib, options, flake-parts-lib, specialArgs, config
}: {
  flake = {
    darwinConfigurations = let 
    
      user = "administrator";
    in {
      default = self.inputs.darwin.lib.darwinSystem {
        system = "aarch64-darwin";
        modules = [
          ({ config, pkgs, ... }: {
            environment.systemPackages = with pkgs; [
              helix
              github-runner
              git
              git-lfs
              docker
              nix
              cachix
            ];

            services = { nix-daemon.enable = true; };
            launchd.daemons.github-runner = {
              serviceConfig = {
                ProgramArguments = [
                  "/bin/sh"
                  "-c"
                  # follow exact steps of github guide to get this available
                  # so more automatic nix version would use pkgs.github-runner (and token sshed as file)
                  "/Users/${user}/actions-runner/run.sh"
                ];
                Label = "github-runner";
                KeepAlive = true;
                RunAtLoad = true;

                StandardErrorPath =
                  "/Users/${user}/actions-runner/err.log";
                StandardOutPath = "/Users/${user}/actions-runner/ok.log";
                WorkingDirectory = "/Users/${user}/actions-runner/";
                SessionCreate = true;
                UserName = "${user}";

              };
            };

            nix.package = pkgs.nix;
            programs.bash.enable = true;
            system.stateVersion = 4;
          }

          )
        ];
        inputs = {
          darwin = self.inputs.darwin;
          nixpkgs = self.inputs.nixpkgs;
        };
      };
    };
  };
}

