{ pkgs, packages, ... }: {
  image = {
    contents = [ pkgs.coreutils pkgs.python3 packages.subsquid-processor ];
    enableRecommendedContents = true;
  };
  service = {
    restart = "always";
    network_mode = "host";
    command = [
      "sh"
      "-c"
      ''
        ${packages.subsquid-processor}/bin/run-subsquid-processor
      ''
    ];
    environment = {
      DB_PORT = 23798;
      DB_NAME = "squid";
      DB_PASS = "squid";
    };
  };
}
