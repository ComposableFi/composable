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

### Setup

This run will target mainnet, for devnet you have to replace contract address.

You can query address using wasm cli as per relevant wasmd usage guides from Notional/Osmosis/CosmWasm.


### Query configuration of relayer


### Prominent identifiers

PICA
PICA<->OSMO

### Send transfer and swap program

### What too look in explorer

On sender side look for wasmd `cvm` prefixed events, specifically.
And underlying IBC transport events.

Here is list of prominent events. All events can be seen by [generating schema](./cosmwasm/README.md).

`cvm.interpreter.exchange.succeeded` - swap success

On receiver side look at IBC event.
On other side look for wasmd `cvm` events for mainnet conract address.




In wasmd events you can observer instanciation of interprter contract.


### Query state of contract 


So after you generated schema, you will see all queries. 

Interpreter has `State` query which will dump whole state of interpreter.

You can follow CW20 and Cosmos Bank guide to get amounts of assets on interpeter address.

All these amounts are fully managed by user. In case of error, funds are retained here.

In case of `ResultRegister` is not empty, program did not executed to the end. 

Failure can happen on IBC and WASMD level, without CVM executed to point where it can issue events. 
For this case please follow IBC and WASMD guides to track execution (generalized indexers like Numua, Mintscan and Cosmos Indexer are super useful in this case).

and use may send next program to move funds (unstuck):


### DOT

In configuraiton you can observer DOT and DOT<->OSMO pools as:

For education purposes pleasse modify PICA swap message to swap DOT.


### Make it fail

First modify timeout of sending from Osmosis to Centauri to small value.

In this case after swap, IBC packet will be sent, but timeout. 

Fund will appear on free balance of interpeter.


Until Osmosis and Centauiry update Cosmos SDK https://github.com/cosmos/ibc-go/pull/4706 to this version, funds will will stuck in IBC. 