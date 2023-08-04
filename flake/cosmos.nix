{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }: {
      packages = rec {
        gex = self.inputs.cosmos.packages.${system}.gex;
        bech32 = self.inputs.bech32.packages.${system}.default;
      };
      _module.args.cosmosTools = rec {
        devnet-root-directory = "/tmp/composable-devnet";
        validator-mnemonic =
          "bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort";

      };
    };
}
