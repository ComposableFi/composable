{ pkgs, packages, relaychainPort, parachainPort }: {
  image = {
    contents = [ pkgs.coreutils packages.devnet-picasso ];
    enableRecommendedContents = true;
  };
  service = {
    restart = "always";
    command = [
      "sh"
      "-c"
      ''
        ${packages.devnet-picasso}/bin/run-devnet-picasso-dev
      ''
    ];
    ports =
      [ "${toString relaychainPort}:9955" "${toString parachainPort}:19988" ];
    stop_signal = "SIGINT";
  };
}
