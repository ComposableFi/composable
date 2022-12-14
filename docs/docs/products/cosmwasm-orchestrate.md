# Introduction

CosmWasm Orchestrate is a tool for simulating and testing CosmWasm contracts on a virtual machine. Although this is the first version of the tool, it already provides a very close simulation experience to running your contracts on an actual chain.

## Why use this?

- **Complete simulation on a VM**: Your contracts do not run in a mock environment but actually in a complete VM, therefor your `CosmosMsg`'s, queries, and sub-messages run properly.

- **IBC capable**: You don't need to spin up a few chains to simulate IBC. You can just start a few VM instances on our framework and run IBC contracts in memory. This means that you will be able to test your IBC contracts really fast and correctly.
    
- **Easy to use**: Talking about VMs might sound frustrating but there is almost no setup necessary to run a test. Just provide the wasm contract that you want to test and call the entry point that you wanna test.

- **Flexible**: The framework is also very flexible and lets you define your own mechanism to handle custom messages, handle different address formats and even implement your own host functions and VM.


## A Quick Example

Enough talking and let's check out a very basic usage of our framework. Here you can see we execute a `cw20-base` contract. The exact steps are:

1. Fetch the wasm binary of the contract with the address `juno19rqljkh95gh40s7qdx40ksx3zq5tm4qsmsrdz9smw668x9zdr3lqtg33mf`.
2. Instantiate the `cw20-base` contract.
3. Execute a transfer.
4. Query the contract to see if the transfer is done correctly.

All in just a few lines.

```rust
// Fetch the wasm binary of the given contract from a remote chain.
let code = CosmosFetcher::from_contract_addr(
    "https://juno-api.polkachu.com",
    "juno19rqljkh95gh40s7qdx40ksx3zq5tm4qsmsrdz9smw668x9zdr3lqtg33mf",
)
.await
.unwrap();

// Generate a Juno compatible address
let sender = Account::generate_from_seed::<JunoAddressHandler>("sender").unwrap();

// Create a VM state by providing the codes that will be executed.
let mut state = StateBuilder::new().add_code(&code).build();

let info = info(&sender);

// Instantiate the cw20 contract
let (contract, _) = <JunoApi>::instantiate(
    &mut state,
    1,
    None,
    block(),
    None,
    info.clone(),
    100_000_000,
    InstantiateMsg {
        name: "Picasso".into(),
        symbol: "PICA".into(),
        decimals: 12,
        initial_balances: vec![Cw20Coin {
            amount: 10000000_u128.into(),
            address: sender.into(),
        }],
        mint: None,
        marketing: None,
    },
)
.unwrap();

// Transfer 10_000 PICA to the "receiver"
let _ = <JunoApi>::execute(
    &mut state,
    env(&contract),
    info,
    100_000_000,
    ExecuteMsg::Transfer {
        recipient: Account::generate_from_seed::<JunoAddressHandler>("receiver")
            .unwrap()
            .into(),
        amount: 10_000_u128.into(),
    },
)
.unwrap();

// Read the balance by using query. Note that the raw storage can be read here as well.
let balance_response: BalanceResponse = JunoApi::<Direct>::query(
    &mut state,
    env(&contract),
    QueryMsg::Balance {
        address: Account::generate_from_seed::<JunoAddressHandler>("receiver")
            .unwrap()
            .into(),
    },
)
.unwrap();

assert_eq!(Into::<u128>::into(balance_response.balance), 10_000_u128);
```