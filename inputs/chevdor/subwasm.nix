{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, ... }: {
    packages = rec {
      subwasm = let
        src = pkgs.fetchFromGitHub {
          owner = "chevdor";
          repo = "subwasm";
          rev = "v0.19.0";
          hash = "sha256-Zo/wB1W3qp1cI+O0hAv0GfQ7tKXABboPY16ZNhyxmlk=";
        };
      in crane.stable.buildPackage {
        name = "subwasm";
        cargoArtifacts = crane.stable.buildDepsOnly {
          inherit src;
          doCheck = false;
          cargoTestCommand = "";
        };
        inherit src;
        doCheck = false;
        cargoTestCommand = "";
        meta = { mainProgram = "subwasm"; };
      };

      subwasm-release-body = let
        subwasm-call = runtime:
          builtins.readFile (pkgs.runCommand "subwasm-info" { }
            "${subwasm}/bin/subwasm info ${runtime}/lib/runtime.optimized.wasm | tail -n+2 > $out");
      in pkgs.writeTextFile {
        name = "release.txt";
        text = ''
          ## Runtimes
          ### Dali
          ```
          ${subwasm-call self'.packages.dali-runtime}
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
