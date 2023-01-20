{ ... }: {
  perSystem = { pkgs, crane, ... }: {
    packages = rec {
      centauri-codegen = let
        src = pkgs.fetchFromGitHub {
          owner = "obsessed-cake";
          repo = "centauri";
          rev = "050b4eb23aa221e7ebec1bd4c1663f784744e50f";
          hash = "sha256-klGg48in0VhqG/V4ZQ0t1v2vE429IlOXolREiCKczK8=";
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
