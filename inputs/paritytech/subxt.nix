{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, ... }: {
      packages = rec {
        subxt = let
          name = "subxt";
          src = pkgs.fetchFromGitHub {
            owner = "paritytech";
            repo = name;
            rev = "44b1690170cab9a77e920a8bd713e247ddaf6254";
            hash = "sha256-L8SJgBh0C+w5OC4BJ0Mb0RrL/YkkHEVH9urvKef8K/w=";
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
