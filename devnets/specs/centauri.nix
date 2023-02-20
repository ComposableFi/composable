{ pkgs, devnet-a, devnet-b, devnetTools, packages, ... }: {
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
          networkName = network-name;
        }
        {
          containerName = "devnet-b";
          ports = [ 29944 29988 29989 29990 ];
          devnet = devnet-b;
          networkName = network-name-2;
        }
      ];

      network-name = "composable_devnet";
      network-name-2 = "composable_devnet_2";
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
        networks."${network-name}" = { };
        networks."${network-name-2}" = { };

        services = builtins.listToAttrs (map toService devnetConfigs) // {
          "hyperspace-create-clients" = mkComposableContainer
            (import ../services/centauri.nix {
              name = "hyperspace-create-clients";
              execCommands =
                [ "create-clients" "--config" configPathContainer ];
              inherit configPathSource configPathContainer pkgs packages
                devnetTools;
              dependsOn = { };
              restartPolicy = "on-failure";
            }) [ network-name network-name-2 ];

          "hyperspace-create-connection" = mkComposableContainer
            (import ../services/centauri.nix {
              name = "hyperspace-create-connection";
              execCommands = [
                "create-connection"
                "--config"
                configPathContainer
                "--delay-period"
                "0"
              ];
              inherit configPathSource configPathContainer pkgs packages
                devnetTools;
              dependsOn = dependsOnCreateClient;
              restartPolicy = "no";
            }) [ network-name network-name-2 ];

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
              ];
              inherit configPathSource configPathContainer pkgs packages
                devnetTools;
              dependsOn = dependsOnCreateConnection;
              restartPolicy = "no";
            }) [ network-name network-name-2 ];

          "hyperspace-relay" = mkComposableContainer
            (import ../services/centauri.nix {
              name = "hyperspace-relay";
              execCommands = [ "relay" "--config" configPathContainer ];
              inherit configPathSource configPathContainer pkgs packages
                devnetTools;
              dependsOn = dependsOnCreateChannels;
              restartPolicy = "on-failure";
            }) [ network-name network-name-2 ];
        };
      };
    })
  ];
  inherit pkgs;
}
