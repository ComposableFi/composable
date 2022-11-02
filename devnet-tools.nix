{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    _module.args.devnetTools = rec {

      mk-devnet = { lib, writeTextFile, writeShellApplication
        , useGlobalChainSpec ? true, polkadot-launch, composable-node
        , polkadot-node, chain-spec, network-config-path ?
          ./scripts/polkadot-launch/rococo-local-dali-dev.nix }:
        let
          original-config = (pkgs.callPackage network-config-path {
            polkadot-bin = polkadot-node;
            composable-bin = composable-node;
          }).result;

          patched-config = if useGlobalChainSpec then
            pkgs.lib.recursiveUpdate original-config {
              parachains = builtins.map
                (parachain: parachain // { chain = "${chain-spec}"; })
                original-config.parachains;
            }
          else
            original-config;

          config = pkgs.writeTextFile {
            name = "devnet-${chain-spec}-config.json";
            text = builtins.toJSON patched-config;
          };
        in {
          inherit chain-spec;
          parachain-nodes = builtins.concatMap (parachain: parachain.nodes)
            patched-config.parachains;
          relaychain-nodes = patched-config.relaychain.nodes;
          script = pkgs.writeShellApplication {
            name = "run-devnet-${chain-spec}";
            text = ''
              rm -rf /tmp/polkadot-launch
              ${polkadot-launch}/bin/polkadot-launch ${config} --verbose
            '';
          };
        };

      mk-bridge-devnet =
        { packages, polkadot-launch, composable-node, polkadot-node }:
        (pkgs.callPackage mk-devnet {
          inherit (packages) polkadot-launch composable-node polkadot-node;
          chain-spec = "dali-dev";
          network-config-path =
            ./scripts/polkadot-launch/bridge-rococo-local-dali-dev.nix;
          useGlobalChainSpec = false;
        });

      mk-devnet-container = { containerName, devNet, container-tools }:
        pkgs.lib.trace "Run Dali runtime on Composable node"
        pkgs.dockerTools.buildImage {
          name = containerName;
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ pkgs.curl pkgs.websocat ] ++ container-tools;
            pathsToLink = [ "/bin" ];
          };
          config = {
            Entrypoint = [ "${devNet}/bin/run-devnet-dali-dev" ];
            WorkingDir = "/home/polkadot-launch";
          };
          runAsRoot = ''
            mkdir -p /home/polkadot-launch /tmp
            chown 1000:1000 /home/polkadot-launch
            chmod 777 /tmp
          '';
        };

      mkDevnetInitializeScript = { polkadotUrl, composableUrl, parachainIds }:
        let
          lease-period-prolongator = pkgs.buildYarnPackage {
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.python3
              pkgs.nodePackages.node-gyp-build
              pkgs.nodePackages.node-gyp
              pkgs.nodePackages.typescript
            ];
            src = ./scripts/lease-period-prolongator;
            buildPhase = ''
              yarn
              ${pkgs.nodePackages.typescript}/bin/tsc
            '';
          };
          composablejs = pkgs.buildYarnPackage {
            nativeBuildInputs = [
              pkgs.pkg-config
              pkgs.python3
              pkgs.nodePackages.node-gyp-build
              pkgs.nodePackages.node-gyp
              pkgs.nodePackages.typescript
            ];
            src = ./composablejs;
            buildPhase = ''
              yarn
            '';
          };
        in pkgs.writeShellApplication {
          name = "qa-state-initialize";
          runtimeInputs = [ pkgs.nodejs ];
          text = ''
            PARACHAIN_ENDPOINT=${composableUrl} ${pkgs.nodejs}/bin/npm run --prefix ${composablejs} start -w packages/devnet-setup
            ${builtins.concatStringsSep "\n" (builtins.map (parachainId:
              "NODE_URL=${polkadotUrl} PARA_ID=${
                toString parachainId
              } ${pkgs.nodejs}/bin/node ${lease-period-prolongator}/dist/index.js")
              parachainIds)}
          '';
        };

    };
  };
}
