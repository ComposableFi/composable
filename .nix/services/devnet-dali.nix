{ pkgs, packages, ... }: {
  image = {
    contents = [ pkgs.coreutils packages.devnet-dali ];
    enableRecommendedContents = true;
  };
  service = {
    restart = "always";
    command = [
      "sh"
      "-c"
      ''
        ${packages.devnet-dali}/bin/run-devnet-dali-dev
      ''
    ];
    network_mode = "host";
    stop_signal = "SIGINT";
  };
}
