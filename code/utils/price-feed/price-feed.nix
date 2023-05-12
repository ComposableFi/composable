{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
      packages = let name = "price-feed";
      in {
        "${name}" = crane.nightly.buildPackage (systemCommonRust.common-attrs
          // {
            pname = name;
            name = name;
            cargoArtifacts = self'.packages.common-deps;
            meta = { mainProgram = name; };
          });
      };
    };
}
