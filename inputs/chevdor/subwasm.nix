{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, ... }: {
      packages = rec {
        subwasm = let
          version = "v0.19.0";
          src = pkgs.fetchFromGitHub {
            owner = "chevdor";
            repo = "subwasm";
            rev = "refs/tags/${version}";
            hash = "sha256-DCPpGn0CrngmDP1QuK+Y9hffoD04yS+FenjQ5d/f49U=";
          };
        in crane.stable.buildPackage (subnix.subenv // {
          name = "subwasm";
          cargoArtifacts = crane.stable.buildDepsOnly (subnix.subenv // {
            inherit src;
            doCheck = false;
            cargoTestCommand = "";
            nativeBuildInputs = systemCommonRust.darwin-deps;
          });
          inherit src version;
          cargoTestCommand = "";
          meta = { mainProgram = "subwasm"; };
        });

        subwasm-release-body = let
          subwasm-call = runtime:
            builtins.readFile (pkgs.runCommand "subwasm-info" { }
              "${subwasm}/bin/subwasm info ${runtime}/lib/runtime.optimized.wasm | tail -n+2 > $out");
        in pkgs.writeTextFile {
          name = "release.txt";
          text = ''
            ## Runtimes
            ```
            ### Picasso
            ```
            ${subwasm-call self'.packages.picasso-runtime}
            ```
            ### Composable
            ```
            ${subwasm-call self'.packages.composable-runtime}
            ```
          '';
        };
      };
    };
}
