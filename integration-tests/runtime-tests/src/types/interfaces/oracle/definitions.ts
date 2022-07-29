export default {
  rpc: {},
  types: {
    ComposableTraitsOraclePrice: {
      price: "u128",
      block: "BlockNumber"
    },
    ComposableTraitsOracleRewardTracker: "Null",
    PalletOracleAssetInfo: {
      threshold: "Percent",
      minAnswers: "u32",
      maxAnswers: "u32",
      blockInterval: "BlockNumber",
      rewardWeight: "Balance",
      slash: "Balance"
    },
    PalletOracleWithdraw: {
      stake: "u128",
      unlockBlock: "u32"
    },
    PalletOraclePrePrice: "Null",
    PalletOraclePrice: "Null",
  }
};
