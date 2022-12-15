{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }: {
      packages = {
        # Very hacky, as this relies on the data being present. Would be much nicer
        # to have this derivation use gsutil to download the simnode data.
        simnode-data = pkgs.stdenv.mkDerivation {
          name = "simnode-data";
          src = ./hash.txt;
          dontUnpack = true;
        };
      };
    };
}
