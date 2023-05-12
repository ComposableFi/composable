{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, ... }: {
      packages = rec {
        subwasm = let
          name = "subwasm";
          src = pkgs.fetchFromGitHub {
            owner = "chevdor";
            repo = name;
            rev = "8c7c1457f203b056bb0c9d28a1d644a6047db30b";
            hash = "sha256-3c7sl6j3CfWCuHDhlKCkoyehXnghtaxe508rhYdLjDc=";
          };
        in crane.stable.buildPackage (subnix.subenv // {
          name = name;
          cargoArtifacts = crane.stable.buildDepsOnly (subnix.subenv // {
            inherit src;
            doCheck = false;
            cargoTestCommand = "";
            nativeBuildInputs = systemCommonRust.darwin-deps;
          });
          inherit src;
          cargoTestCommand = "";
          meta = { mainProgram = name; };
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
