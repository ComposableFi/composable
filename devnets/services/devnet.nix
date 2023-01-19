{ pkgs, devnet, ports, devnetTools }: {
  image = {
    contents = [ devnet ] ++ devnetTools.withBaseContainerTools;
    enableRecommendedContents = true;
  };
  service = {
    restart = "always";
    command = [ "sh" "-c" "${pkgs.lib.meta.getExe devnet}" ];
    ports = builtins.map
      ({ host, container }: "${toString host}:${toString container}") ports;
    stop_signal = "SIGINT";
  };
}
