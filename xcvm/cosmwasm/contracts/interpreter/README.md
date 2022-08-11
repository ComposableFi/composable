# XCVM Interpreter

The XCVM interpreter contract interprets the XCVM programs. Available instructions are:

### Call 
Which is used to call a contract. See that the encoded payload must be in a format:
```
{
	"address": "contract-addr",
	"payload": "json-encoded ExecuteMsg struct"
}
```

### Transfer
Queries `asset-registry`, gets the contract address and then executes that contract to do the transfer.

### Spawn
Emits `spawn` even with the given parameters.

## Compile

```sh
RUSTFLAGS='-C link-arg=-s' cargo b --package=xcvm-interpreter --target=wasm32-unknown-unknown --profile="cosmwasm-contracts"
```

* `-C link-arg=-s` is used for stripping the binary which reduces the binary size drastically.
* `--profile="cosmwasm-contracts"` must be used for cosmwasm contracts.

## Test

```sh
cargo test --package="xcvm-interpreter"
```
