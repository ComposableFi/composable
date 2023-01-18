{ ... }: {
  perSystem = { pkgs, crane, ... }: {
    packages = rec {
      centauri-codegen = let
        src = pkgs.fetchFromGitHub {
          owner = "ComposableFi";
          repo = "centauri";
          rev = "21aff7b812cf0d2821498af815f4069a04d1ce22";
          hash = "sha256-JUAbFxkdNU5ddWDVn0ZA3b2TL0vrVDEZS1RIT6RlFFk=";
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
