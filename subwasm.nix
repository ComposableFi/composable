{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, ... }: {
    packages = rec {
      subwasm = let
        src = pkgs.fetchFromGitHub {
          owner = "chevdor";
          repo = "subwasm";
          rev = "4d4d789326d65fc23820f70916bd6bd6f499bd0a";
          hash = "sha256-+/yqA6lP/5qyMxZupmaYBCRtbw2MFMBSgkmnxg261P8=";
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

      mkChainMetadata = { runtime, chainName }:
        pkgs.stdenv.mkDerivation {
          name = "subwasm-get-${chainName}-metadata";
          dontUnpack = true;

          installPhase = ''
            mkdir $out
            ${pkgs.lib.meta.getExe subwasm} metadata --json ${runtime}/lib/runtime.optimized.wasm > $out/${chainName}-metadata.json;
          '';
        };

      subwasm-get-dali-metadata = mkChainMetadata {
        runtime = self'.packages.dali-runtime;
        chainName = "dali";
      };

      subwasm-get-picasso-metadata = mkChainMetadata {
        runtime = self'.packages.picasso-runtime;
        chainName = "picasso";
      };

      subwasm-get-composable-metadata = mkChainMetadata {
        runtime = self'.packages.composable-runtime;
        chainName = "composable";
      };
    };
  };
}
