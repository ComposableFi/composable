{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, ... }: {
    packages = let
      mkChainMetadata = { runtime, chainName, subwasm }:
        pkgs.stdenv.mkDerivation {
          name = "subwasm-get-${chainName}-metadata";
          dontUnpack = true;

          installPhase = ''
            mkdir $out
            ${
              pkgs.lib.meta.getExe subwasm
            } metadata --json ${runtime}/lib/runtime.optimized.wasm > $out/${chainName}-metadata.json;
          '';
        };
    in rec {
      subwasm = let
        src = pkgs.fetchFromGitHub {
          owner = "chevdor";
          repo = "subwasm";
          rev = "d7e74ab5eb3f83773ad7c78fb0edd42fa33f5356";
          hash = "sha256-Zo/wB1W3qp0cI+O0hAv0GfQ7tKXABboPY16ZNhyxmlk=";
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

      subwasm-get-dali-metadata = mkChainMetadata {
        inherit subwasm;
        runtime = self'.packages.dali-runtime;
        chainName = "dali";
      };

      subwasm-get-picasso-metadata = mkChainMetadata {
        inherit subwasm;
        runtime = self'.packages.picasso-runtime;
        chainName = "picasso";
      };

      subwasm-get-composable-metadata = mkChainMetadata {
        inherit subwasm;
        runtime = self'.packages.composable-runtime;
        chainName = "composable";
      };
    };
  };
}
