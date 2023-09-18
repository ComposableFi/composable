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
            # first run will be slow, so can consider variouse optimization later
            nixos-rebuild switch --fast --flake .#root  --target-host root@65.109.137.28            
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
        "${user}" = let codespace = with pkgs; [ cachix acl direnv ];
        in self.inputs.nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [
            ({ ... }: {
              imports = [
                ./hardware-configuration.nix
                ./networking.nix # generated at runtime by nixos-infect
                ./host.nix
              ];
              system.stateVersion = "23.11";
              environment.systemPackages =
                [ self'.packages.devnet-xc-fresh-background ];
              boot.tmp.cleanOnBoot = true;
              zramSwap.enable = true;
              networking.hostName = "composable-devnet";
              networking.firewall.allowedTCPPorts = [ 22 80 443 9988 9944 ];
              networking.domain = "";
              services.openssh.enable = true;
              services.caddy = {
                enable = true;
                virtualHosts."localhost".extraConfig = ''
                  respond "Hello, world!"
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
              users.users.root.openssh.authorizedKeys.keys = [
                "ssh-rsaAAAAB3NzaC1yc2EAAAADAQABAAABgQCXsAg4FHGlI7YNSM86z + nLpoLKEP8pln4HoqP7GcCYYScoq7OduiYw2uvedYJU2jA91i + Ep6l + mbzh + qxMkAyte80bIHeQbo5f47JXUJblKrveGaVb3mPKQJ7MYVgvw + WySwZcqQrEKbTo + bp6DAIYrCBvWkIdFss // DDMGbcyX3oF5gqZ5DJsiD4q89chY1uOtwIWdjLHe+9LMud7/OetRWwHpbk5i3BFPa3hQiixu48/TynPlzMk4tYSXnmelhzybtrI0j40/UQnpb8nHj/U0+ZAKG2OcNo+wBRuYWUdkoGZh3p5diy7oaPap+AmA+d25qrBnB9Xr2t1QIUoSP7I4+Bq7S4AVKLPQhU1vo8mA4TWzhuHQ4tz5f3iN5XOMKxoxG7wMgL5Y3xF60sqXWlsNJ4VDb285WxuFvEhtPzsRHtdkPMHLjDxePpwASxTkTgoha1FCapvuQOjE73B66urmB5vhpVE5MYb9S5tALEwX95oUJhscwjCcr+9oZFOn5k="
                "ssh-ed25519AAAAC3NzaC1lZDI1NTE5AAAAIM1+8YaFQCOY4D52kpPs8sgsDdfHFqjHIdWopedt4P7x"
              ];
            })

          ];
        };
      })) // { };
  };
}
