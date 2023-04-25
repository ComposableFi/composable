{ self, withSystem, inputs, lib, options, flake-parts-lib, specialArgs, config
}: {
  flake = {
    darwinConfigurations = {
      "62260" = self.inputs.darwin.lib.darwinSystem {
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
                  "/Users/administrator/actions-runner/run.sh"
                ];
                Label = "github-runner";
                KeepAlive = true;
                RunAtLoad = true;

                StandardErrorPath =
                  "/Users/administrator/actions-runner/err.log";
                StandardOutPath = "/Users/administrator/actions-runner/ok.log";
                WorkingDirectory = "/Users/administrator/actions-runner/";
                SessionCreate = true;
                UserName = "administrator";

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

