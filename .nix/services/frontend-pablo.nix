{ pkgs, packages, ... }: {
  image.contents = [ pkgs.bash pkgs.coreutils packages.devnet-dali ];
  service.useHostStore = true;
  service.command = [
    "bash"
    "-c"
    ''
      ${packages.frontend-pablo-server}/bin/frontend-pablo-server
    ''
  ];
  service.ports = [ "8002:8002" ];
  service.stop_signal = "SIGINT";
}
