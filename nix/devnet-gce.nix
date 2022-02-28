{ composable,
  polkadot,
  credentials,
  localtunnel,
}:
let
  gcefy-version = version:
    builtins.replaceStrings [ "." ] [ "-" ] version;
  domain = "composable-${composable.spec}-${gcefy-version composable.version}";
  domain-latest = "composable-${composable.spec}-latest";
  machine-name = "composable-devnet-${composable.spec}";
in {
  resources.gceNetworks.composable-devnet = credentials // {
    name = "composable-devnet-network";
    firewall = {
      allow-http = {
        targetTags = [ "http" ];
        allowed.tcp = [ 80 ];
      };
      allow-https = {
        targetTags = [ "https" ];
        allowed.tcp =  [ 443 ];
      };
    };
  };
  "${machine-name}" = { pkgs, resources, ... }:
    let
      devnet = pkgs.callPackage ./devnet.nix {
        inherit composable;
        inherit polkadot;
      };
    in {
      deployment = {
        targetEnv = "gce";
        gce = credentials // {
          machineName = machine-name;
          network = resources.gceNetworks.composable-devnet;
          region = "europe-central2-c";
          instanceType = "n2-standard-4";
          rootDiskSize = 50;
          tags = [
            "http"
            "https"
          ];
        };
      };
      networking.firewall.allowedTCPPorts = [ 80 443 ];
      systemd.services.composable-devnet = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Composable Devnet";
        serviceConfig = {
          Type = "simple";
          User = "root";
          ExecStart = "${devnet}/bin/run-${composable.spec}";
          Restart = "always";
          RuntimeMaxSec = "86400"; # 1 day lease period for rococo, restart it
        };
      };
      systemd.services.localtunnel-commit = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Local Tunnel Server";
        serviceConfig = {
          Type = "simple";
          User = "root";
          Restart = "always";
          ExecStart = "${localtunnel}/bin/lt --port 80 --subdomain ${domain}";
        };
      };
      systemd.services.localtunnel-latest = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Local Tunnel Server";
        serviceConfig = {
          Type = "simple";
          User = "root";
          Restart = "always";
          ExecStart = "${localtunnel}/bin/lt --port 80 --subdomain ${domain-latest}";
        };
      };
      services.nginx =
        let virtualConfig =
              let
                routify-nodes = prefix:
                  map (node: (node // {
                    name = prefix + node.name;
                  }));
                routified-composable-nodes =
                  routify-nodes "parachain/" composable.nodes;
                routified-polkadot-nodes =
                  routify-nodes "relaychain/" polkadot.nodes;
                routified-nodes =
                  routified-composable-nodes ++ routified-polkadot-nodes;
              in
                {
                  locations = builtins.foldl' (x: y: x // y) {} (map (node: {
                    "/${node.name}" = {
                      proxyPass = "http://127.0.0.1:${builtins.toString node.wsPort}";
                      proxyWebsockets = true;
                      extraConfig = ''
                        proxy_set_header Origin "";
                        proxy_set_header Host 127.0.0.1:${builtins.toString node.wsPort};
                      '';
                    };
                  }) routified-nodes);
                };
        in {
          enable = true;
          serverNamesHashBucketSize = 128;
          virtualHosts."${domain}.loca.lt" = virtualConfig;
          virtualHosts."${domain-latest}.loca.lt" = virtualConfig;
        };
    };
}
