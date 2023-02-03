{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    _module.args.devnetTools = rec {
      withDockerInDocker = with pkgs; [ docker docker-buildx docker-compose ];
      withUserContainerTools = with pkgs; [
        acl
        direnv
        home-manager
        cachix
        curl
        websocat
      ];
      withBaseContainerTools = with pkgs; [ bash coreutils procps findutils ];
      withDevNetContainerTools = with pkgs;
        [ bottom gawk gnugrep less nettools nix ] ++ withBaseContainerTools
        ++ withUserContainerTools;

      getScript = script:
        "${pkgs.lib.getBin script}/bin/${pkgs.lib.getName script}";
      mk-devnet = { lib, writeTextFile, writeShellApplication
        , useGlobalChainSpec ? true, polkadot-launch, composable-node
        , polkadot-node, chain-spec, network-config-path ?
          ../scripts/polkadot-launch/rococo-local-dali-dev.nix }:
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

      mk-devnet-container = { containerName, devNet, container-tools }:
        pkgs.lib.trace "Run Dali runtime on Composable node"
        pkgs.dockerTools.buildImage {
          inherit name;
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ devNet pkgs.glibc.bin ] ++ container-tools;
            pathsToLink = [ "/bin" ];
          };
          config = {
            Entrypoint =
              [ "${pkgs.lib.getBin devNet}/bin/${pkgs.lib.getName devNet}" ];
          };

          runAsRoot = ''
            mkdir --parents /usr/bin /tmp && chown 777 /tmp
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
            src = ../scripts/lease-period-prolongator;
            buildPhase = ''
              export HOME=$(pwd)
              yarn --offline
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
            src = ../composablejs;
            buildPhase = ''
              export HOME=$(pwd)
              yarn --offline
            '';
          };
        in pkgs.writeShellApplication {
          name = "qa-state-initialize";
          runtimeInputs = [ pkgs.nodejs ];
          text = ''
            # TODO: outdated
            # PARACHAIN_ENDPOINT=${composableUrl} ${pkgs.nodejs}/bin/npm run --prefix ${composablejs} start -w packages/devnet-setup
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
