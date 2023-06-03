{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, ... }: {
      packages = rec {
        subxt = let
          name = "subxt";
          src = pkgs.fetchFromGitHub {
            owner = "paritytech";
            repo = name;
            rev = "fdc0d09ca3fb1c8a22cb5a05ba8178f024d0cda3";
            hash = "sha256-ZrrkpqcGHUlb3kVVrnmLpLhGbip+ikTwu6NEiNuX03A=";
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
      };
    };
}
