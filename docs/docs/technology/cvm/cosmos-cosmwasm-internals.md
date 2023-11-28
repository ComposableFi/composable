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

```sh
nix run ".#build-cvm-json-schema-ts"
```

## How to deploy and configure mainnet

```sh
nix develop .#centauri-mainnet --impure
```

```sh
$BINARY tx wasm store "$GATEWAY_WASM_FILE" --from dz --gas=auto
$BINARY tx wasm instantiate 15 '{"admin": "centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k", "network_id" : 2}' --label "cvm_gateway_4" --admin centauri1u2sr0p2j75fuezu92nfxg5wm46gu22ywfgul6k --gas=auto --from=dz

$BINARY tx wasm store "$EXECUTOR_WASM_FILE" --from dz --gas=5305232

$BINARY tx wasm execute centauri1wpf2szs4uazej8pe7g8vlck34u24cvxx7ys0esfq6tuw8yxygzuqpjsn0d "$(cat cvm.json)" --from=dz -y --gas=500000
```

```sh
nix develop ".#osmosis-mainnet --impure"
```

```sh
$BINARY tx wasm store "$GATEWAY_WASM_FILE" --from dz --gas=25000000 --fees=75000uosmo -y

$BINARY tx wasm instantiate 271 '{"admin": "osmo1u2sr0p2j75fuezu92nfxg5wm46gu22yw9ezngh", "network_id" : 3}' --label "cvm_gateway_4" --admin osmo1u2sr0p2j75fuezu92nfxg5wm46gu22yw9ezngh --gas=400000 --from=dz --fees=1000uosmo

$BINARY tx wasm store "$EXECUTOR_WASM_FILE" --from dz --gas=25000000 --fees=75000uosmo -y

$BINARY tx wasm execute osmo1ltevzdpc6ku5en4spjn887nnd7qt4mz0msn6jpk3s40rn80uz9yqa68crl "$(cat cvm.json)" --from=dz -y --gas=500000 --fees=1500uosmo
```

### ICS-20 Memo as `Spawn` carrier

`Spawn` forms `ICS-20` packet with `memo`.

`Assets` are put into `ICS-20` packet.

`Assets` are sent to `cvm-executor` contract, and  `wasm` termination callback is done to `cvm-gateway` contract with sender info and `Spawn` body.

`Memo` wasm message contains information to verify check sender from hash. 

`cvm-gateway` contract verifies amount sent and proceed with move of assets up to amount in message via delegation from `cvm-executor`. 

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