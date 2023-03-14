# ARCHITECTURE

The Composable project consists of a blockchain, various utility applications, deployment scripts and setup scripts.

Composable is a [Polkadot parachain](https://wiki.polkadot.network/docs/learn-parachains) built with [Substrate](https://substrate.dev/). We provide a wide variety of DeFi protocols and cross-chain capabilities.

## Runtimes

When building the chain, we use different names to target different [relay chain](https://wiki.polkadot.network/docs/learn-architecture).

The runtimes can be found under the `runtime` directory.
The chain specs are located in `node/src/chain_spec`.

## Pallets

The building blocks of a substrate blockchain are **pallets**. They define the capability and allow us to implement DeFi protocols.
Write
You can find them under the `frame` directory. We try to give them a meaningful name.

## Utilities in `../utils`

### Price feed server

The `oracle` off-chain worker is fetching prices from a server. We provide a reference implementation named `price-feed` that fetches and caches prices from different, configurable sources.

## Scripts

The `scripts/polkadot-launch` scripts are setting up a whole Relay + Composable + Basilisk environment.
Various [polkadotjs](https://polkadot.js.org/docs/) scripts located in `setup` are helping us testing the pallets once the local node is up and running.
