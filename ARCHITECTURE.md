# Architecture

Composable is a [Polkadot parachain](https://wiki.polkadot.network/docs/learn-parachains) built with [Substrate](https://substrate.dev/). We provide a wide variety of DeFi protocols and cross-chain capabilities.

# Overview

The Composable project consists of a blockchain, various utility applications, deployment scripts and setup scripts.

# Runtimes

When building the chain, we use different names to target different [relay chain](https://wiki.polkadot.network/docs/learn-architecture):
- the **Dali** chain is deployed for **Westend/Rococo/Chachacha**
- the **Picasso** chain is deployed for **Kusama**
- the **Composable** chain is deployed for **Polkadot**

The runtimes can be found under the `runtime` directory.
The chain specs are located in `node/src/chain_spec`.

# Pallets

The building blocks of a substrate blockchain are **pallets**, they define the capability and allow us to implements DeFi protocols.
You can find them under the `frame` directory. We try to give them a meaningful name.

# Utilities in `utils`

## Price feed server

The `oracle` off-chain worker is fetching prices from a server. We provide a reference implementation named `price-feed` that fetches and cache prices from the official binance API.

## Subxt clients

We do our best to maintain generated subxt clients for our runtimes.

# Scripts

The `scripts/polkadot-launch` scripts are setting up a whole Relay + Composable + Basilisk environment.
Various [polkadotjs](https://polkadot.js.org/docs/) scripts located in `setup` are helping us testing the pallets once the local node is up and running.
