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
        cacert
        coreutils
        dasel
        findutils
        git
        git-lfs
        gnused
        procps
      ];
      withDevNetContainerTools = with pkgs;
        [ bottom gawk gnugrep less nettools nix self'.packages.bech32cli ]
        ++ withBaseContainerTools ++ withUserContainerTools;

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
            Env = [ "USER=actions-runner" ];
          };

          runAsRoot = ''
            #!${pkgs.runtimeShell}
            ${pkgs.dockerTools.shadowSetup}
            # so we add 2 potential runners, CI and Codespace just for convenience
            mkdir --parents /usr/bin /home/actions-runner /tmp/composable-devnet /home/vscode
            chown 777 /tmp 
            chown 777 /tmp/composable-devnet
            chown 777 /home/actions-runner
            chown 777 /home/vscode
            groupadd --system actions-runner
            useradd --system --gid actions-runner --groups root actions-runner
          '';
        };
    };
  };
}
