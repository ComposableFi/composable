{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = rec {
      docker-wipe-system = pkgs.writeShellScriptBin "docker-wipe-system" ''
        echo "Wiping all docker containers, images, and volumes";
        docker stop $(docker ps -q)
        docker system prune -f
        docker rmi -f $(docker images -a -q)
        docker volume prune -f
      '';
    };
  };
}
