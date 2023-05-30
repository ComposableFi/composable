# Overview

Composable is first chain which transfers tokens from Parity XCM chains to Cosmos IBC and back.

User experience it would be nice have as solution which will allow to do multi chain transfers with one wallet signature.

This proposal describes the solution to do such transfers.

## Prerequisites

You have read or clear about IBC whitepaper solution architecture and ICS-020 token transfer application and ICS-004 send/receive/acknowledgement/timeout of packets.

You have read general overview of Parity XCM and is up to date with MultiLocation format.

You are aware of how assets encoded in IBC and in XCM.

## How this document is structured

It describes very exact execution of three longest heterogenous multi hope scenarios we should consider.


## Parity Polkadot -> Composable Composable -> Composable Picasso -> Cosmos SDK


## On Polkadot

Send `DOT` transfer to `parent = 0, parachain = Composable, pallet = IBC, index = 15, account = Alice`

**Details**

`index` encodes IBC `channel id` indicating it should be forwarded over it. 

`account` is 32 bit account address. 

`pallet` and `parachain` are numbers, here we just use stings for readability. 


https://github.com/strangelove-ventures/packet-forward-middleware