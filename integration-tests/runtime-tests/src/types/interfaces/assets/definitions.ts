export default {
  rpc: {
    balanceOf: {
      description: "Balance available for the specified account for the specified asset.",
      params: [
        {
          name: "asset",
          type: "CurrencyId"
        },
        {
          name: "account",
          type: "AccountId32"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "AssetsBalance"
    },
  },
  types: {
    CurrencyId: "u128",
    AssetsBalance: "u128",
  },
};
