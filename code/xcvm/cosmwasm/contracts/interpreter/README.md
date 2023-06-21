# cw-xc-account

## Messages

Implements `cw-plus` `cw1` interface.

## Events

Note that these events will be yield from the router in production.

### Instantiate contract

Configured with `gov` account which is cross chain smart contract. 
`Gov` has 100% allowance to any funds on this contract.

```json
{
	"type": "wasm-xcvm.interpreter.instantiated",
	"attributes": [
		{
			"key": "data",
			"value": "{BASE64_ENCODED_DATA}"
		}
	]
}
```

- **BASE64_ENCODED_DATA**: base64 encoded `(network_id, user_id)` pair.


### Execute set allowed

`Owner` may set one account on which he set allowance for this instance address.

### Execute extended allowance

Ensures funds are contract on account for `16` blocks.
Each next block allowance extended by one block. 
Owner can `cancel` extended allowance of funds, and move funds after `16` blocks.
Owner can `remove` extended allowance, but will give `0.05%` of funds to `gov` account.
Parameters are configurable only by `gov` account, with limits hardcoded as 256 blocks and 1%.
Allows to set future `auto cancel` allowance to specific block (not less than `16` blocks from now).

### Execute contract
```json
{
	"type": "wasm-xcvm.interpreter.executed",
	"attributes": [
		{
			"key": "program",
			"value": "{XCVM_PROGRAM_TAG}"
		}
	]
}
```

- **XCVM_PROGRAM_TAG**: Tag of the executed XCVM program

### Execute spawn instruction

```json
{
	"type": "wasm-xcvm.interpreter.spawn",
	"attributes": [
		{
			"key": "origin_network_id",
			"value": "{ORIGIN_NETWORK_ID}"
		},
		{
			"key": "origin_user_id",
			"value": "{ORIGIN_USER_ID}"
		},
		{
			"key": "program",
			"value": "{XCVM_PROGRAM}"
		}
	]
}
```

- **ORIGIN_NETWORK_ID**: Network id of the origin. Eg. Picasso, Ethereum
- **ORIGIN_USER_ID**: Chain agnostic user identifier of the origin. Eg. contract_address in Juno
- **XCVM_PROGRAM**: Json-encoded xcvm program. Note that although it is json-encoded, it is put as a string because of the restrictions of cosmwasm.

## Usage

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
Emits `spawn` event with the given parameters.

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
