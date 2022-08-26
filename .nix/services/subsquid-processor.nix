{pkgs, packages, ...}:
{
  image.contents = [ pkgs.bash pkgs.python3 pkgs.coreutils ];
  service = {
    restart = "always";
    network_mode = "host";
    useHostStore = true;
    command = [
      "bash"
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