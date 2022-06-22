export default {
  rpc: {
    instantiate: {
      description: "Instantiate a contract",
      params: [
        {
          name: "origin",
          type: "AccountId32"
        },
        {
          name: "value",
          type: "BTreeMap<CustomRpcCurrencyId, CustomRpcBalance>"
        },
        {
          name: "gas_limit",
          type: "u64"
        },
        {
          name: "storage_deposit_limit",
          type: "Option<CustomRpcBalance>"
        },
        {
          name: "code",
          type: "Code"
        },
        {
          name: "data",
          type: "Bytes"
        },
        {
          name: "salt",
          type: "Bytes"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "ContractInstantiateResult"
    },
    call: {
      description: "Call a contract",
      params: [
        {
          name: "origin",
          type: "AccountId32"
        },
        {
          name: "dest",
          type: "AccountId32"
        },
        {
          name: "value",
          type: "BTreeMap<CustomRpcCurrencyId, CustomRpcBalance>"
        },
        {
          name: "gas_limit",
          type: "u64"
        },
        {
          name: "storage_deposit_limit",
          type: "Option<CustomRpcBalance>"
        },
        {
          name: "data",
          type: "Bytes"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "ContractExecResult"
    },
    query: {
      description: "Query a contract",
      params: [
        {
          name: "origin",
          type: "AccountId32"
        },
        {
          name: "dest",
          type: "AccountId32"
        },
        {
          name: "value",
          type: "BTreeMap<CustomRpcCurrencyId, CustomRpcBalance>"
        },
        {
          name: "gas_limit",
          type: "u64"
        },
        {
          name: "storage_deposit_limit",
          type: "Option<CustomRpcBalance>"
        },
        {
          name: "data",
          type: "Bytes"
        },
        {
          name: "at",
          type: "Hash",
          isOptional: true,
        },
      ],
      type: "ContractExecResult"
    },
  },
  types: {
    Code: {
      _enum: {
        Upload: "Bytes",
        Existing: "Hash"
      }
    },
    StorageDeposit: {
      _enum: {
        Refund: "CustomRpcBalance",
        Charge: "CustomRpcBalance"
      }
    },
    ContractExecResult: {
      gas_consumed: "u64",
      gas_required: "u64",
      storage_deposit: "StorageDeposit",
      debug_message: "Bytes",
      result: "Result<Option<Bytes>, DispatchError>"
    },
    ContractInstantiateResult: {
      gas_consumed: "u64",
      gas_required: "u64",
      storage_deposit: "StorageDeposit",
      debug_message: "Bytes",
      result: "Result<Option<Bytes>, DispatchError>"
    },
  }
};
