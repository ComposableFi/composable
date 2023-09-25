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
          runtimeDependencies = [ pkgs.solc ];
          buildInputs = [ pkgs.solc ];
          FOUNDRY_SOLC="${pkgs.solc}/bin/solc";
          nativeBuildInputs = [ self'.packages.forge pkgs.solc ];
          pname = "evm-cvm-gateway";
          src = ./.;
          patchPhase = "true";
          buildPhase = "true";
          installPhase = ''
            ls /build
            ls /build/evm/
            ls /build/evm/lib/
            exit 42
            mkdir --parents $out/lib
            forge build --offline --out $out/lib 
          '';
          dontFixup = true;
          dontStrip = true;
        };
      };
    };
}
