export default {
  rpc: {},
  types: {
    ComposableTraitsBondedFinanceBondOffer: {
      beneficiary: "AccountId32",
      asset: "CurrencyId",
      bondPrice: "u128",
      nbOfBonds: "u128",
      maturity: "ComposableTraitsBondedFinanceBondDuration",
      reward: "ComposableTraitsBondedFinanceBondOfferReward",
      keepAlive: "bool"
    },
    ComposableTraitsBondedFinanceBondDuration: {
      Finite: { returnIn: "u32" }
    },
    ComposableTraitsBondedFinanceBondOfferReward: {
      asset: "CurrencyId",
      amount: "u128",
      maturity: "u32"
    }
  }
};
