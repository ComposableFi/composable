{ name, execCommands, configPathSource, configPathContainer, dependsOn
, restartPolicy, pkgs, packages, devnetTools, RUST_LOG ? "trace" }: {
  image = {
    contents = [ packages.hyperspace-dali ]
      ++ devnetTools.withBaseContainerTools;
    enableRecommendedContents = true;
  };
  service = {
    restart = restartPolicy;
    environment = { inherit RUST_LOG; };
    entrypoint = "${pkgs.lib.meta.getExe packages.hyperspace-dali}";
    command = execCommands;
    volumes = [{
      source = configPathSource;
      target = configPathContainer;
      type = "bind";
    }];
  } // pkgs.lib.optionalAttrs (dependsOn != null) { depends_on = dependsOn; };
}
