{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
      packages = {
        subxt-exports = crane.nightly.buildPackage
          (systemCommonRust.common-attrs // {
            pname = "subxt-exports";
            cargoArtifacts = self'.packages.common-deps;
            # You can use RELAY_HOST and PARA_HOST environment variables to configure this crate.
            # You can use SUBXT_ENABLED=1 to enable code generation.
            cargoBuildCommand = "cargo build --release -p subxt-exports";
            meta = { mainProgram = "subxt-exports"; };
          });
      };
    };
}
