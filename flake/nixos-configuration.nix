{ self, withSystem, inputs, lib, options, flake-parts-lib, specialArgs, config
}:
let
  ip = "28.137.109.65";
  host = "static.28.137.109.65.clients.your-server.de";
in {

  perSystem = { config, self', inputs', pkgs, system, ... }:

    {
      apps = let name = "composable-devnet";
      in {
        composable-devnet-deploy = {
          type = "app";
          program = pkgs.writeShellApplication {
            runtimeInputs = [ pkgs.nixos-rebuild ];
            name = "composable-devnet-deploy";
            text = ''
              NIX_SSHOPTS="-i .ssh/id_rsa"         
              export NIX_SSHOPTS
              nixos-rebuild switch --fast --flake .#root  --target-host root@static.${ip}.clients.your-server.de            
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
          ports = [
            22
            80
            443
            9988
            9944
            10008
            29988
            29944
            30008
            26657
            36657
            8551
            30303
            591
            8008
            8080
            3500
            4000
            8081
            13000
          ];
          service = self'.packages.devnet-xc-cosmos-fresh-background;
        in self.inputs.nixpkgs.lib.nixosSystem {
          inherit system;
          modules = [
            ({ ... }: {
              imports =
                [ ./hardware-configuration.nix ./networking.nix ./host.nix ];
              system.stateVersion = "23.11";
              environment.systemPackages = with pkgs;
                with self'.packages;
                with pkgs;
                [ service helix process-compose caddy wget openssl cacert ]
                ++ devnetTools.withDevNetContainerTools;
              boot.tmp.cleanOnBoot = true;
              zramSwap.enable = true;
              networking.hostName = "composable-devnet";
              networking.firewall.allowedTCPPorts = ports;
              networking.domain = "";
              services.openssh.enable = true;

              security.acme = {
                acceptTerms = true;
                preliminarySelfsigned = true;
                defaults = { email = "dzmitry@lahoda.pro"; };
              };

              services.nginx = { enable = true; };
              services.nginx.virtualHosts = {
                "${host}" = {
                  addSSL = true;
                  enableACME = true;
                  root = "/var/www/default";

                  locations."/" = {
                    root = pkgs.runCommand "testdir" { } ''
                      mkdir "$out"
                      echo "CVM and MANTIS remote devnet" > "$out/index.html"
                    '';
                  };
                  locations."/centauri/" = {
                    proxyPass = "http://127.0.0.1:26657/";
                    proxyWebsockets = true;
                  };
                  locations."/osmosis/" = {
                    proxyPass = "http://127.0.0.1:36657/";
                    proxyWebsockets = true;
                  };
                };
              };

              systemd.services.composable-devnet = {
                wantedBy = [ "multi-user.target" ];
                after = [ "network.target" ];
                description = "composable-devnet";
                serviceConfig = {
                  Type = "simple";
                  User = "root";
                  ExecStart = "${pkgs.lib.meta.getExe service}";
                  Restart = "always";
                };
              };
            })

          ];
        };
      })) // { };
  };
}
