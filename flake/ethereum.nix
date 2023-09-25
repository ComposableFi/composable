{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }: {
      apps = rec {

        forge = self.inputs.flake-utils.lib.mkApp {
          name = "forge";
          drv = self.inputs.ethereum.packages.${system}.foundry;
          exePath = "/bin/forge";
        };
      };
    };
}
