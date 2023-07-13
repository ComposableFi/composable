{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, lib, crane, ... }: {
    packages = {
      cw20_base = pkgs.fetchurl {
        url =
          "https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm";
        hash = "sha256-nClak9UDPLdALVnN7e9yVKafnKUO7RAYDFO7sxwAXpI=";
      };
    };
  };
}
