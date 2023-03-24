{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }:
    let
      prelude = pkgs.callPackage ./default.nix { };
      runtimeDeps = with pkgs;
        [ git git-lfs ] ++ devnetTools.withBaseContainerTools
        ++ lib.optional stdenv.isLinux glibc.bin;

      writeZombienetShellApplication = name: config:
        pkgs.writeShellApplication rec {
          inherit name;
          runtimeInputs = with pkgs;
            [ nodejs zombienet.default ] ++ runtimeDeps;
          text = ''
            ACTIONS_RUNNER_DEBUG=''${ACTIONS_RUNNER_DEBUG:-false} 
            LEVEL=''${1:-error}
            if [[ $LEVEL = "debug" ]] || [[ $ACTIONS_RUNNER_DEBUG = 'true' ]] ;then
              export DEBUG="zombie*"
            fi
            if [[ -d /tmp ]];
            then 
              echo "using /tmp"
            else
              mkdir --parents /tmp && chown 777 /tmp
            fi               
            printf '${builtins.toJSON config}' > /tmp/${name}.json
            zombienet spawn /tmp/${name}.json            
          '';
        };
    in with prelude; {
      _module.args.zombieTools = rec {
        inherit zombienet-rococo-local-composable-config
          writeZombienetShellApplication zombienet-to-ops;
        builder = prelude;
      };
    };
}
