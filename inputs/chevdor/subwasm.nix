{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, ... }: {
      packages = rec {
        subwasm = let
          name = "subwasm";
          src = pkgs.fetchFromGitHub {
            owner = "chevdor";
            repo = name;
            rev = "2815d4f4a60918339afc9a48804786f6c70b22f6";
            hash = "sha256-Ex18cH+lxYQuSad2w9GM1eHX8TWNHyMTOBCKTzyJ/YY=";
          };
        in crane.stable.buildPackage (subnix.subenv // {
          name = name;
          pname = name;
          cargoArtifacts = crane.stable.buildDepsOnly (subnix.subenv // {
            inherit src;
            pname = name;
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
