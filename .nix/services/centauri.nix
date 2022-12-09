{ name, execCommands, configPathSource, configPathContainer, dependsOn
, restartPolicy }: {
  service = {
    image =
      "composablefi/composable-centauri:d8fdb31227a221a794f86c5bba4a8127cf8e71d5";
    restart = restartPolicy;
    volumes = [{
      type = "bind";
      source = configPathSource;
      target = configPathContainer;
    }];
    environment = { RUST_LOG = "info"; };
    command = execCommands;
    # should only be added if it's null
    depends_on = dependsOn;
  };
}