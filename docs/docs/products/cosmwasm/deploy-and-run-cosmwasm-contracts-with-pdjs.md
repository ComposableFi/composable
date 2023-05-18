# Overview

Follow this guide to upload, initialize and execute cw20 contract on local Picasso Rococo DevNet.

## Prerequisites

You successfully run simple transaction and observe events using Polkadot.js one one of Composable or Picasso runtimes.

It does not require you to know CosmWasm contracts well, but general awareness would be super useful.

You have installed Nix using Zero-to-Nix guide and successfully run package from `composable` registry. 

## Steps

1. Run `nix run composable#devnet-picasso`    
2. Download [cw20](https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm) contract
3. Click `Direct Link` in devnet startup output on one of nodes to open Polkadot.js.
4. Go to `Developer -> Extrinsics -> cosmwasm -> upload -> file upload` , input `cw20_base.wasm` file and `Submit Transaction`.
5. Observe `cosmwasm.Uploaded.codeId` in events.
6. Call `cosmwasm -> instantiate` with `codeId` from event, gas `10000000000`, message `{ "decimals" : 18, "initial_balances": [], "name" : "SHIB", "symbol" : "SHIB", "mint": {"minter" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"} }`, salt and labels to unique random numbers like `0x1234` and  `0x4321`, other fields `0x`.
7. Observe `cosmwasm.Instantiated` event and click icon to copy `contract` address.
8. `cosmwasm -> execute` , put `contract` address from event, gas `10000000000`, message `{ "mint" : { "amount" : "123456789", "recipient" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" }}`, other fields `0x`.
9. Observe `cosmwasm.Executed` execution success.

### Testnet

Repeat steps on testnet.
