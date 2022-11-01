{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
    packages = {
      price-feed = crane.nightly.buildPackage (systemCommonRust.common-attrs // {
        pnameSuffix = "-price-feed";
        cargoArtifacts = self'.packages.common-deps;
        cargoBuildCommand = "cargo build --release -p price-feed";
        meta = { mainProgram = "price-feed"; };
      });
    };
  };
}
