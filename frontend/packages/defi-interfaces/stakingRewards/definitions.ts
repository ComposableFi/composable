export default {
  rpc: {
    claimableAmount: {
      description:
        "Get the claimable amount for current collection and instance id",
      params: [
        {
          name: "fnft_collection_id",
          type: "SafeRpcWrapper<AssetId>",
        },
        {
          name: "fnft_instance_id",
          type: "SafeRpcWrapper<FinancialNftInstanceId>",
        },
      ],
      type: "Result<BTreeMap<AssetId, Balance>, ClaimableAmountError>",
    },
  },
  types: {
    ClaimableAmountError: "Null",
  },
};
