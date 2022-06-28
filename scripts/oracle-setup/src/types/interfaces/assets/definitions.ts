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
      description: "Lists the available recognized assets for the runtime.",
      params: [
        {
          name: "at",
          type: "Hash",
          isOptional: true
        }
      ],
      type: "Vec<Asset>"
    }
  },
  types: {
    Asset: {
      name: "Vec<u8>",
      id: "u64"
    }
  }
};
