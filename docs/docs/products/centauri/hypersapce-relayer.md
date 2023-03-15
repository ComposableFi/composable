# Hyperspace Relayer

Composable's chain-agnostic, IBC-enabled Relayer
This section outlines [Hyperspace] - the first event-driven chain-agnostic IBC relayer fully written in Rust, 
and based on ibc-rs (the implementation of IBC in Rust). 
In the IBC architecture, a relayer is needed as blockchains do not directly communicate over the network. 
Primarily, it serves the purpose of “relaying” information between any rust-based chains
with the added benefit of being fully functional with any IBC-enabled chain. 

[Hyperspace]: https://github.com/ComposableFi/centauri/blob/master/hyperspace/README.md

As we designed Centauri to extend the reach of IBC beyond the Cosmos ecosystem, 
we realized that we needed a relayer that was both rust-based and IBC-compatible 
in order to birth interoperability between the various ecosystems on our roadmap. 
Currently, there are [3 relayers in Rust, Golang, and Typescript available], however, 
existing relayers were not built to be chain agnostic, and refactoring them would consist of major technical overhead, 
therefore, we opted to build a relayer universal to any underlying consensus or language of the source/destination chain. 
There are several client verification algorithms currently available in our light-client directory for different consensus engines, including:

[3 relayers in Rust, Golang, and Typescript available] https://ibcprotocol.org/relayers/

- Ics07-tendermint
- Ics10-grandpa
- Ics11-beefy
- Ics13-near

## An overview of Hyperspace's unique value proposition

The core infrastructure of Hyperspace comprises modular elements, 
each performing specific functions in the relaying process.

Hyperspace is a chain-agnostic IBC relayer that can be extended to support new chains with little to no changes to the core framework. 
Hyperspace does not perform any form of data caching. 
The relayer, therefore, relies heavily on the nodes it's connected to for sourcing data as needed. 
This design choice eliminates a class of bugs that could come from cache invalidation.

The relayer follows an event-driven model, where it waits idly until it receives a finality notification from any of the chains it's connected to. 
The finality notification represents new IBC messages and events that have been finalized and are ready to be sent to the connected counterparty chain.

**Hyperspace runs the Fisherman Protocol**
While trustless bridges are a preferable alternative to trusted solutions 
they are still subject to attacks if one of the blockchains in question is taken over by a malicious validator set. 
To address this threat, we implement the Fisherman Protocol.
Thus, any relayer running Hyperspace in ‘Fisherman mode’ can report malicious blockchain headers to freeze bridges 
if a chain is taken over by bad actors. 
The relayer then receives the associated slashing rewards for themselves, 
incentivizing relayers to remain vigilant and on the lookout for malicious blockchain takeovers.

Hyperspace is a permissionless relayer by design, 
which means that anyone can run a Fisherman node to earn rewards for helping to secure trustless bridges.

![hyperspace_fisherman](../images-centauri/hyperspace-fisherman.png)

