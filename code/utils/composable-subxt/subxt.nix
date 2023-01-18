{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, ... }: {
    packages = {
      subxt = pkgs.rustPlatform.buildRustPackage rec {
        pname = "subxt-cli";
        # NOTE: keep in sync with Cargo.toml as generator version must be same as dependency to compile generated code
        version = "0.23.0";

        src = pkgs.fetchCrate {
          inherit pname version;
          sha256 = "sha256-iESFu4rpHVORyFV+g53eVADqUt6x6vB6rCuxEUq/MiM=";
        };

        cargoHash = "sha256-roQ6fAHT9pdzeaLjedStg+C8voDnj8gbo/R0zloXZlo=";
        cargoDepsName = pname;
      };
    };
  };
}
