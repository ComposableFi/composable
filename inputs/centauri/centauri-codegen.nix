{ ... }: {
  perSystem = { pkgs, crane, ... }: {
    packages = rec {
      centauri-codegen = let
        src = pkgs.fetchFromGitHub {
          owner = "ComposableFi";
          repo = "centauri";
          rev = "172e9cadff09db00b91f973cbbdce3f9f9a0eb05";
          hash = "sha256-B41MACZ6eMhxWfluKdcQ43b8Eicj/Uy4PBUewnXf/9k=";
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
