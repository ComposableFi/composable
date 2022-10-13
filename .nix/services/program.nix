{ pkgs, command, program, environment ? { }, ports }: {
  image = {
    contents = [ pkgs.coreutils pkgs.cacert program ];
    enableRecommendedContents = true;
  };
  service = {
    inherit environment;
    restart = "always";
    command = [ "sh" "-c" command ];
    ports = builtins.map
      ({ host, container }: "${toString host}:${toString container}") ports;
    stop_signal = "SIGINT";
  };
}
