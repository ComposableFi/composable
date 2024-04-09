# Kusama ⬌ Polkadot IBC

The **first implementation of IBC outside of the Cosmos** ecosystem was between [Kusama](https://kusama.network/) and [Polkadot](https://polkadot.network/) via Picasso Kusama and Composable. This was a significant development as no bridge exists connecting these two relay chains. 

:::info

On Polkadot & Kusama, parachains can connect to each other using the [Cross-Chain Message Passing protocol (XCM)](https://wiki.polkadot.network/docs/learn-xcm). XCM messages are used to transfer tokens, execute smart contracts, and perform other actions across different parachains. Parachains lacked the means to communicate with each other across different relay chains until the development of IBC outside Cosmos.

:::

A diagram showcasing the components utilized in this implementation can be found below:

![kusama_polkadot_bridge_stack](../images-centauri/kusama-polkadot-bridge-stack.png)
Kusama ⬌ Polkadot transfers utilizing IBC

The shared components of the IBC implementations between Kusama ⬌ Polkadot and [Polkadot ⬌ Cosmos](../ibc/polkadot.md) is apparent from their respective architectures. This is due to the fact that the IBC protocol features a standardized structure for message passing and messaging requirements that must be adhered to

By utilizing IBC as a trust-minimised access point for DotSama to the wider cryptocurrency ecosystem, a proven and successful cross-chain communication protocol can be leveraged to enable transfers of assets between parachains in both ecosystems.

The fees for transferring between Polkadot and Kusama is 0.5% of the notional amount bridged.