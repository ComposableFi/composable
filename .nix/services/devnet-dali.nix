{ pkgs, packages, ... }: {
  image.contents = [ pkgs.bash pkgs.coreutils packages.devnet-dali ];
  service.useHostStore = true;
  service.command = [
    "bash"
    "-c"
    ''
      ${packages.devnet-dali}/bin/run-devnet-dali-dev
    ''
  ];
  service.network_mode = "host";
  # service.ports = [
  #   "9944:9944" # host:container
  # ];
  service.stop_signal = "SIGINT";
}
