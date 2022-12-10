# Router Contract

Router is used by gateway to pass funds to interpreter and execute them.

## Compile

```sh
RUSTFLAGS='-C link-arg=-s' cargo b --package=xcvm-router --target=wasm32-unknown-unknown --profile="cosmwasm-contracts"
```

* `-C link-arg=-s` is used for stripping the binary which reduces the binary size drastically.
* `--profile="cosmwasm-contracts"` must be used for cosmwasm contracts.

## Test

```sh
cargo test --package="xcvm-router"
```
