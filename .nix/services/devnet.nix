{ pkgs, devnet, ports }:
let getScript = script: "${pkgs.lib.getName script}";
in {
  image = {
    contents = [ pkgs.coreutils pkgs.glibc.bin pkgs.bash pkgs.procps devnet ];
    enableRecommendedContents = true;

  };
  service = {
    environment = { DEBUG = "zombie*"; };
    restart = "always";
    command =
      "sh -c 'mkdir -p /usr/bin /tmp &&  chown 777 /tmp && ln --target-directory=/usr/bin /bin/ldd  && ${
        getScript devnet
      }'";
    ports = builtins.map
      ({ host, container }: "${toString host}:${toString container}") ports;
    stop_signal = "SIGINT";
  };
}
