{ pkgs, packages, metricsPort }: {
  image = {
    contents = [ pkgs.coreutils packages.hyperspace-template-default ];
    enableRecommendedContents = true;
  };
  service = {
    restart = "always";
    command = [
      "sh"
      "-c"
      ''
        ${pkgs.lib.meta.getExe packages.hyperspace-template-default}
      ''
    ];
    stop_signal = "SIGINT";
  };
}
