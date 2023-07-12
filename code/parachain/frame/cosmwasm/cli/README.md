# Composable Cosmwasm CLI

CosmWasm interaction commands.

## Guide

Follow this guide to upload, initialize and execute cw20 contracts on local Picasso Rococo DevNet using command line.

### Prerequisites

Same as for [manual guide](../../../../../docs/docs/products/cosmwasm/deploy-and-run-cosmwasm-contracts-with-pdjs.md).

### CW20

1. `nix run composable#devnet-picasso`    
2. `curl --location https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm > cw20_base.wasm`
3. `CODE_ID=$(cargo run -- substrate --node ws://127.0.0.1:9988 --from alice --output json tx store ./cw20_base.wasm  | jq '.extrinsic.details.code_id')`
4. `CONTRACT_ADDRESS=$(cargo run -- substrate --node ws://127.0.0.1:9988 --from alice --output json tx instantiate2 $CODE_ID '{ "decimals" : 18, "initial_balances": [], "name" : "SHIB", "symbol" : "SHIB", "mint": {"minter" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"} }' 0x9999 --label 0x1111 --gas 10000000000 | jq '.cosmwasm_events[0].contract' -r)`
5. `cargo run substrate --node ws://127.0.0.1:9988 --from alice --output json tx execute --contract "$CONTRACT_ADDRESS" --gas 10000000000 --message '{ "mint" : { "amount" : "123456789", "recipient" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" }}'`
6. `cargo run substrate --node http://127.0.0.1:9988 --output json query wasm --contract "$CONTRACT_ADDRESS" --gas 10000000000 --query '{"balance": {"address": "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"}}'`

### DEX precompile

```bash
cargo run substrate --node http://127.0.0.1:9988   --output json query wasm --contract "5EYCAe5iidyqfb6z7dgK2d2Wpk9D1n8KpBUi1jra4a4PTPg4"   --gas 10000000000   --query '{"assets": {"pool_id": "1"} }'
```

### Testnet

Repeat steps on testnet.
