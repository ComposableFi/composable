{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    _module.args.devnetTools = rec {
      withDockerInDocker = with pkgs; [ docker docker-buildx docker-compose ];
      withUserContainerTools = with pkgs; [
        acl
        direnv
        home-manager
        cachix
        curl
        websocat
        patchelf
        file
      ];
      withBaseContainerTools = with pkgs; [
        bash
        coreutils
        procps
        findutils
        cacert
      ];
      withDevNetContainerTools = with pkgs;
        [ bottom gawk gnugrep less nettools nix ] ++ withBaseContainerTools
        ++ withUserContainerTools;

      buildDevnetImage = { name, devNet, container-tools }:
        pkgs.dockerTools.buildImage {
          inherit name;
          tag = "latest";
          copyToRoot = pkgs.buildEnv {
            name = "image-root";
            paths = [ devNet pkgs.glibc.bin ] ++ container-tools;
            pathsToLink = [ "/bin" ];
          };
          config = {
            Entrypoint =
              [ "${pkgs.lib.getBin devNet}/bin/${pkgs.lib.getName devNet}" ];
          };

          runAsRoot = ''
            mkdir --parents /usr/bin /tmp/composable-devnet && chown 777 /tmp
          '';
        };
    };
  };
}
