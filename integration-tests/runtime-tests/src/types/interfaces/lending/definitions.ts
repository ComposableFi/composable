import { DefinitionRpc } from "@polkadot/types/types";

export default {
  rpc: {
    getBorrowLimit: {
      description: "The unclaimed amount",
      params: [
        {
          name: "market_id",
          type: "MarketId"
        },
        {
          name: "accountId",
          type: "AccountId"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        }
      ],
      type: "Balance"
    },
  },
  types: {
    MarketId: "u32"
  },
};
