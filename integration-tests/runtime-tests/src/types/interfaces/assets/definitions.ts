export default {
  rpc: {
    balanceOf: {
      description: "Balance available for the specified account for the specified asset.",
      params: [
        {
          name: "asset",
          type: "CustomRpcCurrencyId"
        },
        {
          name: "account",
          type: "AccountId32"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true
        }
      ],
      type: "CustomRpcBalance"
    },
    listAssets: {
      description: "Lists assets.",
      params: [
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "Vec<Asset>"
    },
  },
  types: {
    CurrencyId: "u128",
    AssetsBalance: "u128",
    Asset: {
       name: "Vec<u8>",
       id: "u64"
    }
  },
};
