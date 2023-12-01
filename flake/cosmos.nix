{ self, ... }: {
  perSystem =
    { self', pkgs, systemCommonRust, subnix, lib, system, devnetTools, ... }: {
      packages = rec {
        gex = self.inputs.cosmos-old.packages.${system}.gex;
        beaker = self.inputs.cosmos-old.packages.${system}.beaker;
        bech32cli = self.inputs.bech32cli.packages.${system}.default;
      };
      _module.args.cosmosTools = rec {
        devnet-root-directory = "/tmp/composable-devnet";
        pools = {
          mnemonic =
            "traffic cool olive pottery elegant innocent aisle dial genuine install shy uncle ride federal soon shift flight program cave famous provide cute pole struggle";

        };
        validators = {
          mnemonic =
            "bottom loan skill merry east cradle onion journey palm apology verb edit desert impose absurd oil bubble sweet glove shallow size build burst effort";
          centauri = "centauri12smx2wdlyttvyzvzg54y2vnqwq2qjatescq89n";
          moniker = "validator";
          osmosis = "osmo12smx2wdlyttvyzvzg54y2vnqwq2qjateuf7thj";
        };
        mantis = {
          mnemonic =
            "green inch denial draw output great truth source dad summer betray price used claim lab garment scout twice increase buyer banana sniff forum salad";
          moniker = "mantis";
          centauri = "centauri1apckrk2dfpp32qrklk5cne5shdlekundvcdzxz";
        };
        cvm = {
          mnemonic =
            "apart ahead month tennis merge canvas possible cannon lady reward traffic city hamster monitor lesson nasty midnight sniff enough spatial rare multiply keep task";

          centauri = "centauri1qq0k7d56juu7h49arelzgw09jccdk8sujrcrjd";
          key = "A03mRJjzKKa8+4INiSDSdIzaMuA1nhbNs/B0fOVLlYNI";
          moniker = "cvm";
          osmosis = "osmo1qq0k7d56juu7h49arelzgw09jccdk8su7jx0qv";
        };
      };
    };
}
