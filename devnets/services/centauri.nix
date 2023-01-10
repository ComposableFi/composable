{ name, execCommands, configPathSource, configPathContainer, dependsOn
, restartPolicy, pkgs, packages, devnetTools }: {
  image = {
    contents = [ packages.hyperspace-dali ]
      ++ devnetTools.withBaseContainerTools;
    enableRecommendedContents = true;
  };
  service = {
    restart = restartPolicy;
    environment = { RUST_LOG = "info"; };
    entrypoint = "${pkgs.lib.meta.getExe packages.hyperspace-dali}";
    command = execCommands;
    # should only be added if it's null
    depends_on = dependsOn;
  };
}
