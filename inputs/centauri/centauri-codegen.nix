{ ... }: {
  perSystem = { pkgs, crane, ... }: {
    packages = rec {
      centauri-codegen = let
        src = pkgs.fetchFromGitHub {
          owner = "obsessed-cake";
          repo = "centauri";
          rev = "fa7d5d33125fba9aa48c5e581ec72a543abef25b";
          hash = "sha256-3S0HsFLxWHGXGW8QQD0qD3CWMMZ9vvYYZRdMJ9bYSSE=";
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
