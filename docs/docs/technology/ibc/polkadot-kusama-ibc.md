# Kusama ⬌ Polkadot IBC

The first implementation of Composable IBC was between [Kusama](https://kusama.network/) and [Polkadot](https://polkadot.network/) via Picasso and Composable. This was a significant development as no bridge exists connecting these two relay chains and it also marked the **first implementation of IBC outside of the Cosmos ecosystem**.

:::info

On Polkadot & Kusama, parachains can connect to each other using the [Cross-Chain Message Passing protocol (XCM)](https://wiki.polkadot.network/docs/learn-xcm). XCM messages are used to transfer tokens, execute smart contracts, and perform other actions across different parachains. Parachains lacked the means to communicate with each other across different relay chains until the development of Composable IBC.

:::

A diagram showcasing the components utilized in this implementation can be found below:

![kusama_polkadot_bridge_stack](../images-centauri/kusama-polkadot-bridge-stack.png)
Kusama ⬌ Polkadot transfers utilizing IBC

The shared components of Composable IBC's implementations between Kusama ⬌ Polkadot and other connections is apparent from their respective architectures. This is because the IBC protocol has a generalized structure for passing messages.

By utilizing IBC as a trustless access point for DotSama to the wider cryptocurrency ecosystem, we can leverage a proven and successful cross-chain communication protocol and enable transfers of assets between parachains in both ecosystems. This innovation is of significant importance in achieving Kusama's goals, which have always been focused on experimentation, developer acquisition, and paving the way for the future of DotSama.

The fees for transferring through Composable IBC are paid by utilizing the tokens being transferred, which have a value of 0.4% of the notional amount. In the future, this will be switched to a $10 fee in the case where there is a pool on Pablo. 
