{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , subnix, ... }: {
      packages = rec {
        subxt = let
          name = "subxt";
          src = pkgs.fetchFromGitHub {
            owner = "paritytech";
            repo = name;
            rev = "059723e4313d91e8ca0bcd84b0dd7dd66686ca50";
            hash = "sha256-eEsb88f16Ug9h7JNkzwSTxJZEV5r4XmmzsTxTQGk+j8=";
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
