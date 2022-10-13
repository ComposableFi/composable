{ pkgs, port, frontend, app, ... }: {
  image.contents = [ pkgs.bash pkgs.coreutils ];
  service.useHostStore = true;
  service.command = [
    "bash"
    "-c"
    "${pkgs.miniserve}/bin/miniserve -p 8000 --spa --index index.html ${frontend}/${app}"
  ];
  service.ports = [ "${builtins.toString port}:8000" ];
  service.stop_signal = "SIGINT";
}
