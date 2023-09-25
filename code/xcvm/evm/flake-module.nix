{ self, ... }: {
  perSystem =
    { config, self', inputs', pkgs, system, crane, systemCommonRust, ... }:
    let
      x = "";
    in
    {
      packages = rec {
        evm-cvm-gateway = pkgs.stdenv.mkDerivation rec {
          name = "evm-cvm-gateway";
          FOUNDRY_SOLC="${pkgs.solc}/bin/solc";
          nativeBuildInputs = [ self'.packages.forge pkgs.solc ];
          pname = "evm-cvm-gateway";
          src = ./.;
          patchPhase = "true";
          buildPhase = "true";
          installPhase = ''
            ls /build
            echo "12321321321321321"
            ls /build/evm/
            echo "asdsadsads"
            ls /build/evm/lib/
            echo "zxczxcxzczxczxczxcxc"
            ls $src/lib

            mkdir --parents $out/lib
            exit
            forge build --offline --out $out/lib --lib-paths $src/lib 
          '';
          dontFixup = true;
          dontStrip = true;
        };
      };
    };
}
