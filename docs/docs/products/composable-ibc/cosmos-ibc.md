# Cosmos ⬌ Polkadot Bridge

IBC is the native communication protocol of the Cosmos ecosystem, it allows for trustless communication to occur between any blockchains built using the Cosmos SDK which have the IBC module enabled. For the IBC connection between Polkadot and Cosmos, the Tendermint and Grandpa light clients are utlised alongside `ibc-rs` and the Hyperspace relayer on Picasso and Cosmos chains.

To connect the grandpa client implementation in Cosmos with the existing IBC stack, we are [extending the IBC specification](https://github.com/cosmos/ibc/pull/901) to include the `10-wasm-light-client module`. This module serves as an adapter between CosmWasm light clients and the ibc-go stack, allowing for an easy integration between two different Consensus protocols. In the future, this update also enables light clients from other ecosystems to be easily ported into Cosmos with minimal difficulty.

Composable has launched a Cosmos chain equipped with essential components for establishing connections to Parachains via IBC. This alleviates the integration bottlenecks that previously required each Cosmos appchain to wait and integrate with a wasm client module. This streamlined approach significantly simplifies the process of bringing IBC communication between Polkadot and Cosmos to production, enabling seamless integration with Cosmos chains as if they were connecting with each other directly. 

The Composable Cosmos chain functions as an intermediary network for Cosmos SDK chains, facilitating smooth IBC transfers between every Composable IBC extension to the Cosmos as a result of utilizing the IBC Packet forwarding middleware.

A diagram showcasing the high-level architecture utilized in this implementation can be found below:

![cosmos_polkadot_bridge_stack](../images-centauri/centauri-stack.png)

Cosmos ⬌ Polkadot transfers utilizing IBC

