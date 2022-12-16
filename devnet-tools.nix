{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, zombieTools, ... }: {
    _module.args.devnetTools = rec {

      mk-devnet = { lib, chain-spec }:
        let
          config = zombieTools.zombienet-rococo-local-composable-config {
            chain = "${chain-spec}";
          };
          script =
            zombieTools.writeZombienetShellApplication "devnet-${chain-spec}"
            config;

        in {
          inherit chain-spec script;
          config = zombieTools.zombienet-to-ops config;
        };

      getScript = script:
        "${pkgs.lib.getBin script}/bin/${pkgs.lib.getName script}";

      mk-devnet-container = { containerName, devNet, container-tools }:
        pkgs.lib.trace "Run Dali runtime on Composable node"
        pkgs.dockerTools.buildImage {
          name = containerName;
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ devNet pkgs.curl pkgs.websocat pkgs.glibc.bin ]
              ++ container-tools;
            pathsToLink = [ "/bin" ];
          };
          config = { Entrypoint = [ getScript devNet ]; };

          runAsRoot = ''
            mkdir --parents /usr/bin /tmp
            chown 777 /tmp
            ln --target-directory=/usr/bin /bin/ldd # https://github.com/napi-rs/napi-rs/issues/1335
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
            src = ./composablejs;
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
