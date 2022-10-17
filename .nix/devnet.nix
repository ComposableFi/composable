{ nixpkgs, devnet-dali, devnet-picasso, book, gce-input, rev }:
let
  region = "europe-central2-c";
  persistent-machine = let
    domain = "persistent.devnets.composablefinance.ninja";
    machine-name = "composable-persistent-devnet";
  in {
    "${machine-name}" = { pkgs, resources, ... }: {
      deployment = {
        targetEnv = "gce";
        gce = gce-input // {
          inherit region;
          machineName = machine-name;
          network = resources.gceNetworks.composable-devnet;
          instanceType = "n2-standard-4";
          rootDiskSize = 200;
          tags = [ "http" "https" ];
        };
      };
      nix = {
        enable = true;
        gc.automatic = true;
        settings = {
          auto-optimise-store = true;
          experimental-features = [ "nix-command" "flakes" ];
        };
        package = pkgs.nixUnstable;
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
      networking = { firewall.allowedTCPPorts = [ 80 443 ]; };
      virtualisation.docker.enable = true;
      systemd.services.devnet = {
        wantedBy = [ "multi-user.target" ];
        after = [ "network.target" ];
        description = "Composable Devnet";
        serviceConfig = {
          Type = "simple";
          User = "root";
          LimitNOFILE = 1048576;
          # If docker compose is killed prematurely, it will exit with 255.
          # Allow for arion to SIGKILL docker compose with success, hence avoiding an issue when deploying (deployment would fail if 255 is considered as error code).
          SuccessExitStatus = "255";
          ExecStart = "${
              pkgs.writeShellApplication {
                name = "run-devnet";
                runtimeInputs = [ pkgs.nixUnstable pkgs.git ];
                text =
                  "nix run github:ComposableFi/Composable/${rev}#devnet-persistent -L";
              }
            }/bin/run-devnet";
          Restart = "always";
        };
      };
      security.acme = {
        acceptTerms = true;
        defaults = { email = "hussein@composable.finance"; };
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
          # I agree, the hardcoded ports are a shame
        in mkDomain domain ({
          locations = proxyChain "dali" 9988 // proxyChain "dali/bob" 9989
            // proxyChain "dali/charlie" 9990 // proxyChain "rococo" 9944
            // proxyChain "karura" 9999 // proxyChain "statemine" 10008 // {
              "/" = { root = "${book}/book"; };
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
in builtins.foldl' (machines: devnet:
  let
    machine = import ./devnet-gce.nix {
      inherit region;
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
