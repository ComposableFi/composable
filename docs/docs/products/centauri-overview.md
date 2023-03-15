# Centauri (IBC)

## Trustless bridging via the IBC Protocol

Centauri leverages and expands upon the existing Inter-Blockchain Communication Protocol (IBC) beyond Cosmos. 
The IBC protocol previously allowed for trustless bridging between Cosmos SDK chains; 
however, we are amongst the first to extend IBC to other ecosystems. 
The trustless condition of Centauri is due to the fact it is:

- built upon light clients that communicate with each other
- updates the state of a counterparty chain
- sends finality proofs and other types of transactions and packets that can be verified
- able to upgrade the state of a chain through sending finality proofs.

We are building an interconnected ecosystem via the IBC Protocol to chains 
by integrating light clients that track the finality in these ecosystems on IBC. 
There are 3 key pieces needed in order to achieve this goal:

## Pallet IBC

Pallet IBC is referred to as the IBC stack for non-Cosmos chains 
and is composed of IBC-rs and a Tendermint light client(ic07). 
IBC-rs is an implementation of IBC in Rust, 
which allows for IBC messages to be interpreted on Picasso(and other Substrate-based chains). 
Together these two components enable parachain to process and interpret IBC packets.

## GRANDPA light client
GRANDPA is a protocol developed by Parity to verify finality proofs of parachain headers. 
This custom light client is based on the GRANDPA protocol and complies with IBC-rs, 
allowing us to integrate the GRANDPA client into pallet-ibc.

The ICS-8 client enables light client implementations 
written in CosmWasm to run natively on blockchains built with the Cosmos SDK. 
This allows for a GRANDPA light client implementation on Cosmos as a CosmWasm contract 
which can be deployed to Cosmos chains, 
allowing blockchains in the Cosmos ecosystem to track the finality of DotSama parachains.

A custom-built relayer implementation allows 
for transferring arbitrary packets between DotSama and Cosmos-based blockchains using the IBC protocol. 
In the future we expect other relayer solutions to add support for cross-ecosystem message passing via IBC. 
Still, for the time being, Hyperspace is the only relayer implementation with this functionality.

The diagram presented here illustrates the proposed Cosmos ⬌ DotSama connection that will result 
from the integration of all the pieces of infrastructure we are pioneering.

## Hyperspace relayer implementation
A custom-built relayer implementation allows 
for transferring arbitrary packets between DotSama and Cosmos-based blockchains using the IBC protocol. 
In the future we expect other relayer solutions to add support for cross-ecosystem message passing via IBC. 
Still, for the time being, Hyperspace is the only relayer implementation with this functionality.

![centauri_stack](./images-centauri/centauri-stack.png)

The diagram presented here illustrates the proposed Cosmos ⬌ DotSama connection 
that will result from the integration of all the pieces of infrastructure we are pioneering.
