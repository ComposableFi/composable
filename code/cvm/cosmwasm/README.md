# Overview 


In general CVM on CW-IBC is very similar, but more abstract version of https://github.com/osmosis-labs/osmosis/tree/main/cosmwasm/contracts/crosschain-swaps


## How to run local Cosmos nodes(devnet) with CVM 

There are 2 options to run CVM enabled devnet.

First is Nix installation guide, and run via nix. Another run via docker.

```bash
# this runs lightweight chain only with cosmos chains as on latest main branch
nix run composable#devnet-cosmos-fresh
```

```bash
# run all cosmos and dotsama chain with relays
docker run composablefi/devnet-xc:main
```

Picasso, Centauri and Osmosis are uploaded with CVM contracts.

Centauri and Osmosis main target of testing and configuration.


Wait for relayer start relaying.

Transfer ppica to Osmosis and uosmo to Centauri using command lines for daemons.
`nix run .#centauri-tx` and `nix run .#osmosis-tx` do it. 

Use preferred methods to observe transfer happened.
I look at logs at `/tmp/composable-devnet/`.

And finally run sample CVM program: 
```bash
# ppica on Centauri -> ppica on Osmosis -> swap to uosmo -> uosmo on Centauri
nix run .#xc-swap-pica-to-osmo
```


## How to generate schema

For gateway 

```sh
cargo run --package xc-core --bin gateway
```

For interpreter

```sh
cargo run --package cw-xc-executor --bin interpreter
```

For query/execute message look into official CosmWasm docs, for events, look at node names in `events.json`. 

## How to deploy and configure mainnet

```
nix develop .#centauri-mainnet --impure
``````

```
$BINARY tx wasm store "$GATEWAY_WASM_FILE" --from dz --gas=5305232
$BINARY tx wasm instantiate 13 '{"admin": "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k", "network_id" : 2}' --label "cvm_gateway_2" --admin centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k --gas=400000 --from=dz

$BINARY tx wasm store "$INTERPRETER_WASM_FILE" --from dz --gas=5305232

$BINARY tx wasm instantiate 13 '{"admin": "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k", "network_id" : 2}' --label "cvm_gateway_2" --admin centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k --from=dz


$BINARY tx wasm execute centauri1c676xpc64x9lxjfsvpn7ajw2agutthe75553ws45k3ld46vy8pts0w203g "$(cat cvm.json)" --from=dz -y --gas=500000
```

```
nix develop .#osmosis-mainnet --impure
```

```
$BINARY tx wasm store "$GATEWAY_WASM_FILE" --from dz --gas=5305232 --fees=20000uosmo -y

$BINARY tx wasm instantiate 163 '{"admin": "osmo1u2sr0p2j75fuezu92nfxg5wm46gu22yw9ezngh", "network_id" : 3}' --label "cvm_gateway_2" --admin osmo1u2sr0p2j75fuezu92nfxg5wm46gu22yw9ezngh --gas=400000 --from=dz --fees=1000uosmo

$BINARY tx wasm store "$INTERPRETER_WASM_FILE" --from dz --gas=5305232 --fees=20000uosmo

$BINARY tx wasm execute osmo126n3wcpf2l8hkv26lr4uc8vmx2daltra5ztxn9gpfu854dkfqrcqzdk8ql "$(cat cvm.json)" --from=dz -y --gas=500000 --fees=1500uosmo
```



### ICS-20 Memo as `Spawn` carrier

`Spawn` forms `ICS-20` packet with `memo`.

`Assets` are put into `ICS-20` packet.

`Assets` are sent to `xc-account` contract, and  `wasm` termination callback is done to `xc` master contract with sender info and `Spawn` body.

`Memo` wasm message contains information to verify check sender from hash. 

`xc-master` contract verifies amount sent and proceed with move of assets up to amount in message via delegation from `xc-account`. 

Approach is needed because there is no `amount` information can be securely transferred in `memo`.

### How to make Cosmos(CosmWasm) chain CVM enabled

#### CW enabled chain

If chain has `hook`s, there should be adapter in core library. There are Neutron, Osmosis and ICF hooks.

If chain has custom protocol `adapter` should be coded for it. For example, order book or liquid staking.

#### Non CW enabled

`Shortcut`s for remote call should be developed. For example for native IBC protocol to do staking.


### Both

For `hook` and `shortcut` build `outpost` tracking to handle packet failures

Make CVM Executor to handle `instruction` via `adapter+hook` or `shortcut`.