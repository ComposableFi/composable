{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      rustSrc = pkgs.lib.cleanSourceWith {
        filter = pkgs.lib.cleanSourceFilter;
        src = pkgs.lib.cleanSourceWith {
          filter = let
            isJSON = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".json" name;
            isREADME = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix "README.md" name;
            isDir = name: type: type == "directory";
            isCargo = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".toml" name
              || type == "regular" && pkgs.lib.strings.hasSuffix ".lock" name;
            isRust = name: type:
              type == "regular" && pkgs.lib.strings.hasSuffix ".rs" name;
            customFilter = name: type:
              ((isCargo name type) || (isRust name type) || (isDir name type)
                || (isREADME name type) || (isJSON name type));
          in pkgs.nix-gitignore.gitignoreFilterPure customFilter
          [ ../.gitignore ] ./.;
          src = ./.;
        };
      };
    in {
      packages = {
        check-picasso-integration-tests = crane.nightly.cargoBuild
          (systemCommonRust.common-attrs // {
            src = rustSrc;
            pname = "picasso-local-integration-tests";
            doInstallCargoArtifacts = false;
            cargoArtifacts = self'.packages.common-test-deps;
            cargoBuildCommand = "cargo test --package local-integration-tests";
            cargoExtraArgs =
              "--features=local-integration-tests,picasso,std --no-default-features --verbose";
          });
        check-dali-integration-tests = crane.nightly.cargoBuild
          (systemCommonRust.common-attrs // {
            src = rustSrc;
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
