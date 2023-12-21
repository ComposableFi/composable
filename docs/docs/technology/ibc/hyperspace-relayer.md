# Hyperspace Relayer

_Composable's chain-agnostic, IBC Relayer_
  
This section introduces [Hyperspace] - the first event-driven, chain-agnostic IBC relayer that is fully written in Rust and based on ibc-rs, the Rust implementation of IBC. In the IBC architecture, a relayer is necessary because blockchains cannot communicate directly over the network. The primary function of a relayer is to "relay" information between any IBC-enabled chain. 

[Hyperspace]: https://github.com/ComposableFi/centauri/blob/master/hyperspace/README.md

As Composable IBC was designed to extend the reach of IBC beyond the Cosmos ecosystem, it was necessary to develop a relayer that was both rust-based and IBC-compatible in order to birth interoperability on new ecosystems on. 

:::info

At present, there are three IBC relayers available in Rust, Golang, and Typescript, which can be found [here](https://ibcprotocol.org/relayers/). However, these existing relayers were not designed to support chains beyond Tendermint/CometBFT based chains, and reconfiguring them would require significant technical effort. Therefore, it was necessary to develop a new relayer that can function with any underlying consensus or programming language used by the source/destination chain.

:::

There are several client verification algorithms currently available in Composable's light-client directory for different consensus engines, including:

- [Ics07-tendermint](https://github.com/ComposableFi/centauri/tree/master/light-clients/ics07-tendermint)
- [Ics10-grandpa](https://github.com/ComposableFi/centauri/tree/master/light-clients/ics10-grandpa)
- [Ics11-beefy](https://github.com/ComposableFi/centauri/tree/master/light-clients/ics11-beefy)
- [Ics13-near](https://github.com/ComposableFi/centauri/tree/master/light-clients/ics13-near)

## Hyperspace architecture 

Hyperspace is a chain-agnostic IBC relayer that can be extended to support new chains with little to no changes to the core framework. **Hyperspace is stateless**, this means it does not perform any form of data caching. The relayer, therefore, relies heavily on the nodes it's connected to for sourcing data as needed. This design choice eliminates a class of bugs that could come from cache invalidation.

The relayer operates on an **event-driven model**, which means that it waits idly until it receives a finality notification from any of the chains it is linked to. This finality notification represents new IBC messages and events that have been finalized and are ready to be sent to the connected counterparty chain.

Taking a closer look at Hyperspace's design, it can be broken down into three central components, each handling a distinct layer of the relayer's operations that collectively comprise the entire system. A visual representation of this breakdown is provided in the diagram below.

![hyperspace_overview](../images-centauri/hyperspace-overview.png)

The **Primitives** package serves as the foundation layer for the relayer, containing generic types and traits that allow for constructing the client component to interact with individual blockchains. Although it also includes functions for creating clients, connections, and channels on each chain, these operations may be better suited for the Core package.

The second layer consists of **chain-specific** packages that are built upon Primitives. These packages contain customized objects, methods, and implementations specific to each blockchain.

Finally, the **Core** package is responsible for the relaying logic and processes. It frequently communicates with the chain-specific packages, utilizing client objects such as ParachainClient and manipulating their endpoints to read on-chain data for preparing response messages to be sent to the counter-party chain. The Core functions as a processor, determining the workflow and performing tasks like parsing events, grouping messages, and batching packets. It also includes a simple CLI that manages four commands: relay, create-client, create-connection, and create-channel.

The diagram below provides a visual representation of the different components of the relayer and their interactions.

![hyperspace_architecture](../images-centauri/hyperspace-arch.png)
(Source:https://informal.systems/blog/comparing-hyperspace-hermes)

## Hyperspace runs the Fisherman Protocol

While trustless bridges are a preferable alternative to trusted solutions, they are still subject to attacks if one of the blockchains in question is taken over by a malicious validator set. The solution lies in the Fisherman Protocol. Thus, any relayer running Hyperspace in ‘Fisherman mode’ can report malicious blockchain headers to freeze bridges if a chain is taken over by bad actors. The relayer then receives the associated slashing rewards for themselves, thus incentivizing relayers to remain vigilant and on the lookout for malicious blockchain takeovers.

![hyperspace_fisherman](../images-centauri/hyperspace-fisherman.png)

:::tip

Hyperspace is a permissionless relayer by design, which means that anyone can run a Fisherman node to earn rewards for helping to secure trustless bridges.

:::