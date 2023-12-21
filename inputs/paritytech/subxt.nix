{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, ... }: {
      packages = rec {
        subxt = let
          name = "subxt";
          src = pkgs.fetchFromGitHub {
            owner = "paritytech";
            repo = name;
            rev = "e40a8629e279e80a7fbb56ff553a430a36612956";
            hash = "sha256-fMCy1QAb8rdgQesRqbNCEh6lqEgf7ZsVhYdGvctjbQU";
          };
        in crane.nightly.buildPackage (rec {
          inherit name;
          pname = "subxt-cli";
          nativeBuildInputs = systemCommonRust.darwin-deps;
          cargoArtifacts = crane.nightly.buildDepsOnly ({
            doCheck = false;
            inherit name;
            inherit src;
            cargoTestCommand = "";
            nativeBuildInputs = systemCommonRust.darwin-deps;
          });
          doCheck = false;
          inherit src;
          cargoBuildCommand = "cargo build --release --package ${pname}";
          cargoTestCommand = "";
          meta = { mainProgram = name; };
        });
      };
    };
}
