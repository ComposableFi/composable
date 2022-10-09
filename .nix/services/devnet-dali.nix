{ pkgs, devnet-dali, relaychainPort, parachainPort }: {
  image = {
    contents = [ pkgs.coreutils devnet-dali ];
    enableRecommendedContents = true;
  };
  service = {
    restart = "always";
    command = [
      "sh"
      "-c"
      ''
        ${devnet-dali}/bin/run-devnet-dali-dev
      ''
    ];
    ports =
      [ "${toString relaychainPort}:9944" "${toString parachainPort}:9988" ];
    stop_signal = "SIGINT";
  };
}

