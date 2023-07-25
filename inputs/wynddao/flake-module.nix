{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, lib, crane, ... }: {
    packages = rec {
      wyndex-pair = pkgs.fetchurl {
        url =
          "https://github.com/wynddao/wynddex/releases/download/v2.1.0/wyndex_pair.wasm";
        hash = "sha256-GQh3SBVccriWhHNPe22VMGWJVqfJa7x3cWy67j6NFTg=";
      };

      wyndex-factory = pkgs.fetchurl {
        url =
          "https://github.com/wynddao/wynddex/releases/download/v2.1.0/wyndex_factory.wasm";
        hash = "sha256-2ZYILTelKNsuqfOisXhrg4TPLwocaVNp6UN+6LN51SQ=";
      };
    };
  };
}
