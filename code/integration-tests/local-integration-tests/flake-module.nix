{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust
    , cargoTools, ... }: {
      packages = let
        checkIntegrationTests = { chainName }:
          crane.nightly.cargoBuild (systemCommonRust.common-attrs // {
            src = cargoTools.rustSrc;
            pname = "${chainName}-local-integration-tests";
            doInstallCargoArtifacts = false;
            cargoArtifacts = self'.packages.common-test-deps;
            cargoBuildCommand = "cargo test --package local-integration-tests";
            cargoExtraArgs =
              "--features=local-integration-tests,${chainName},std --no-default-features --verbose";
          });
      in {
        check-dali-integration-tests =
          checkIntegrationTests { chainName = "dali"; };
        check-picasso-integration-tests =
          checkIntegrationTests { chainName = "picasso"; };
      };
    };
}
