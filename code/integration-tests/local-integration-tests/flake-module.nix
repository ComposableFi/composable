{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , cargoTools, ... }: {
      packages = {
        check-picasso-integration-tests = crane.nightly.cargoBuild
          (systemCommonRust.common-attrs // {
            src = cargoTools.rustSrc;
            pname = "picasso-local-integration-tests";
            doInstallCargoArtifacts = false;
            cargoArtifacts = self'.packages.common-test-deps;
            cargoBuildCommand = "cargo test --package local-integration-tests";
            cargoExtraArgs =
              "--features=local-integration-tests,picasso,std --no-default-features --verbose";
          });
        check-dali-integration-tests = crane.nightly.cargoBuild
          (systemCommonRust.common-attrs // {
            src = cargoTools.rustSrc;
            pname = "dali-local-integration-tests";
            doInstallCargoArtifacts = false;
            cargoArtifacts = self'.packages.common-test-deps;
            cargoBuildCommand = "cargo test --package local-integration-tests";
            cargoExtraArgs =
              "--features=local-integration-tests,dali,std --no-default-features --verbose";
          });
      };
    };
}
