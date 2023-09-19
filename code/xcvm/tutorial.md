# Overview

This document describes basic usage of CVM on CosmWasm.

## Prerequisites

You have followed official guides of Osmosis and Centauri to setup their mainnet shells.

Mainnets has `osmo18tq76p8zmj49jr5zmsytulrzluljvd6m7he2uavvy6f8lsp77jwqwr0z6a` and `centauri18tq76p8zmj49jr5zmsytulrzluljvd6m7he2uavvy6f8lsp77jwqzp84md` gateway deployed.

For local run container or [nix](../../docs/docs/nix.md) according docs.

You must have PICA and (optional) DOT. Please follow multi hop guide to transfer amount.

## Recording 

You can find how devnet runs end to end https://www.youtube.com/watch?v=_nMD407E3F4

## End to end

These steps given that the user has send tx from Centauri to osmosis with PICA and swaps its to OSMO on Osmosis.

Identifiers for same flow with DOT are provided, but left for manual repeating.

Also common queries to get state of CVM in general and specific user are described. 

Finally, program to handle stuck funds (in case of cross chain message failure) is give.



### Asnwers

nix0t
Joon(ART)
  < 1 minute ago
1. example payload

- live TX https://explorer.nodestake.top/composable/tx/F4BDDC1F0D502E55F5B413C139C7DCE5608C1662B5D7919CAD9BA889B38C8861
- same in raw JSON https://github.com/ComposableFi/composable/pull/4091/files#diff-79ad53577207629481229b103ab8a8abc18cc1535206aa36b664bdff2c7a5215
- how i send on devnet via cli https://github.com/ComposableFi/composable/blob/866f40ba5558ef9c1a1adc6209e726c69f3c492a/inputs/notional-labs/composable-centauri/flake-module.nix#L377

2.1 which events to look for 
2.2. how to query the state

```
## How to generate schema
For gateway 
```sh
cargo run --package xc-core --bin gateway
```

For interpreter

```
cargo run --package cw-xc-interpreter --bin interpreter
```

So after you generated schema, you will see all queries. 

Interpreter has `State` query which will dump whole state of interpreter.

Also for interpreter, the is `events.json` which describes names and shapes of all events from interpreter, including exchange, fail, success.

3. ditto
@fl-y ????????????????

5. how to tell whether funds are stuck
in progress. This will be PR soon.

### Required for DoD

- How to execute the CW contract on Centauri, as in an example payload to an RPC endpoint

See above.

-> @dzmitry-lahoda

<p>How to query the CW contract on, </p>
- Centauri, to verify the IBC tx has been initiated to go over to Osmosis. Which events to look for or how to query the state

See above, `events.json`. For execute, instantaite, wasm events - look into official docs. CVM cannot do any special in that area.

In general, run contract and see output. If some events are missed from CVM contract, will add. Tried to add and unify what is super useful.

-> @dzmitry-lahoda

- Osmosis, to verify the swap has happened. Which events to look for or how to query the state.

`cvm.interpreter.exchange.succeeded`. If will contain exchange id. Id is mapped via config. Can cw query `config` from gateway to see what pool is used.

For state, run CW query named `state` agains interpreter. Will dump all state.

-> @dzmitry-lahoda

- How to check whether there was an error during the process to tell the user their funds are stuck.

You will see failed events, prefixed with cvm or see schema. Failues of CW/IBC/Cosmos on its own - CVM contracts nothing to do with that.

-> @dzmitry-lahoda