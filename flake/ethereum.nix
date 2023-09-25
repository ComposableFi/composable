{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }: {
      packages = rec {
        forge = pkgs.stdenv.mkDerivation rec {
          name = "forge";
          src = self.inputs.ethereum.packages.${system}.foundry;
          installPhase = ''
            mkdir --parents $out/bin
            cp $src/bin/forge $out/bin/forge
          '';
        };
      };
    };
}
