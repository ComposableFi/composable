{ name, execCommands, configPathSource, configPathContainer, dependsOn
, restartPolicy, pkgs, packages, devnetTools }: {
  image = {
    contents = [ packages.hyperspace-dali ]
      ++ devnetTools.withBaseContainerTools;
    enableRecommendedContents = true;
  };
  service = {
    restart = restartPolicy;
    environment = { RUST_LOG = "debug"; };
    entrypoint = "${pkgs.lib.meta.getExe packages.hyperspace-dali}";
    command = execCommands;
    volumes = [{
      source = configPathSource;
      target = configPathContainer;
      type = "bind";
    }];
  } // pkgs.lib.optionalAttrs (dependsOn != null) { depends_on = dependsOn; };
}
