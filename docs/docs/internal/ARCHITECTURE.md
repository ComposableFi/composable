# ARCHITECTURE

The Composable project consists of a blockchain, various utility applications, deployment scripts and setup scripts.

Composable is a [Polkadot parachain](https://wiki.polkadot.network/docs/learn-parachains) built with [Substrate](https://substrate.dev/). We provide a wide variety of DeFi protocols and cross-chain capabilities.

## Runtimes

When building the chain, we use different names to target different [relay chain](https://wiki.polkadot.network/docs/learn-architecture):

| Chain and Runtime | Deployed to       | Relayer Native Currency | Link                                                                                                      | Docs                                          | Index(History) | 
| ----------------- | ----------------- | ----------------------- | --------------------------------------------------------------------------------------------------------- | --------------------------------------------- |  -------------------|
| Dali              | Devnet(own Relay) | KSM                     | https://polkadot.js.org/apps/?rpc=wss://dali.devnets.composablefinance.ninja/parachain/alice#/explorer    | https://dali.devnets.composablefinance.ninja/ | https://dali.stg.subscan.io/ |
| Picasso           | Devnet(own Relay) | KSM                     | https://polkadot.js.org/apps/?rpc=wss://picasso.devnets.composablefinance.ninja/parachain/alice#/explorer |                                               | https://composable-picasso-staging.vercel.app/ |
| Dali              | Westend           | WND(DOT)                |                                                                                                           |                                               |
| Dali              | Rococo            | ROT(KSM)                | https://polkadot.js.org/apps/?rpc=wss://rpc.composablefinance.ninja                                       |                                               |
| Dali              | Chachacha         |                         |                                                                                                           |                                               |
| Picasso           | Kusama            | KSM                     | https://polkadot.js.org/apps/?rpc=wss%3A%2F%2Fpicasso-rpc.composable.finance#/explorer                    |                                               | https://picasso.stg.subscan.io/ |
| Composable        | Polkadot          | DOT                     |                                                                                                           |                                               | https://composable.stg.subscan.io/ | 

The runtimes can be found under the `runtime` directory.
The chain specs are located in `node/src/chain_spec`.

## Pallets

The building blocks of a substrate blockchain are **pallets**. They define the capability and allow us to implement DeFi protocols.
Write
You can find them under the `frame` directory. We try to give them a meaningful name.

## Utilities in `../utils`

### Price feed server

The `oracle` off-chain worker is fetching prices from a server. We provide a reference implementation named `price-feed` that fetches and caches prices from different, configurable sources.

### Rust clients

You may run `subxt` against our runtimes to get Rust clients.

## Scripts

The `scripts/polkadot-launch` scripts are setting up a whole Relay + Composable + Basilisk environment.
Various [polkadotjs](https://polkadot.js.org/docs/) scripts located in `setup` are helping us testing the pallets once the local node is up and running.
