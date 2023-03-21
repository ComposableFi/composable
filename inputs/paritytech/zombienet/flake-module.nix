{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }:
    let
      # paritytech-zombienet = pkgs.stdenv.mkDerivation {
      #   name = "zombienet";
      #   inherit version;
      #   src = "${paritytech-zombienet-src}/javascript";
      #   buildInputs = with pkgs; [ nodejs ];
      #   nativeBuildInputs = with pkgs;
      #     [
      #       yarn
      #       nodejs
      #       python3
      #       nodePackages.node-gyp-build
      #       nodePackages.node-gyp
      #       nodePackages.typescript

      #       vips
      #       pkg-config
      #     ] ++ devnetTools.withBaseContainerTools;
      #   buildPhase = ''
      #     mkdir home
      #     export HOME=$PWD/home
      #     npm install
      #     npm run build
      #   '';
      #   installPhase = ''
      #     mkdir --parents $out
      #     cp . $out --recursive
      #   '';
      #   # https://app.clickup.com/t/3w8y83f
      #   __noChroot = true;
      # };

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
