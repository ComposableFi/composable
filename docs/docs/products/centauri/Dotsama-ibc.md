# DotSama IBC
_IBC beyond the Cosmos_
## Kusama ⬌ Polkadot Implementation

The first implementation of Centauri is between [Kusama](https://kusama.network/) and [Polkadot](https://polkadot.network/) via Picasso and Composable. This is a significant development because currently there is no bridge connecting these two relay chains. This also marks the **first implementation of IBC outside of the Cosmos ecosystem**.

:::info

On Polkadot & Kusama, parachains can connect to each other using the [Cross-Chain Message Passing protocol (XCM)](https://wiki.polkadot.network/docs/learn-xcm). XCM messages are used to transfer tokens, execute smart contracts, and perform other actions across different parachains. Parachains lacked the means to communicate with each other across different relay chains until the development of Centauri.

:::

A diagram showcasing the components utilized in this implementation can be found below:

![kusama_polkadot_bridge_stack](../images-centauri/kusama-polkadot-bridge-stack.png)
Kusama ⬌ Polkadot transfers utilizing IBC

The shared architecture of Centauri's IBC implementations between Kusama ⬌ Polkadot and other connections is apparent from their respective architectures. This is because the IBC protocol has a generalized structure for passing messages.

By utilizing IBC as a trustless access point for DotSama to the wider cryptocurrency ecosystem, we can leverage a proven and successful cross-chain communication protocol and enable transfers of assets between parachains in both ecosystems. This innovation is of significant importance in achieving Kusama's goals, which have always been focused on experimentation, developer acquisition, and paving the way for the future of DotSama.

## Cosmos ⬌ DotSama Bridge

IBC is the native communication protocol of the Cosmos ecosystem, it allows for trustless communication to occur between any blockchains built using the Cosmos SDK which have the IBC module enabled. For this connection, we will be using Tendermint and Grandpa as light clients on Picasso and Cosmos chains, respectively.

To connect the grandpa client implementation in Cosmos with the existing IBC stack, we are [extending the IBC specification](https://github.com/cosmos/ibc/pull/901) to include the `10-wasm-light-client module`. This module serves as an adapter between CosmWasm light clients and the ibc-go stack, allowing for easy integration. This update enables light clients from other ecosystems to be easily ported into Cosmos with minimal difficulty. This module is included in the Cosmos SDK 0.47 and ibc-go v7 upgrades.

As of now, most Cosmos chains have yet to upgrade to the required versions of SDK v0.47.0 and ibc-go v7, which makes it challenging for them to support Centauri. To address this, we are creating a workaround solution that enables packet forwarding between Picasso and Cosmos chains. This would simplify the connection process between the two ecosystems without requiring the Cosmos chains to undergo significant upgrades. Essentially, Picasso will be able to communicate with Cosmos chains as if it was anither Cosmos chain, without any technical requirements needed from the counterparty chains.

A diagram showcasing the high-level architecture utilized in this implementation can be found below:

![cosmos_polkadot_bridge_stack](../images-centauri/centauri-stack.png)

Cosmos ⬌ DotSama transfers utilizing IBC

We are currently working with [Notional DAO](https://notional.ventures/) on the DotSama <-> Cosmos testnet, making strides in establishing an efficient IBC-enabled connection for users of both ecosystems. If you are a Cosmos chain, validator, or relayer and wish to participate in the testnet efforts, please contact our Community Managers on [Discord](https://discord.gg/composable).
