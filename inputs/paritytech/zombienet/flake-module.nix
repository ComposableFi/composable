{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }:
    let
      prelude = pkgs.callPackage ./default.nix { };
      runtimeDeps = with pkgs;
        [ git git-lfs ] ++ devnetTools.withBaseContainerTools
        ++ lib.optional stdenv.isLinux glibc.bin;

      writeZombienetShellApplication = name: config:
        writeZombienet { inherit name config; };

      writeZombienet = { name, config, dir ? null }:
        pkgs.writeShellApplication rec {
          inherit name;
          runtimeInputs = with pkgs;
            [ nodejs zombienet.default ] ++ runtimeDeps;
          text = let dir-parameter = if dir != null then "--dir ${dir}" else "";
          in ''
            ACTIONS_RUNNER_DEBUG=''${ACTIONS_RUNNER_DEBUG:-false} 
            LEVEL=''${1:-error}
            if [[ $LEVEL = "debug" ]] || [[ $ACTIONS_RUNNER_DEBUG = 'true' ]] ;then
              export DEBUG="zombie*"
            fi
            mkdir --parents /tmp/composable-devnet
            printf '${
              builtins.toJSON config
            }' > /tmp/composable-devnet/${name}.json              
            zombienet spawn /tmp/composable-devnet/${name}.json ${dir-parameter} --force
          '';
        };
    in with prelude; {
      _module.args.zombieTools = rec {
        inherit zombienet-rococo-local-composable-config
          writeZombienetShellApplication writeZombienet;
        builder = prelude;
      };
      packages = { zombienet = pkgs.zombienet.default; };
    };
}
