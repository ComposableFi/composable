{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }: {
    packages =
      let
        packages = self'.packages;

        # for containers which are intended for testing, debug and development (including running isolated runtime)
        docker-in-docker = with pkgs; [ docker docker-buildx docker-compose ];
        containers-tools-minimal = with pkgs; [
          acl
          direnv
          home-manager
          cachix
        ];
        container-tools = with pkgs;
          [
            bash
            bottom
            coreutils
            findutils
            gawk
            gnugrep
            less
            nettools
            nix
            procps
          ] ++ containers-tools-minimal;
      in
      rec {
        # Dali devnet
        devnet-dali = (pkgs.callPackage devnetTools.mk-devnet {
          inherit (packages) polkadot-launch composable-node polkadot-node;
          chain-spec = "dali-dev";
        }).script;

        # Dali bridge devnet
        bridge-devnet-dali = (devnetTools.mk-bridge-devnet {
          inherit packages;
          inherit (packages) polkadot-launch composable-node polkadot-node;
        }).script;

        # Dali bridge devnet with mmr-polkadot
        bridge-mmr-devnet-dali = (devnetTools.mk-bridge-devnet {
          inherit packages;
          inherit (packages) polkadot-launch composable-node;
          polkadot-node = packages.mmr-polkadot-node;
        }).script;

        # Picasso devnet
        devnet-picasso = (pkgs.callPackage devnetTools.mk-devnet {
          inherit (packages) polkadot-launch composable-node polkadot-node;
          chain-spec = "picasso-dev";
        }).script;

        devnet-container = devnetTools.mk-devnet-container {
          inherit container-tools;
          containerName = "composable-devnet-container";
          devNet = packages.devnet-dali;
        };

        # Dali Bridge devnet container
        bridge-devnet-dali-container = devnetTools.mk-devnet-container {
          inherit container-tools;
          containerName = "composable-bridge-devnet-container";
          devNet = packages.bridge-devnet-dali;
        };

        # Dali Bridge devnet container with mmr-polkadot
        bridge-mmr-devnet-dali-container = devnetTools.mk-devnet-container {
          inherit container-tools;
          containerName = "composable-bridge-mmr-devnet-container";
          devNet = packages.bridge-mmr-devnet-dali;
        };

      };
  };
}
