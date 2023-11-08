# Tutorial: Testing a DEX 

Let's see a real-world example by testing `wasmswap`. We are going to swap the native token `cosm`
with the Cw20 token `pica`.

## Setting up

We are using a slightly modified version of `wasmswap`. The only difference is our VM doesn't alter
the `data` field because it is supposed to be up to user to use any data in any format. So we chose
to follow the spec there. Let's clone the template for our tutorial:

```sh
git clone https://github.com/ComposableFi/cw-toolkit
cd cw-toolkit
git checkout tags/wasmswap-template
cd orchestrate-tutorial
```

Add the latest `cosmwasm-orchestrate` as a `dev-dependency`:

```toml
# In Cargo.toml

[dev-dependencies]
cosmwasm-orchestrate = { git = "https://github.com/ComposableFi/cosmwasm-vm" }
```

And let's create a file for the integration tests:

```bash
mkdir tests
touch tests/integration.rs
```

## Setting up the state for the test

### Code

```rust
// In `integration.rs`

use cosmwasm_orchestrate::{
    block,
    cosmwasm_std::{Coin, MessageInfo},
    env, info,
    vm::*,
    Direct, JunoApi, StateBuilder, WasmLoader,
};
use cosmwasm_std::{Addr, Decimal, Uint128};
use cw20::{BalanceResponse, Cw20Coin, Cw20ExecuteMsg, Cw20QueryMsg, Denom};
use cw20_base::msg::InstantiateMsg as Cw20InstantiateMsg;
use wasmswap::msg::{ExecuteMsg, InstantiateMsg, TokenSelect};

#[test]
fn swap_works() {
    // Compile and load the wasmswap contract
    let wasmswap_code = WasmLoader::new(env!("CARGO_PKG_NAME")).load().unwrap();
    let cw20_code = include_bytes!("../scripts/cw20_base.wasm");
    // Create a VM state by providing the codes that will be executed.
    let mut state = StateBuilder::new()
        .add_code(&wasmswap_code)
        .add_code(cw20_code)
        .build();
}
```

If you were to build your test right now, it will fail but we'll shortly make it work, no worries.

### Analyze

```rust
let cw20_code = include_bytes!("../scripts/cw20_base.wasm");
```
For loading `cw20_base` contract, we just used `include_bytes` because we have the binary locally.
But if you need to get this binary from a (reliable) remote source, you could also use:
* `CosmosFetcher`: To fetch the code from a Cosmos chain.
* `FileFetcher`: To fetch the code from a server. (Like using `wget`)

---

```rust
let wasmswap_code = WasmLoader::new(env!("CARGO_PKG_NAME")).load().unwrap();
```
For the wasmswap contract, we used a special type of loader which is `WasmLoader`. The thing is
our VM works with wasm binaries just like any other chain. So we need to make sure that we are
feeding the VM with the latest compiled wasm binary every time we run the tests. We could manually
compile the binary and just do `include_bytes` but believe me, you will forget to do it very often
and get confused. 

`WasmLoader` gets the package name and compiles and loads the contract for you. Note that the default
configuration assumes the rust package name is the same as the contract name. So `wasmswap` as the package
name means that the contract's name is also `wasmswap` and the full name will be `wasmswap.wasm`.
The second assumption of the default loader is the target directory is `target/wasm32-unknown-unknown/release`.
The good thing is you can configure any of those, even the command that will be used to build your 
contract.

---

```rust
let mut state = StateBuilder::new()
    .add_code(&wasmswap_code) // code_id = 1
    .add_code(cw20_code)      // code_id = 2
    .build();
```
Finally, we need to create a `State` for the VM which will contain the wasm binaries. Note that 
code id's that we are gonna use later on will be in the same order as how it is given to the `StateBuilder`.
So in this case, `code_id` for `wasmswap_code` will be `1` and `2` for `cw20_code`.

## Instantiating the token

Let's instantiate the `cw20` contract for `PICA`.

### Code
```rust
    let sender = Account::generate_from_seed::<JunoAddressHandler>("sender").unwrap();

    // Instantiate the cw20 contract
    let (cw20_address, _) = <JunoApi>::instantiate(
        &mut state,
        2,  // Code ID of cw20_base is 2
        None,
        block(),
        None,
        info(&sender),
        100_000_000_000,
        Cw20InstantiateMsg {
            name: "Picasso".into(),
            symbol: "PICA".into(),
            decimals: 10,
            initial_balances: vec![Cw20Coin {
                address: sender.clone().into(),
                amount: Uint128::new(100_000_000_000_000),
            }],
            mint: None,
            marketing: None,
        },
    )
    .unwrap();
```

