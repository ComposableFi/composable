# How to transfer assets between Picasso & Osmosis via Centauri

Centauri is the first IBC-based transfer protocol that operates outside of the Cosmos ecosystem. This guide will demonstrate how to transfer assets between Picasso and Osmosis using Centauri. Being the pioneer light-client bridging protocol that enables asset transfers between a chain running the Tendermint client and the Grandpa client, **Centauri facilitates trust-minimized asset transfers between parachains on Polkadot/Kusama and the Cosmos ecosystem.**

In this guide, we'll be using Osmosis's native token OSMO as an example to transfer assets between Osmosis and Picasso.

1. Head to https://app.trustless.zone/multihop/ and connect both, your Polkadot and Cosmos wallets.

![transfer](./images-centauri-chain/stake-1.png)

2. Enter the amount of OSMO you would like to transfer and click 'Transfer'. A pop-up asking you to sign your transaction will appear, approve the transaction.

![notification](./images-centauri-chain/stake-2.png)

Transactions can take up to 3-5 minutes to complete, hit the 'Refresh Balances' button at any time you'd like to see a live update. 

:::note
We are implementing a batching process which enables parachains to send assets directly to Cosmos chains and vice-versa.
:::

