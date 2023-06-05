{ self, ... }: {
  perSystem =
    { config
    , self'
    , inputs'
    , pkgs
    , lib
    , system
    , crane
    , systemCommonRust
    , subnix
    , ...
    }:
    let
      x = 2;
    in
    {
      packages = rec {
        wasmvm = crane.nightly.buildPackage  {
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

        banksyd = pkgs.buildGoModule {
          name = "banksyd";
          doCheck = false;
          nativeBuildInputs = [pkgs.patchelf];
          excludedPackages = ["interchaintest" "simd"];
                ldflags = [
            "-v -extldflags '-L${wasmvm}/lib'"
          ];
          src = pkgs.fetchFromGitHub {
            owner = "notional-labs";
            repo = "composable-centauri";
            rev = "409772f02ef62321dcad7cdc9ee709d6ae0afffa";
            sha256 = "sha256-dMIzjjYCH8dhSHJzpXmZuBbP+E7+9OI1CcGoTNc/xRE=";
          };
          dontFixup = true;
          vendorSha256 = "sha256-T2CGOx99toy9fDOrf35TP7JQEYMgbq26mwlscsMSGl0=";
        };
      };
    };
}
