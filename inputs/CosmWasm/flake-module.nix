{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, lib, crane, ... }: {
    packages = rec {
      cw20_base = pkgs.fetchurl {
        url =
          "https://github.com/CosmWasm/cw-plus/releases/download/v1.1.0/cw20_base.wasm";
        hash = "sha256-no9YPaUjE3fjYzLFhWpW1lOqCuIoR1K/EavSsORoUq4=";
      };
      cw4_stake = pkgs.fetchurl {
        url =
          "https://github.com/CosmWasm/cw-plus/releases/download/v1.1.0/cw4_stake.wasm";
        hash = "sha256-iptZCkOVlZSCy5XlSbtV388+/bUSdovSIztVoJmAQ8Y=";
      };
    };
  };
}
