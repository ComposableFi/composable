{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
      packages = rec {
        cmc-api = crane.nightly.buildPackage (systemCommonRust.common-attrs // {
          name = "cmc-api";
          cargoArtifacts = self'.packages.common-deps;
          cargoBuildCommand = "cargo build --release --package cmc-api";
          installPhase = ''
            mkdir -p $out/bin
            cp target/release/cmc-api $out/bin/cmc-api
          '';
          meta = { mainProgram = "cmc-api"; };
        });

        cmc-api-image = pkgs.dockerTools.buildImage {
          name = "cmc-api";
          config = { Entrypoint = [ "${cmc-api}/bin/cmc-api" ]; };
        };
      };
    };
}
