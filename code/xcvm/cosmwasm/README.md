### CW specifics 

#### Stake on Strides

Program to `Stake` on Stride and transfer staked token to Osmosis
is detected as pattern expressed in CVM.

That part of program is translated to IBC calls to Stride without contracts deployed.

So this program is possible
```
Osmosis ATOM -> 
Spawn(Stride, ATOM) -> Stake(ATOM) + Spawn(Osmosis, stATOM) 
-> Spawn(Centauri, stATOM)  
```

### ICS-20 Memo as `Spawn` carrier

`Spawn` forms `ICS-20` packet with `memo`.

`Assets` are put into `ICS-20` packet.

`Assets` are sent to `xc-account` contract, and  `wasm` termination callback is done to `xc` master contract with sender info and `Spawn` body.

`Memo` wasm message contains information to verify check sender from hash. 

`xc-master` contract verifies amount sent and proceed with move of assets up to amount in message via delegation from `xc-account`. 

Approach is needed because there is no `amount` information can be securely transferred in `memo`.


## How to run local Cosmos nodes(devnet) with CVM 

There are 2 options to run CVM enabled devnet.

First is Nix installation guide, and run via nix. Another run via docker.

```bash
# this runs lightweight chain only with cosmos chains as on latest main branch
nix run composable#devnet-xc-cosmos-fresh
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
cargo run --package cw-xc-interpreter --bin interpreter
```

## How to configure

Shell into relevant net, and run (with relevant modifications):

```sh
osmosisd tx wasm execute "osmo18tq76p8zmj49jr5zmsytulrzluljvd6m7he2uavvy6f8lsp77jwqwr0z6a" "$(cat code/CVM/cvm.json)" --gas=427753 --fees=2000$FEE --from=dz
```