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



### ICS-20 Memo as `Spawn` carrier

`Spawn` forms `ICS-20` packet with `memo`.

`Assets` are put into `ICS-20` packet.

`Assets` are sent to `cvm-executor` contract, and  `wasm` termination callback is done to `cvm-outpost` contract with sender info and `Spawn` body.

`Memo` wasm message contains information to verify check sender from hash. 

`cvm-outpost` contract verifies amount sent and proceed with move of assets up to amount in message via delegation from `cvm-executor`. 

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