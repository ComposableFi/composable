{ pkgs, packages, ... }: {
  image.contents = [ pkgs.bash pkgs.coreutils packages.devnet-dali ];
  service.useHostStore = true;
  service.command = [
    "bash"
    "-c"
    ''
      ${packages.frontend-picasso-server}/bin/frontend-picasso-server
    ''
  ];
  service.ports = [ "8003:8003" ];
  service.stop_signal = "SIGINT";
}
