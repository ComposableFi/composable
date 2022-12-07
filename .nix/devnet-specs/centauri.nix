{ pkgs, devnet-1, devnet-2, ... }: {
  modules = [
    (
      let
        configPathSource = "/tmp/config.toml";
        configPathContainer = "/tmp/config.toml";

        dependsOnCreateClient = {
              depends_on = {
                create-clients = {
                  condition = "service_completed_successfully";
                };
              };
        };
        dependsOnCreateConnection = {
              depends_on = {
                create-connection = {
                  condition = "service_completed_successfully";
                };
              };
        };
        devnetConfigs = [
          {
            containerName = "devnet-1";
            ports = [ 9944 9988 9989 9990 ];
            devnet = devnet-1;
          }
          {
            containerName = "devnet-2";
            ports = [ 29944 29988 29989 29990 ];
            devnet = devnet-2;
          }
        ];

        network-name = "composable_devnet";
        mkComposableContainer = container:
          container // {
            service = container.service // { networks = [ network-name ]; };
          };

        toService = devnetConfig: {
          name = devnetConfig.containerName;
          value = mkComposableContainer
            (import ../services/devnet.nix {
              inherit pkgs;
              devnet = devnetConfig.devnet;
              ports = map (port: { host = port; container = port; }) devnetConfig.ports;
            });
        };
      in
      {
        config = {
          project.name = "composable";
          networks."${network-name}" = { };

          services = builtins.listToAttrs (map toService devnetConfigs) // {
            "centauri" = mkComposableContainer (import ../services/centauri.nix
              [ 
                "create-clients" "--config" configPathContainer
              ] configPathSource configPathContainer null
            );

          } // {
            "centauri" = mkComposableContainer (import ../services/centauri.nix 
              [ 
                "create-connection" "--config" configPathContainer "--delay-period" "0"
              ] configPathSource configPathContainer dependsOnCreateClient
            );
          } // {
            "centauri" = mkComposableContainer (import ../services/centauri.nix
              [ 
                "create-clients" "--config" configPathContainer "--port-id" "transfer" "--version" "ics20-1"
                "--order" "unordered"
              ] configPathSource configPathContainer dependsOnCreateConnection
            );
          };
        };
      })
  ];
  inherit pkgs;
}

