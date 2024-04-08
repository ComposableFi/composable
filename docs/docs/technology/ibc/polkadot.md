# Polkadot IBC

IBC was once known as the native communication protocol of the Cosmos ecosystem until Composable worked on extending this piece of technology to non-Cosmos chains. It allows for trust-minimised cross-chain messaging to occur between blockchains. For the IBC connection between Polkadot and Cosmos, the Tendermint and Grandpa light clients are utilised alongside `ibc-rs` and the Hyperspace relayer on Picasso and Picasso Kusama.

To connect the grandpa client implementation in Cosmos with the existing IBC stack, [the IBC specification](https://github.com/cosmos/ibc/pull/901) was extended to include the `10-wasm-light-client module`. This module serves as an adapter between CosmWasm light clients and the `ibc-go` stack, allowing for an easy integration between two different Consensus protocols. In the future, this update also enables light clients from other ecosystems to be easily ported into Cosmos with minimal difficulty.

Picasso is equipped with the essential components for establishing connections to Parachains via IBC. This alleviates the integration bottlenecks that previously required each Cosmos appchain to wait and integrate with the wasm client module which is yet to be upstreamed. This streamlined approach significantly simplifies the process of bringing IBC communication between new extensions and Cosmos to production, enabling seamless integration with Cosmos chains as if they were connecting with each other directly. 

By leveraging the packet forwarding midddle, Picasso functions as an IBC Hub for Cosmos SDK chains, facilitating smooth IBC transfers between every IBC extension Picasso implements.

A diagram showcasing the high-level architecture of this IBC connection can be found below:

![cosmos_polkadot_bridge_stack](../images-centauri/centauri-stack.png)

Cosmos ⬌ Polkadot transfers utilising IBC

## Pallet Multihop
Pallet Multihop cam be likened to a packet forwarding middleware for connecting with the Polkadot Relay chain and parachains. It provides the functionality to send tokens via IBC and XCM simulatenously in one call. This unlocks seamless UX for users on other parachains to execute IBC transfers without requiring to bridge to Composable or Picasso. For example, this would enable a user to send ATOM from the Cosmos Hub to Moonbeam in one click or vice versa, enable a user on Moonbeam to send GLMR to Osmosis in one click.

:::tip
With this entire IBC stack designed for blockchains built with Polkadot-SDK (previously named Substrate), implementing IBC on parachains and standalone Substrate chains has become easily portable.
:::