To run, use:
```rust
cargo test --test integration
```

### Analyze

```rust
let sender = Account::generate_from_seed::<JunoAddressHandler>("sender").unwrap();
```

We need to create a valid address which we are gonna use to call the contracts. Which is the
`sender` field in `MessageInfo`. For that, we are using `Account` type with `JunoAddressHandler`.
This will create a `bech32` encoded address with `juno` prefix.

---

```rust
let (cw20_address, _) = <JunoApi>::instantiate(
    &mut state,
    2,  // Code ID of cw20_base is 2
    None,
    block(),
    None,
    info(&sender),
    100_000_000_000, // We don't care about the gas
    Cw20InstantiateMsg {
        name: "Picasso".into(),
        symbol: "PICA".into(),
        decimals: 10,
        initial_balances: vec![Cw20Coin {
            address: sender.clone().into(),
            amount: Uint128::new(100_000_000_000_000),
        }],
        mint: None,
        marketing: None,
    },
)
.unwrap();
```

And then we are instantiating the `cw20` contract. The first notable thing is we are using `JunoApi` because
we have used the `JunoAddressHandler`. `JunoApi` will use `JunoAddressHandler` as the address handler.

Note that we are giving the instantiate message as is without doing any JSON encoding. This is because
`instantiate` function gets any JSON-serializable type and serializes it under the hood. But if you
have no access to message type and you don't want to define them yourself, you can use `instantiate_raw`
function and provide the JSON-encoded message.

Finally, note that we do `info(&sender)` which creates a `MessageInfo` and sets the `sender` field to
the given account's address.

## Instantiating the swap

Next thing is to instantiate the swap with the token we just created.

```rust
    let (contract_addr, _) = <JunoApi>::instantiate(
        &mut state,
        1, // Code ID for wasmswap is 1
        None,
        block(),
        None,
        info(&sender),
        100_000_000_000,
        InstantiateMsg {
            token1_denom: Denom::Native("cosm".into()),
            token2_denom: Denom::Cw20(Addr::unchecked(cw20_address.clone())),
            lp_token_code_id: 2,
            owner: None,
            protocol_fee_recipient: sender.clone().into(),
            protocol_fee_percent: Decimal::zero(),
            lp_fee_percent: Decimal::zero(),
        },
    )
    .unwrap();
```

Just as we instantiated the `cw20` token, we instantiated the swap. Note that `cw20_address` that
we got from the previous token instantiation is given to the swap contract as the `cw20` token.

And also, `lp_token_code_id` is set to `2` which is the code id for `cw20_base`.

## Increasing allowance

Before executing the swap, we need to make sure that the swap contract has the allowance to transfer tokens
from the account that does the swap. So let's create a second account and increase the allowance for the
swap contract.

```rust
    // Create the second account which will do the swap
    let swapper = Account::generate_from_seed::<JunoAddressHandler>("swapper").unwrap();

    let _ = <JunoApi>::execute(
        &mut state,
        env(&cw20_address),
        info(&sender),
        100_000_000_000,
        Cw20ExecuteMsg::IncreaseAllowance {
            spender: contract_addr.clone().into(),
            amount: Uint128::new(100_000_000_000),
            expires: Some(cw0::Expiration::Never {}),
        },
    )
    .unwrap();
```

Note that we have given `cw20_address` to the `env` function. This will create an `Env` and set 
`contract.address` field to the given address and this contract will be executed.

## Adding liquidity to the swap

We also need to have enough liquidity to be able to do the swap. So let's add some juice.

```rust
    let _ = <JunoApi>::execute(
        &mut state,
        env(&contract_addr),
        MessageInfo {
            sender: sender.clone().into(),
            funds: vec![Coin::new(400_000_000, "cosm")],
        },
        100_000_000_000,
        ExecuteMsg::AddLiquidity {
            token1_amount: 400_000_000u128.into(),
            min_liquidity: 50_000u128.into(),
            max_token2: 300_000_000u128.into(),
            expiration: None,
        },
    )
    .unwrap();
```

And when we execute it:
```sh
thread 'works' panicked at 'called `Result::unwrap()` on an `Err` value: BankError(InsufficientBalance)', tests/orchestrate.rs:109:6
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

WHAT THE HELL?

No worries, it is intended to show you how to easily understand what went wrong. Our VM logs too many
great things about the execution flow which includes the error messages. So let's enable logs to see
all that yummy logs.

```toml
# In Cargo.toml
env_logger = "0.10"
```

```rust
// In integration.rs

fn initialize() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        env_logger::init();
    });
}

