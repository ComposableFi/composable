{ self, withSystem, inputs, lib, options, flake-parts-lib, specialArgs, config
}: {

  perSystem = { config, self', inputs', pkgs, system, ... }: {
    apps = let name = "composable-devnet";
    in {
      composable-devnet-deploy = {
        type = "app";
        program = pkgs.writeShellApplication {
          name = "composable-devnet-deploy";
          text = ''
            NIX_SSHOPTS="-i .ssh/id_rsa"         
            export NIX_SSHOPTS
            nixos-rebuild switch --fast --flake .#root  --target-host root@static.28.137.109.65.clients.your-server.de            
          '';
        };
      };
    };
  };

  flake = {
    nixosConfigurations = let user = "root";
    in (withSystem "x86_64-linux"
      ({ config, self', inputs', pkgs, devnetTools, subnix, system, ... }: {
        default = "${user}";
        "${user}" = let
          ports = [ 22 80 443 9988 9944 10008 29988 29944 30008 26657 36657 ];
        in self.inputs.nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [
            ({ ... }: {
              imports =
                [ ./hardware-configuration.nix ./networking.nix ./host.nix ];
              system.stateVersion = "23.11";
              environment.systemPackages = with pkgs;
                with self'.packages;
                [ devnet-xc-fresh-background helix process-compose ]
                ++ devnetTools.withDevNetContainerTools;
              boot.tmp.cleanOnBoot = true;
              zramSwap.enable = true;
              networking.hostName = "composable-devnet";
              networking.firewall.allowedTCPPorts = ports;
              networking.domain = "";
              services.openssh.enable = true;
              services.caddy = {
                enable = true;
                virtualHosts."localhost".extraConfig = ''
                  respond "composable devnet!"
                '';
              };

              systemd.services.composable-devnet = {
                wantedBy = [ "multi-user.target" ];
                after = [ "network.target" ];
                description = "composable-devnet";
                serviceConfig = {
                  Type = "simple";
                  User = "root";
                  ExecStart = "${pkgs.lib.meta.getExe
                    self'.packages.devnet-xc-fresh-background}";
                  Restart = "always";
                };
              };
            })

          ];
        };
      })) // { };
  };
}
