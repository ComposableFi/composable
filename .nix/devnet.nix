{ nixpkgs, devnet-dali, devnet-picasso, book, gce-input, frontend }:
let
  persistent-machine = let
    pabloPort = 8110;
    picassoPort = 8111;
    domain = "persistent.devnets.composablefinance.ninja";
  in import ./devnet-gce.nix {
    inherit gce-input;
    inherit book;
    inherit domain;
    devnet = devnet-dali;
    disk-size = 200;
    machine-name = "composable-persistent-devnet";
    # Overwrite the devnet to avoid restarting for lease period (runtimemaxsec)
    extra-services = { pkgs, ... }: {
      systemd.services.composable-devnet = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Composable Devnet";
        serviceConfig = {
          Type = "simple";
          User = "root";
          ExecStart =
            "${devnet-dali.script}/bin/run-devnet-${devnet-dali.chain-spec}";
          Restart = "always";
        };
      };
      systemd.services.pablo = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Composable Pablo Frontend";
        serviceConfig = {
          Type = "simple";
          User = "root";
          ExecStart = "${pkgs.miniserve}/bin/miniserve -p ${
              builtins.toString pabloPort
            } --spa --index index.html ${frontend}/pablo";
          Restart = "always";
        };
      };
      systemd.services.picasso = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Composable Picasso Frontend";
        serviceConfig = {
          Type = "simple";
          User = "root";
          ExecStart = "${pkgs.miniserve}/bin/miniserve -p ${
              builtins.toString picassoPort
            } --spa --index index.html ${frontend}/picasso";
          Restart = "always";
        };
      };
    };
    extra-nginx-hosts = args: {
      "pablo.${domain}" = {
        enableACME = true;
        forceSSL = true;
        locations."/" = {
          proxyPass = "http://127.0.0.1:${builtins.toString pabloPort}/";
        };
      };
      "picasso.${domain}" = {
        enableACME = true;
        forceSSL = true;
        locations."/" = {
          proxyPass = "http://127.0.0.1:${builtins.toString picassoPort}/";
        };
      };
    };
  };
in builtins.foldl' (machines: devnet:
  let
    machine = import ./devnet-gce.nix {
      inherit gce-input;
      inherit devnet;
      inherit book;
      disk-size = 200;
      machine-name = "composable-devnet-${devnet.chain-spec}";
      domain = let prefix = nixpkgs.lib.removeSuffix "-dev" devnet.chain-spec;
      in "${prefix}.devnets.composablefinance.ninja";
    };
  in machines // machine) ({
    inherit nixpkgs;
    network = {
      description = "Composable Devnet";
      storage.legacy = { };
    };
  } // persistent-machine) [ devnet-dali devnet-picasso ]