#[test]
fn swap_works() {
    initialize();
}

```

And then let's execute the test once more with enabling the logs.

```rust
RUST_LOG=debug cargo test --test integration
```

Before going into the reason for failure, take your time to read the logs and see how much useful information
there is. You can see the host functions that are running, responses from the executions, and
sub-messages that are dispatched.

Now let's see what went wrong:
```
[2022-12-11T20:10:53Z DEBUG cosmwasm_orchestrate::vm] Transfer: 
    Account(Addr("juno1pgm8hyk0pvphmlvfjc8wsvk4daluz5tgrw6pu5mfpemk74uxnx9qwm56ug")) 
    -> Account(Addr("juno1ptl22cw3jth6ue4ruhgef2s5gfz23tt57vlrmydqu7vxk5xhn4gse7s6fe"))
    [Coin { denom: "cosm", amount: Uint128(400000000) }]

[2022-12-11T20:10:53Z DEBUG cosmwasm_orchestrate::vm] < Transaction abort: 0
```

The VM is trying to transfer `400_000_000cosm` and then it aborts the transaction with `BankError(InsufficientBalance)`.

The problem is obvious. See that we needed to transfer some funds to the contract to be able to add the
liquidity by properly setting `funds` field in the `MessageInfo`. But the `sender` account does not have
any `cosm` balance at all. So, the VM tried to transfer the given funds to the contract before execution
and failed.

Let's add some balance to the `sender` account in the `StateBuilder`.

```rust
    // Change the `StateBuilder` to:
    let mut state = StateBuilder::new()
        .add_code(&wasmswap_code)
        .add_balance(sender.clone(), Coin::new(100_000_000_000_000, "cosm"))
        .add_code(cw20_code)
        .build();

```

Now that we have enough balance, let's re-run again and see the lovely green `ok` message.

## `cosmwasm_std` vs. `cosmwasm_orchestrate::cosmwasm_std`

You might have already noticed it but we are using `Coin` and `MessageInfo` from `cosmwasm_orchestrate::cosmwasm_std`
instead of using them directly from `cosmwasm_std`. This is a temporary thing that we hope to resolve
soon. The problem is our VM is `no_std` but `cosmwasm_std` is originally only `std`. We created a PR
for this and until it gets merged, `cosmwasm-orchestrate` will be using our fork, hence `Cargo` thinks
that `cosmwasm_std::Coin` is not the same thing as `cosmwasm_orchestrate::cosmwasm_std::Coin`. Until the
merge, for any type/function that is given to `cosmwasm-orchestrate`, use `cosmwasm_orchestrate::cosmwasm_std`.
You will get a type mismatch error if you do it wrong, so beware of that.

## Executing the swap
```rust
    // Change the `StateBuilder` to:
    let mut state = StateBuilder::new()
        .add_code(&wasmswap_code)
        .add_balance(sender.clone(), Coin::new(100_000_000_000_000, "cosm"))
        .add_balance(swapper.clone(), Coin::new(120_000_000, "cosm"))
        .add_code(cw20_code)
        .build();

    let _ = <JunoApi>::execute(
        &mut state,
        env(&contract_addr),
        MessageInfo {
            sender: swapper.clone().into(),
            funds: vec![Coin::new(120_000_000, "cosm")],
        },
        100_000_000_000,
        ExecuteMsg::Swap {
            input_token: TokenSelect::Token1,
            input_amount: Uint128::new(120_000_000),
            min_output: Uint128::zero(),
            expiration: None,
        },
    )
    .unwrap();
```

Note that we add balance to `swapper` account as well to be able to send funds to the swap contract.
And finally, we can swap `120_000_000cosm`. The final thing to do is verify the swap worked.

## Verifying the swap

We will check `swapper`'s `pica` balance to verify the swap. To do that, we need to query the `cw20` token.

```rust
    let query_res: BalanceResponse = <JunoApi<Direct>>::query(
        &mut state,
        env(&cw20_address),
        Cw20QueryMsg::Balance {
            address: swapper.clone().into(),
        },
    )
    .unwrap();

    assert_eq!(
        query_res.balance,
        Uint128::new((120_000_000 * 300_000_000) / (400_000_000 + 120_000_000))
    );
```

Note that we used `<JunoApi<Direct>>` this time instead of `<JunoApi>`. This is because the execution
type for `query` can only be `Direct`. See previous sections to learn more about this.

And here we verified the whole execution.

One final note is if you were to verify the native balances also, you could directly use the bank module
to see the balances:

```rust
state.db.bank.balance(&swapper, "cosm");
```