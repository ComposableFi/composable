{ name, execCommands, dependsOn, restartPolicy, pkgs, packages, devnetTools
, singleFileWriteMounts }: {
  image = {
    contents = [ packages.hyperspace-composable-rococo-picasso-rococo ]
      ++ devnetTools.withBaseContainerTools;
    enableRecommendedContents = true;
  };
  service = {
    restart = restartPolicy;
    environment = {
      RUST_LOG =
        "trace,soketto=debug,tracing::span=debug,mio::poll=debug,trie=debug";
    };
    entrypoint = "${pkgs.lib.meta.getExe
      packages.hyperspace-composable-rococo-picasso-rococo}";
    command = execCommands;
    volumes = builtins.map ({ _1, _2} : {
      source = _1;
      target = _2;
      type = "bind";
    }) singleFileWriteMounts;
  } // pkgs.lib.optionalAttrs (dependsOn != null) { depends_on = dependsOn; };
}
