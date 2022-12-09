{ pkgs, devnet-1, devnet-2, ... }: {
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
        value = mkComposableContainer (import ../services/devnet.nix {
          inherit pkgs;
          devnet = devnetConfig.devnet;
          ports = map (port: {
            host = port;
            container = port;
          }) devnetConfig.ports;
        });
      };
    in {
      config = {
        project.name = "composable";
        networks."${network-name}" = { };

        services = builtins.listToAttrs (map toService devnetConfigs) // {
          "hyperspace-create-clients" = mkComposableContainer
            (import ../services/centauri.nix {
              name = "hyperspace-create-clients";
              execCommands =
                [ "create-clients" "--config" configPathContainer ];
              configPathSource = configPathSource;
              configPathContainer = configPathContainer;
              dependsOn = { };
              # for the first process in the dependency chain we set a restart policy so that it
              # restarts on failure. This is because the chain can be still unavailable (takes longer)
              # than hyperspace). In that case, restarting hyperspace with this create-clients command
              # is the cleanest option. Once it succeeds, the next commands won't have the same restart
              # policy, as they should not fail
              restartPolicy = "on-failure";
            });

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
              configPathSource = configPathSource;
              configPathContainer = configPathContainer;
              dependsOn = dependsOnCreateClient;
              restartPolicy = "no";
            });

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
              configPathSource = configPathSource;
              configPathContainer = configPathContainer;
              dependsOn = dependsOnCreateConnection;
              restartPolicy = "no";
            });
        };
      };
    })
  ];
  inherit pkgs;
}