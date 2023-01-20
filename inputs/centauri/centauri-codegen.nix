{ ... }: {
  perSystem = { pkgs, crane, ... }: {
    packages = rec {
      centauri-codegen = let
        src = pkgs.fetchFromGitHub {
          owner = "obsessed-cake";
          repo = "centauri";
          rev = "83d7cffce6032cc12ac44fd718798069dd966c45";
          hash = "sha256-nnhMKuy4fTr04aZXwVw74gnNi1X/+baqkmMtymZeRZY=";
        };
      in crane.stable.buildPackage {
        name = "centauri-codegen";
        cargoArtifacts = crane.stable.buildDepsOnly {
          inherit src;
          doCheck = false;
          cargoExtraArgs = "-p codegen";
          cargoTestCommand = "";
        };
        inherit src;
        doCheck = false;
        cargoExtraArgs = "-p codegen";
        cargoTestCommand = "";
        meta = { mainProgram = "codegen"; };
      };
    };
  };
}
