# Composable Cosmwasm CLI

CosmWasm interaction commands.

## Guide

Follow this guide to upload, initialize and execute cw20 contracts on local Picasso Rococo DevNet using command line.

### Prerequisites

Same as for [manual guide](../../../../../docs/docs/products/cosmwasm/deploy-and-run-cosmwasm-contracts-with-pdjs.md).

### Steps

1. `nix run composable#devnet-picasso`    
2. `curl --location https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm > cw20_base.wasm`
3. `cargo run substrate -c ws://127.0.0.1:9988 -n alice tx upload --file-path ./cw20_base.wasm` 
4. `cargo run substrate -c ws://127.0.0.1:9988 -n alice tx instantiate --code-id 1 --salt 0x1234 --label 0x4321 --gas 10000000000 --message '{ "decimals" : 18, "initial_balances": [], "name" : "SHIB", "symbol" : "SHIB", "mint": {"minter" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"} }'`
5. `export CONTRACT_ADDRESS`
6. `cargo run substrate -c ws://127.0.0.1:9988 -n alice tx execute --contract "$CONTRACT_ADDRESS" --gas 10000000000 --message '{ "mint" : { "amount" : "123456789", "recipient" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" }}'`
7. `cargo run substrate -c http://127.0.0.1:9988 rpc query --contract "$CONTRACT_ADDRESS" --gas 10000000000 --query '{"balance": {"address": "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"}}'`
 
### Testnet

Repeat steps on testnet.
