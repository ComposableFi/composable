{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }:
    let
      paritytech-zombienet-src = pkgs.fetchFromGitHub {
        owner = "dzmitry-lahoda-forks";
        repo = "zombienet";
        rev = "b9f089eb55a7cb1d4c12575b8323cb9b9fab4a60";
        hash = "sha256-EC6XKbcI+Is0RGlmC8WGjPqiFh9Ulf3bXDoVihtYqsU=";
      };
      paritytech-zombienet = pkgs.stdenv.mkDerivation {
        name = "zombienet";
        src = "${paritytech-zombienet-src}/javascript";
        buildInputs = with pkgs; [ nodejs ];
        nativeBuildInputs = with pkgs; [
          yarn
          nodejs
          python3
          pkg-config
          vips
          nodePackages.node-gyp-build
          nodePackages.node-gyp
          nodePackages.typescript
          coreutils
        ];
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
        # npm build fails with https://github.com/serokell/nix-npm-buildpackage/pull/62 (also i have updated to this PR...)
        # cannot use fix because of https://github.com/serokell/nix-npm-buildpackage/pull/54#issuecomment-1254908364
        __noChroot = true;
      };

      prelude = pkgs.callPackage ./default.nix { };
      runtimeDeps = with pkgs;
        [ coreutils bash procps git git-lfs ]
        ++ lib.optional stdenv.isLinux glibc.bin;

      writeZombienetShellApplication = name: config:
        pkgs.writeShellApplication rec {
          inherit name;
          runtimeInputs = [ pkgs.nodejs paritytech-zombienet ];
          text = ''
            export DEBUG="zombie*"
            printf '${builtins.toJSON config}' > /tmp/${name}.json
            cd ${paritytech-zombienet}            
            npm run zombie spawn /tmp/${name}.json
          '';
        };
    in with prelude; {
      _module.args.zombieTools = rec {
        inherit zombienet-rococo-local-composable-config
          writeZombienetShellApplication zombienet-to-ops;
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
