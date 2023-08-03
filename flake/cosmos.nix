{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }: {
      packages = rec { gex = self.inputs.cosmos.packages.${system}.gex; };
    };
}
