{ pkgs, packages, relaychainPort, parachainPort, packageName ? "devnet-dali"
, binaryName ? "devnet-dali-dev" }: {
  image = {
    contents = [ pkgs.coreutils packages.${packageName} ];
    enableRecommendedContents = true;
  };
  #devnet-dali-b
  service = {
    restart = "always";
    command = [
      "sh"
      "-c"
      ''
        ${packages.${packageName}}/bin/run-${binaryName}
      ''
    ];
    ports = [
      "${toString relaychainPort}:${toString relaychainPort}"
      "${toString parachainPort}:${toString parachainPort}"
    ];
    stop_signal = "SIGINT";
  };
}
