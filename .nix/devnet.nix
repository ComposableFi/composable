{ nixpkgs, devnet-dali, devnet-picasso, gce-input, docs, rev, domainSuffix
, cerificateEmail }:
let
  region = "europe-central2-c";

  mkPersistentMachine = { domain, machine-name, ip, package, chain }: {
    "${machine-name}" = { pkgs, resources, ... }: {
      deployment = {
        targetEnv = "gce";
        gce = gce-input // {
          inherit region;
          machineName = machine-name;
          network = resources.gceNetworks.composable-devnet;
          instanceType = "n2-standard-8";
          rootDiskSize = 500;
          tags = [ "http" "https" ];
          ipAddress = ip;
        };
      };
      nix = {
        enable = true;
        package = pkgs.nix;
        gc.automatic = true;
        settings = {
          auto-optimise-store = true;
          experimental-features = [ "nix-command" "flakes" ];
        };
        useSandbox = "relaxed";
        binaryCaches = [
          "https://nix-community.cachix.org/"
          "https://composable-community.cachix.org/"
        ];
        binaryCachePublicKeys = [
          "nix-community.cachix.org-1:mB9FSh9qf2dCimDSUo8Zy7bkq5CX+/rkCWyvRCYg3Fs="
          "composable-community.cachix.org-1:GG4xJNpXJ+J97I8EyJ4qI5tRTAJ4i7h+NK2Z32I8sK8="
        ];
      };
      networking = {
        firewall.allowedTCPPorts = [ 80 443 ];
        dhcpcd.denyInterfaces = [ "veth*" "docker0" "br-*" ];
      };
      virtualisation.docker = { enable = true; };
      systemd.services.devnet = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Composable Devnet";
        serviceConfig = {
          Type = "simple";
          User = "root";
          LimitNOFILE = 1048576;
          ExecStart = "${
              pkgs.writeShellApplication {
                name = "start";
                runtimeInputs = [ pkgs.nix pkgs.git ];
                text =
                  "nix run github:ComposableFi/Composable/${rev}#${package} -L";
              }
            }/bin/start";
        };
      };
      security.acme = {
        acceptTerms = true;
        defaults = { email = cerificateEmail; };
      };
      services.journald = {
        extraConfig = ''
          SystemMaxUse=100M
        '';
      };
      services.nginx = {
        enable = true;
        enableReload = true;
        recommendedOptimisation = true;
        recommendedGzipSettings = true;
        serverNamesHashBucketSize = 128;
        virtualHosts = let
          proxyChain = name: port: {
            "/chain/${name}" = {
              proxyPass = "http://127.0.0.1:${builtins.toString port}";
              proxyWebsockets = true;
              extraConfig = ''
                proxy_set_header Origin "";
                proxy_set_header Host 127.0.0.1:${builtins.toString port};
              '';
            };
          };
          mkDomain = name: attrs: {
            "${name}" = {
              enableACME = true;
              forceSSL = true;
            } // attrs;
          };
        in mkDomain domain ({
          locations = proxyChain chain 9988 // proxyChain "${chain}/bob" 9989
            // proxyChain "${chain}/charlie" 9990 // proxyChain "rococo" 9944
            // proxyChain "karura" 9999 // proxyChain "statemine" 10008 // {
              "/" = { root = "${docs}/"; };
              "/subsquid/" = { proxyPass = "http://127.0.0.1:4350/"; };
              "/price-feed/" = { proxyPass = "http://127.0.0.1:8003/"; };
            };
        }) // mkDomain "pablo.${domain}" ({
          locations."/" = { proxyPass = "http://127.0.0.1:8001/"; };
        }) // mkDomain "picasso.${domain}" ({
          locations."/" = { proxyPass = "http://127.0.0.1:8002/"; };
        });
      };
    };
  };

  dali-persistent-machine = mkPersistentMachine {
    domain = "persistent.${domainSuffix}";
    machine-name = "composable-persistent-devnet";
    ip = "persistent-devnet-ip";
    package = "devnet-dali-persistent";
    chain = "dali";
  };

  picasso-persistent-machine = mkPersistentMachine {
    domain = "persistent.picasso.${domainSuffix}";
    machine-name = "picasso-composable-persistent-devnet";
    ip = "picasso-persistent-devnet-ip";
    package = "devnet-picasso-persistent";
    chain = "picasso";
  };

in builtins.foldl' (machines: devnet:
  let
    machine = import ./devnet-gce.nix {
      inherit region cerificateEmail gce-input devnet docs;
      disk-size = 200;
      machine-name = "composable-devnet-${devnet.chain-spec}";
      domain = let prefix = nixpkgs.lib.removeSuffix "-dev" devnet.chain-spec;
      in "${prefix}.${domainSuffix}";
    };
  in machines // machine) ({
    inherit nixpkgs;
    network = {
      description = "Composable Devnet";
      storage.legacy = { };
    };
  } // (dali-persistent-machine // picasso-persistent-machine)) [
    devnet-dali
    devnet-picasso
  ]
