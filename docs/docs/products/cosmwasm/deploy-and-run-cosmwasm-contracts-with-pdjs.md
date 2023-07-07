# Overview

Follow this guide to upload, initialize and execute cw20 contract on local Picasso Rococo DevNet.

## Prerequisites

You successfully run simple transaction(like transfer) and observe events using Polkadot.js one one of Composable or Picasso runtimes.

It does not require you to know CosmWasm contracts well, but general awareness would be super useful.

You have [installed Nix](https://zero-to-nix.com/start/install) and successfully run any package from `composable` registry or installed container runner like `docker`.

You know how Picasso DEX can be used via PD.js.

## Examples

Run `nix run composable#devnet-picasso` or `docker run --publish 9988:9988 composablefi/devnet-picasso`   

### CW20 

1. Download [cw20](https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm) contract
3. Click [Direct Link](https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9988#/explorer) in devnet startup output on one of nodes to open Polkadot.js.
4. Go to `Developer -> Submission -> Extrinsics -> cosmwasm -> upload -> file upload` , click on input and peek `cw20_base.wasm`  
5. `Submit Transaction` as `ALICE`
6. Observe `cosmwasm.Uploaded.codeId` in events.
7. Events can be seen in `Network -> Explorer`
8. Call `cosmwasm -> instantiate` with `codeId` from event, gas `10000000000`, message `{ "decimals" : 18, "initial_balances": [], "name" : "SHIB", "symbol" : "SHIB", "mint": {"minter" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"} }`, salt and labels to unique random numbers like `0x1234` and  `0x4321`, other fields `0x`.
9. Observe `cosmwasm.Instantiated` event and click icon to copy `contract` address.
10. `cosmwasm -> execute` , put `contract` address from event, gas `10000000000`, message `{ "mint" : { "amount" : "123456789", "recipient" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" }}`, other fields `0x`.
11. Observe `cosmwasm.Executed` execution success.

### DEX precompile (singleton instance contract embedded into Substrate runtime)

1. Contract address is `5w3oyasYQg6vkzwETMqUfvtVM99GQ4Xy8mMdKXMgJZDoRYwg`

2. Execute message is `{ "members": [ { "addr": "5woQTSqveJemxVbj4eodiBTSVfC4AAJ8CQS7SoyoyHWW7MA6", "weight": 0 } ] }`


### Do

## CW4

Download [CW4 Group](https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw4_group.wasm) contract
and instantiate it with `{"members": [{"addr": "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL", "weight" : 1 }]}` message. 

## Testnet

Repeat steps here.

## Swap

Form Swap according schema and execute.