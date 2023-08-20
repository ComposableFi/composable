{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }: {
      packages = rec {
        gex = self.inputs.cosmos.packages.${system}.gex;
        beaker = self.inputs.cosmos.packages.${system}.beaker;
        bech32cli = self.inputs.bech32cli.packages.${system}.default;
      };
      _module.args.cosmosTools = rec {
        mnemonics = {
          pools =
            "traffic cool olive pottery elegant innocent aisle dial genuine install shy uncle ride federal soon shift flight program cave famous provide cute pole struggle";
        };
        devnet-root-directory = "/tmp/composable-devnet";
        validators = {
          mnemonic =
            "bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort";
          centauri = "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n";
          moniker = "validator";
          osmosis = "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj";
        };

        xcvm = {
          mnemonic =
            "apart ahead month tennis merge canvas possible cannon lady reward traffic city hamster monitor lesson nasty midnight sniff enough spatial rare multiply keep task";

          centauri = "centauri1qq0k7d56juu7h49arelzgw09jccdk8sujrcrjd";
          key = "A03mRJjzKKa8+4INiSDSdIzaMuA1nhbNs/B0fOVLlYNI";
          moniker = "xcvm";
          osmosis = "osmo1qq0k7d56juu7h49arelzgw09jccdk8su7jx0qv";
        };
      };
    };
}
