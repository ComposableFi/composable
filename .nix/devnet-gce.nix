{ region, gce-input, book, devnet, disk-size, machine-name, domain
, extra-gce ? (_args: { }), extra-services ? (_args: { })
, extra-nginx-root ? (_args: { }), extra-nginx ? (_args: { })
, extra-nginx-virtual ? (_args: { }), extra-nginx-hosts ? (_args: { }) }: {
  resources.gceNetworks.composable-devnet = gce-input // {
    name = "composable-devnet-network";
    firewall = {
      allow-http = {
        targetTags = [ "http" ];
        allowed.tcp = [ 80 ];
      };
      allow-https = {
        targetTags = [ "https" ];
        allowed.tcp = [ 443 ];
      };
    };
  };
  "${machine-name}" = args@{ resources, ... }:
    ({
      deployment = {
        targetEnv = "gce";
        gce = (gce-input // {
          inherit region;
          machineName = machine-name;
          network = resources.gceNetworks.composable-devnet;
          instanceType = "n2-standard-4";
          rootDiskSize = disk-size;
          tags = [ "http" "https" ];
        }) // (extra-gce args);
      };
      nix = {
        enable = true;
        gc.automatic = true;
        settings = { auto-optimise-store = true; };
      };
      networking.firewall.allowedTCPPorts = [ 80 443 ];
      systemd.services.composable-devnet = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Composable Devnet";
        serviceConfig = {
          Type = "simple";
          User = "root";
          ExecStart = "${devnet.script}/bin/run-devnet-${devnet.chain-spec}";
          Restart = "always";
          RuntimeMaxSec = "86400"; # 1 day lease period for rococo, restart it
        };
      };
      security.acme = {
        acceptTerms = true;
        defaults = { email = "hussein@composable.finance"; };
      };
      services.nginx = let
        virtualConfig = let
          routify-nodes = prefix:
            map (node: (node // { name = prefix + node.name; }));
          routified-composable-nodes =
            routify-nodes "parachain/" devnet.parachain-nodes;
          routified-polkadot-nodes =
            routify-nodes "relaychain/" devnet.relaychain-nodes;
          routified-nodes = routified-composable-nodes
            ++ routified-polkadot-nodes;
        in {
          enableACME = true;
          forceSSL = true;
          locations = builtins.foldl' (x: y: x // y)
            ({ "/" = { root = "${book}/book"; }; } // (extra-nginx-root args))
            (map (node: {
              "/${node.name}" = {
                proxyPass = "http://127.0.0.1:${builtins.toString node.wsPort}";
                proxyWebsockets = true;
                extraConfig = ''
                  proxy_set_header Origin "";
                  proxy_set_header Host 127.0.0.1:${
                    builtins.toString node.wsPort
                  };
                '';
              };
            }) routified-nodes);
        } // (extra-nginx-virtual args);
      in {
        enable = true;
        enableReload = true;
        recommendedOptimisation = true;
        recommendedGzipSettings = true;
        serverNamesHashBucketSize = 128;
        virtualHosts = {
          "${domain}" = virtualConfig;
        } // (extra-nginx-hosts args);
      } // (extra-nginx args);
    } // (extra-services args));
}
