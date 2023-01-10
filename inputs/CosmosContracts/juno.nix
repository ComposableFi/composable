{ self, ... }: {
  perSystem = { config, self', inputs', pkgs, system, lib, ... }: {
    packages = {
      junod = pkgs.buildGoModule {
        name = "junod";
        doCheck = false;
        src = pkgs.fetchFromGitHub {
          owner = "CosmosContracts";
          repo = "juno";
          rev = "e6f9629538a88edf11aa7e7ed3d68c61f8e96aa6";
          sha256 = "sha256-ro4ACIolNPbGnZnK610uX1KPO+b728O284PlKrPY1JY=";
        };
        vendorSha256 = "sha256-yGvxHS3wzjY1ZPUwuLK6B1+Xii8ipzhJpGi2Gl5Ytdo=";
        fixupPhase = ''
          ${pkgs.patchelf}/bin/patchelf \
            --shrink-rpath \
            --allowed-rpath-prefixes /nix/store \
            --replace-needed libwasmvm.${
              builtins.head (lib.strings.split "-" system)
            }.so libwasmvm.so \
            $out/bin/junod
          ${pkgs.patchelf}/bin/patchelf \
            --add-rpath ${self'.packages.libwasmvm}/lib \
            $out/bin/junod
        '';
      };
    };
  };
}
