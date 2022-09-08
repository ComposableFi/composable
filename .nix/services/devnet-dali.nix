{ pkgs, packages, relaychain-port, parachain-port }: {
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
    ports =
      [ "${toString relaychain-port}:9944" "${toString parachain-port}:9988" ];
    stop_signal = "SIGINT";
  };
}
