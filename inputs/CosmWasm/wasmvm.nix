{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, lib, crane, ... }: {
    packages = {
      libwasmvm = crane.nightly.buildPackage {
        src = "${
            pkgs.fetchFromGitHub {
              owner = "CosmWasm";
              repo = "wasmvm";
              rev = "a9e26c0e4e5a076d82556c4f44abeee2a64ff37e";
              hash = "sha256-zR47q8Z2znPigecPDmw5L4ef20/TXv8cPxaXTdJGxg0=";
            }
          }/libwasmvm";
        doCheck = false;
        installPhase = ''
          mkdir -p $out/lib
          mv target/release/libwasmvm.so $out/lib/libwasmvm.${
            builtins.head (pkgs.lib.strings.splitString "-" system)
          }.so
        '';
      };
    };
  };
}
