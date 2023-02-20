{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, devnetTools, ... }:
    let
      version = "v1.3.34";
      paritytech-zombienet-src = pkgs.fetchgit {
        url = "https://github.com/paritytech/zombienet.git";
        rev = "refs/tags/${version}";
        sha256 = "sha256-69kOjoVAldz39h0MBzO0o19RtoWeNavCWg/JjqCf2fE=";
        fetchSubmodules = true;
      };
      paritytech-zombienet = pkgs.stdenv.mkDerivation {
        name = "zombienet";
        inherit version;
        src = "${paritytech-zombienet-src}/javascript";
        buildInputs = with pkgs; [ nodejs ];
        nativeBuildInputs = with pkgs;
          [
            yarn
            nodejs
            python3
            nodePackages.node-gyp-build
            nodePackages.node-gyp
            nodePackages.typescript

            vips
            pkg-config
          ] ++ devnetTools.withBaseContainerTools;
        buildPhase = ''
          mkdir home
          export HOME=$PWD/home
          npm install
          npm run build
        '';
        installPhase = ''
          mkdir --parents $out
          cp . $out --recursive
        '';
        # https://app.clickup.com/t/3w8y83f
        __noChroot = true;
      };

      prelude = pkgs.callPackage ./default.nix { };
      runtimeDeps = with pkgs;
        [ git git-lfs ] ++ devnetTools.withBaseContainerTools
        ++ lib.optional stdenv.isLinux glibc.bin;

      writeZombienetShellApplication = name: config:
        pkgs.writeShellApplication rec {
          inherit name;
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet ] ++ runtimeDeps;
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
            cd ${paritytech-zombienet}            
            npm run zombie spawn /tmp/${name}.json
          '';
        };
    in with prelude; {
      _module.args.zombieTools = rec {
        inherit zombienet-rococo-local-composable-config
          writeZombienetShellApplication zombienet-to-ops;
        builder = prelude;
      };
      packages = rec {
        inherit paritytech-zombienet;

        zombienet = pkgs.writeShellApplication {
          name = "zombienet";
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet ] ++ runtimeDeps;
          text = ''
            cd ${paritytech-zombienet}
            npm run zombie
          '';
        };
      };

      apps = rec {
        zombienet = {
          type = "app";
          program = self'.packages.zombienet;
        };
      };
    };
}
