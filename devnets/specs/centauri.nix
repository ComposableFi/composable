{ pkgs, devnet-a, devnet-b, devnetTools, packages, hyperspace-relay ? true, ...
}: {
  modules = [
    (let
      configPathSource = "/tmp/config.toml";
      configPathContainer = "/tmp/config.toml";

      dependsOnCreateClient = {
        hyperspace-create-clients = {
          condition = "service_completed_successfully";
        };
      };
      dependsOnCreateConnection = {
        hyperspace-create-connection = {
          condition = "service_completed_successfully";
        };
      };
      dependsOnCreateChannels = {
        hyperspace-create-channels = {
          condition = "service_completed_successfully";
        };
      };
      devnetConfigs = [
        {
          containerName = "devnet-a";
          ports = [ 9944 9988 9989 9990 ];
          devnet = devnet-a;
          networkName = network-a;
        }
        {
          containerName = "devnet-b";
          ports = [ 29944 29988 29989 29990 ];
          devnet = devnet-b;
          networkName = network-b;
        }
      ];

      network-a = "picasso-kusama-centauri";
      network-b = "composable-polkadot-centauri";
      mkComposableContainer = container: networks:
        container // {
          service = container.service // { inherit networks; };
        };

      toService = devnetConfig: {
        name = devnetConfig.containerName;
        value = mkComposableContainer (import ../services/devnet.nix {
          inherit pkgs devnetTools;
          devnet = devnetConfig.devnet;
          ports = map (port: {
            host = port;
            container = port;
          }) devnetConfig.ports;
        }) [ devnetConfig.networkName ];
      };
    in {
      config = {
        project.name = "composable";
        networks."${network-a}" = { };
        networks."${network-b}" = { };

        services = builtins.listToAttrs (map toService devnetConfigs) // {
          "hyperspace-create-clients" = mkComposableContainer
            (import ../services/centauri.nix {
              name = "hyperspace-create-clients";
              execCommands = [
                "create-clients"
                "--config"
                configPathContainer
                "--delay-period-seconds"
                "10"
              ];
              inherit configPathSource configPathContainer pkgs packages
                devnetTools;
              dependsOn = { };
              restartPolicy = "on-failure";
            }) [ network-a network-b ];

          "hyperspace-create-connection" = mkComposableContainer
            (import ../services/centauri.nix {
              name = "hyperspace-create-connection";
              execCommands = [
                "create-connection"
                "--config"
                configPathContainer
                "--delay-period-seconds"
                "10"
              ];
              inherit configPathSource configPathContainer pkgs packages
                devnetTools;
              dependsOn = dependsOnCreateClient;
              restartPolicy = "on-failure";
            }) [ network-a network-b ];

          "hyperspace-create-channels" = mkComposableContainer
            (import ../services/centauri.nix {
              name = "hyperspace-create-channel";
              execCommands = [
                "create-channel"
                "--config"
                configPathContainer
                "--port-id"
                "transfer"
                "--version"
                "ics20-1"
                "--order"
                "unordered"
                "--delay-period-seconds"
                "10"
              ];
              inherit configPathSource configPathContainer pkgs packages
                devnetTools;
              dependsOn = dependsOnCreateConnection;
              restartPolicy = "no";
            }) [ network-a network-b ];
        } // pkgs.lib.optionalAttrs hyperspace-relay {
          "hyperspace-relay" = mkComposableContainer
            (import ../services/centauri.nix {
              name = "hyperspace-relay";
              execCommands = [
                "relay"
                "--config"
                configPathContainer
                "--delay-period-seconds"
                "10"
              ];
              inherit configPathSource configPathContainer pkgs packages
                devnetTools;
              dependsOn = dependsOnCreateChannels;
              restartPolicy = "on-failure";
            }) [ network-a network-b ];
        };
      };
    })
  ];
  inherit pkgs;
}
