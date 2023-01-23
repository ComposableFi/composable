{ name, execCommands, configPathSource, configPathContainer, dependsOn
, restartPolicy }: {
  service = {
    image =
      "composablefi/hyperspace-dali:3ec1c34048981c399e6df4ee02cc0a6ce0320b25";
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
