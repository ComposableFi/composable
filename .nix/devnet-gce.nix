{ gce-input, book, devnet }:
let
  description = "What machines we will deploy on GCE";
  machine-name = "composable-devnet-${devnet.chain-spec}";
in {
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
  "${machine-name}" = { pkgs, resources, ... }: {
    deployment = {
      targetEnv = "gce";
      gce = gce-input // {
        machineName = machine-name;
        network = resources.gceNetworks.composable-devnet;
        region = "europe-central2-c";
        instanceType = "n2-standard-4";
        rootDiskSize = 200;
        tags = [ "http" "https" ];
      };
    };
    nix = {
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
      runtimeName = pkgs.lib.removeSuffix "-dev" devnet.chain-spec;
      domain = "${runtimeName}.devnets.composablefinance.ninja";
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
        locations = builtins.foldl' (x: y: x // y) {
          # TODO: make sure we build doc
          # "= /doc/" = {
          #   return = "301 https://${domain}/doc/composable/index.html";
          # };
          # "= /doc" = {
          #   return = "301 https://${domain}/doc/composable/index.html";
          # };
          # "/doc/" = { root = devnet.documentation; };
          "/" = { root = "${book}/book"; };
        } (map (node: {
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
      enableReload = true;
      recommendedOptimisation = true;
      recommendedGzipSettings = true;
      serverNamesHashBucketSize = 128;
      virtualHosts."${domain}" = virtualConfig;
    };
  };
}
