{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }: {
      packages = rec {
        gex = self.inputs.cosmos.packages.${system}.gex;
        bech32cli = self.inputs.bech32cli.packages.${system}.default;
      };
      _module.args.cosmosTools = rec {
        devnet-root-directory = "/tmp/composable-devnet";
        validators = {
          mnemonic =
            "bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort";
          centauri = "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n";
          moniker = "validator";
          osmosis = "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj";
        };

      };
    };
}
