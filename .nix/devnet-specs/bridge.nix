{ pkgs, packages, ... }: {
  modules = [
    (let
      # more secured parachain allows only tokens originated on it self
      hyperspace = "dali-a-devnet-ibc-dali-b-devnet";

      more-secured-parachain = "dali-a-devnet";
      securedRelaychainPort = 9944;
      securedParachainPort = 9988;

      less-secured-parachain = "dali-b-devnet";
      lessSecuredRelaychainPort = 19944;
      lessSecuredParachainPort = 19988;

      hyperspaceMetrics = 31328;

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
          "${more-secured-parachain}" = mk-composable-container
            (import ../services/devnet-dali.nix {
              inherit pkgs;
              inherit packages;
              relaychainPort = securedRelaychainPort;
              parachainPort = securedParachainPort;
            });
          "${less-secured-parachain}" = mk-composable-container
            (import ../services/devnet-dali.nix {
              inherit pkgs;
              inherit packages;
              relaychainPort = lessSecuredRelaychainPort;
              parachainPort = lessSecuredParachainPort;
              packageName = "devnet-dali-b";
              binaryName = "devnet-dali-b";
            });
          "${hyperspace}" = mk-composable-container
            (import ../services/hyperspace.nix {
              inherit pkgs;
              inherit packages;
              metricsPort = hyperspaceMetrics;
            });
        };
      };
    })
  ];
}
