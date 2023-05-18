# Composable Cosmwasm CLI

CosmWasm interaction commands.

## Guide

Follow this guide to upload, initialize and execute cw20 contract on local Picasso Rococo DevNet.

### Prerequisites

It does not require you to know CosmWasm contracts well, but general awareness would be super useful.

You have installed Nix using Zero-to-Nix guide and successfully run any package from `composable` registry. 

For clarification of meaning of parameters and outputs on each step please refer to manual guide to use CosmWasm via Polkadot.js.

### Steps

1. `nix run composable#devnet-picasso`    
2. `curl --location https://github.com/CosmWasm/cw-plus/releases/download/v1.0.1/cw20_base.wasm > cw20_base.wasm`
3. `cargo run substrate -c ws://127.0.0.1:9988 -n alice tx upload --url ./cw20_base.wasm` 
4. `cargo run substrate -c ws://127.0.0.1:9988 -n alice tx instantiate --code-id 1 --salt 0x1234 --label 0x4321 --gas 10000000000 --message '{ "decimals" : 18, "initial_balances": [], "name" : "SHIB", "symbol" : "SHIB", "mint": {"minter" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"} }'`
5. `cargo run substrate -c ws://127.0.0.1:9988 -n alice tx execute --contract "5CntM2NFn4Vkyu77tMDm5TRosKd9qskYpafh8L6Lz2FGP2rD" --gas 10000000000 --message '{ "mint" : { "amount" : "123456789", "recipient" : "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL" }}'`
6. `cargo run substrate -c http://127.0.0.1:9988 rpc query --contract "5CntM2NFn4Vkyu77tMDm5TRosKd9qskYpafh8L6Lz2FGP2rD" --gas 10000000000 --query 's{"balance": {"address1": "5yNZjX24n2eg7W6EVamaTXNQbWCwchhThEaSWB7V3GRjtHeL"}}'`



 curl --header "Content-Type: application/json" --data '{"id":1, "jsonrpc":"2.0", "method" : "cosmwasm_query", "params": {"contract":"5CntM2NFn4Vkyu77tMDm5TRosKd9qskYpafh8L6Lz2FGP2rD", "gas": 10000000000, "query_request": []}}' http://127.0.0.1:9988 

 
### Testnet

Repeat steps on testnet.


[ + ] Instantiated
        - Contract address: 5CntM2NFn4Vkyu77tMDm5TRosKd9qskYpafh8L6Lz2FGP2rD
- Event: instantiate
        - Contract: 5CntM2NFn4Vkyu77tMDm5TRosKd9qskYpafh8L6Lz2FGP2rD
        - Attributes:
                - _contract_address: 5uJW9yA5U6cr5JwNE5NvimHnPidDEAAC9AYcS7bTE1QgMNZU
                - code_id: 1



