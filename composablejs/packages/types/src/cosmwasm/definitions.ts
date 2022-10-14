export default {
  rpc: {
  },
  types: {
    PalletCosmwasmEntryPoint: {
      _enum: {
        Instantiate: "()",
        Execute: "()",
        Migrate: "()",
        Reply: "()",
        Sudo: "()",
        Query: "()"
      }
    },
    PalletCosmwasmContractInfo: {
      codeId: "u64"
    },
    PalletCosmwasmCodeInfo: "Null"
  }
};
