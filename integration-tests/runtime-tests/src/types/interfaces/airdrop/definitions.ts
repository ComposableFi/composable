// eslint-disable-next-line @typescript-eslint/no-unused-vars

export default {
  rpc: {},
  types: {
    // Structs
    PalletAirdropModelsAirdrop: {
      creator: "AccountId",
      total_funds: "Balance",
      total_recipients: "u32",
      claimed_funds: "Balance",
      start: "Option<Moment>",
      schedule: "Moment",
      disabled: "bool"
    },
    PalletAirdropModelsRecipientFund: {
      total: "Balance",
      claimed: "Balance",
      vesting_period: "Period",
      funded_claim: "bool",
    },

    // Enums
    PalletAirdropModelsIdentity: "Null",
    PalletAirdropModelsProof: "Null",
  }
};
