{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , devnetTools, cargoTools, ... }:

    {
      packages = rec {
        devnet-picasso-image = devnetTools.buildDevnetImage {
          name = "devnet-picasso";
          container-tools = devnetTools.withDevNetContainerTools;
          devNet = self'.packages.zombienet-rococo-local-picasso-dev;
        };

        devnet-image = devnetTools.buildDevnetImage {
          name = "devnet";
          container-tools = devnetTools.withDevNetContainerTools;
          devNet = self'.packages.zombienet-rococo-local-picasso-dev;
        };

        devnet-xc-image = devnetTools.buildDevnetImage {
          name = "devnet-xc";
          container-tools = devnetTools.withDevNetContainerTools;
          devNet = self'.packages.devnet-xc-background;
        };
      };
    };
}

