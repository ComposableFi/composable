# pallet-cosmwasm
In this section we provide actual implementation details of the CosmWasm specification as a Substrate pallet.
We cover some of the theoretical topics that were presented before, and provide real code that exemplifies how they can be developed in a pallet.

We are deeply grateful to Ethan Frey and the talented team at Confio for their visionary work in creating CosmWasm. Their dedication, innovation, and commitment to advancing the field of blockchain technology have been a driving force in our journey.


## 0. Module Extensions

Module extensions are part of ambient host "contract" (like Bank or Ibc) which receive query and execute messages,potentially encoded in ProtoBuf, against modules interfaces.

We do not have such. 
Instead we have pallet precompiles which are "contracts" with pallet account as address which receive and execute contract messages, against pallet interfaces.

## 1. Gas Metering
When a contract is uploaded through the `upload` extrinsic, its code is instrumented.
This instrumentation process adds both gas metering and stack height limit to the uploaded code.

Proper gas metering is achieved in two steps:
1. **Benchmarking**: Each instruction is benchmarked to determine the cost of its execution.
2. **Injecting Gas Metering**: Gas metering is injected into the code by using [wasm_instrument](https://github.com/paritytech/wasm-instrument). This process injects a call into the gas function for every execution or path/code block (function call, if, else, etc.) with the associated execution cost parameter. Therefore, it computes the overall total cost and ensures that every code block is paid before getting executed.

Then, whenever a contract entrypoint is called, the pallet checks if the instrumentation version is up-to-date.
In case not, the code gets re-instrumented to ensure proper gas metering.

## 2. Uploading Contracts
The `upload` extrinsic is used to upload smart contracts´ `code` to the pallet.

### 2.1. Definition

```rust,ignore
#[pallet::weight(T::WeightInfo::upload(code.len() as u32))]
pub fn upload(origin: OriginFor<T>, code: ContractCodeOf<T>) -> DispatchResultWithPostInfo;
```

### 2.2. Execution Flow
1. Check if the `code` is already uploaded.
2. Reserve `length(code) * deposit_per_byte` amount of native asset.
3. Check if the wasm code is valid.
4. Do the instrumentation by injecting gas metering and stack height limit.
5. Assign a code id to the `code`. This `code id ` is incremented on each upload.
6. Deposit the upload event.

```rust,ignore
pub enum Event<T: Config> {
	Uploaded {
		code_hash: CodeHashOf<T>,
		code_id: CosmwasmCodeId,
	}
```

### 2.3. Fee
Fees depend linearly on the size of the code.

## 3. Instantiating a Contract
The `instantiate` extrinsic is used to instantiate a smart contract.

### 3.1 Definition

```rust,ignore
#[pallet::weight(T::WeightInfo::instantiate(funds.len() as u32).saturating_add(*gas))]
pub fn instantiate(
	origin: OriginFor<T>,
	code_identifier: CodeIdentifier<T>,
	salt: ContractSaltOf<T>,
	admin: Option<AccountIdOf<T>>,
	label: ContractLabelOf<T>,
	funds: FundsOf<T>,
	gas: u64,
	message: ContractMessageOf<T>,
) -> DispatchResultWithPostInfo;
```

`origin` will be the `sender` from the contracts' perspective.

Our goal is to create a deterministic smart contract environment.
Hence, we are not only using `code id` to identify a code.
Since `code id` depends on the current state of the chain, users won't be able to deterministically identify their code.
So we created `CodeIdentifier` which makes users able to also identify their code by using `CodeHash`.
And the corresponding `code id` is fetched internally.
This feature comes in handy when users want to batch their `upload + instantiate` calls, or they do any kind of scripting to upload and run the contracts.

```rust,ignore
pub enum CodeIdentifier<T: Config> {
	CodeId(CosmwasmCodeId),
	CodeHash(CodeHashOf<T>),
}
```

`admin` is the optional owner of the contract. Note that if it is set to `None`, the contract cannot ever be migrated or do any `admin` operations.
Therefore, it will become an immutable contract.
The `label` field is used as a human-readable `String` for the instantiated contract.
`salt` is used when a user wants to instantiate the same contract with the same parameters twice.
This ensures that during the contract address generation, addresses remain unique.
`funds` are transferred to the contract prior to instantiation.
Then, new balances will be visible to the contract.
`gas` represents the gas limit, and, `message` field is passed to the contract as the `InstantiateMsg`.

### 3.2 Execution Flow
First, the contract address MUST be derived.
As stated previously, one of our goals is determinism.
Then, the contract addresses are also deterministic as opposed to other CosmWasm-running chains.
The algorithm is based on `instantiator`, `salt`, `code_hash` and `message` which is:

```ignore
hash(instantiator + salt + code_hash + hash(message))
```

This gives users opportunity to know the contract address prior to creation, which becomes really handy when it comes to XCVM.
Because this will provide a way to know the `interpreter` address prior to the creation for example so that users can add `Transfer` instruction which transfers some funds to the `interpreter` without paying for late-binding.

Then the necessary setup is done like deriving a contract trie id for storage, increasing the `refcount` of the code.
Finally, instrumentation version is checked, and re-instrumentation happens if necessary.
Then, the `instantiate` entrypoint of the contract is called and `Instantiated` event is yielded.

```rust,ignore
pub enum Event<T: Config> {
	Instantiated {
		contract: AccountIdOf<T>,
		info: ContractInfoOf<T>,
	}
}
```

### 3.3 Fee
Total fees depend on three factors:
- Instructions to be run.
- Base cost of instantiate call
- Funds to be transferred.

The total fee can be computed as follows.

```ignore
base_instantiate_fee + (fee_per_fund * length(funds)) + executed_instruction_costs
```

The remaining gas is refunded after execution.

## 4. Executing a Contract
The `execute` extrinsic is used for executing a smart contract.

### 4.1. Definition

```rust,ignore
#[pallet::weight(T::WeightInfo::execute(funds.len() as u32).saturating_add(*gas))]
pub fn execute(
	origin: OriginFor<T>,
	contract: AccountIdOf<T>,
	funds: FundsOf<T>,
	gas: Gas,
	message: ContractMessageOf<T>,
) -> DispatchResultWithPostInfo;
```

The `contract` field contains the contract address to execute.
The `funds` are transferred to the contract prior to instantiation.
So that the new balances will be visible to the contract.
`gas` represents the gas limit, and, `message` is passed to the contract as the `ExecuteMsg`.

### 4.2 Execution Flow
The execution flow is reduced to a minimum for contract execution.
Only a check for re-instrumentation is performed, and then the execution of the `execute` entrypoint of the contract is triggered.

### 4.3 Fee
Total fees depend on three factors:
- Instructions to be run.
- Base cost of instantiate call
- Funds to be transferred.

The total fee can be computed as follows.

```ignore
base_instantiate_fee + (fee_per_fund * length(funds)) + executed_instruction_costs
```

The remaining gas is refunded after execution.
