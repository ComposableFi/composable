{ pkgs, packages, config, ... }: {
  modules = [
    (let
      # more secured parachain allows only tokens originated on it self
      hyperspace = "dali-a-devnet-ibc-dali-b-devnet";

      inherit config;
      hyperspaceMetricsPort = 31328;

      network-name = "composable_bridge_devnet";
      mk-composable-container = container:
        container // {
          service = container.service // { networks = [ network-name ]; };
        };
    in {
      config = {
        project.name = "composable_xcvm_devnet";
        networks."${network-name}" = { };
        services = {
          "${config.moreSecuredParachain}" = mk-composable-container
            (import ../services/devnet-dali.nix {
              inherit pkgs;
              inherit packages;
              relaychainPort = config.moreSecuredRelaychainPort;
              parachainPort = config.moreSecuredParachainPort;
            });
          "${config.lessSecuredParachain}" = mk-composable-container
            (import ../services/devnet-dali.nix {
              inherit pkgs;
              inherit packages;
              relaychainPort = config.lessSecuredRelaychainPort;
              parachainPort = config.lessSecuredParachainPort;
              packageName = "devnet-dali-b";
              binaryName = "devnet-dali-b";
            });
          "${hyperspace}" = mk-composable-container
            (import ../services/hyperspace.nix {
              inherit pkgs;
              inherit packages;
              metricsPort = hyperspaceMetricsPort;
            });
        };
      };
    })
  ];
}
