# Overview

This documents describes generate solution stucture of CosmWasm on Substrate
and references documents and code about usage and quality of it.

## Stucture

CosmWasm on substrat consists of 2 parts.

[VM](https://github.com/ComposableFi/cosmwasm-vm), 
uses [parity-wasmi](https://github.com/paritytech/wasmi) as interpeter,
and [cosmwasm-sdk](https://github.com/CosmWasm/cosmwasm/) for interface types, 
used to produce CW VM capable of running CW contracts which runnalbe by reference `wasmd`
(wasmd is go module for Cosmos chains with AOT native complication for CW contracts).

All features are supported (as defined by CW SDK feature flags).

Second part is substrate [pallet-cosmwasm](../../../code/parachain/frame/cosmwasm). 
Which integrates CW with Substrate storage, RPC, [cli](../../../code/parachain/frame/cosmwasm/cli),
[Polkadot.JS](technology/cosmwasm/deploy-and-run-cosmwasm-contracts-with-pdjs.md). 

PD.js and cli can be used to execute transactions and qeury contract state.

## Substrate Runtime integration

Contracts can use pallet-assets via standard `Bank` interface.

Any pallet can be called via precompile(CW messaging interface to pallets as if these are usual contracts).

Polkadot XCM pallet can be called via [this](https://github.com/ComposableFi/composable/blob/main/code/xcvm/lib/core/src/transport/xcm/mod.rs) precompile. 

Our DEX pallet can be called via [this](https://github.com/ComposableFi/composable/blob/6db2dedb093b1cfb02cd5a3abbfb49a0a9c0fb96/code/parachain/frame/composable-traits/src/dex.rs#L20) precompile.

## Testing

VM can be tested via `cargo test`, examples of tests are [here](https://github.com/ComposableFi/cosmwasm-vm/blob/main/orchestrate/README.md),
run of contracts in simulator and asserts of results.

Pallet is covered with tests using test runtime you may find in pallet directory.

Above tests including property tests.

Pallet src folder has some [basic fuzzy tests](../../../code/parachain/frame/cosmwasm/fuzz_targets). 

Model checking(kani) as [per their documentation](https://model-checking.github.io/kani/tutorial-real-code.html) and our final research are not applicable to current solution. 

Benchamrks for pallet extrinics and gas metering of WASM instuctions are done using parity benchmarking infrastucture.

## Local devnet

Local devnet is released to [docker hub](https://hub.docker.com/r/composablefi/devnet) 
and to `nix run composable#devnet-picasso`.

CW20(analog of ERC-20) contract and 2 Composable contracts are embedded into genesis.
