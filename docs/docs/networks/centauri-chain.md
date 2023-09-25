# Centauri

_This page provides an overview of the Centauri chain, which is a Tendermint-based blockchain serving as a core module of [Centauri](../products/composable-ibc.md) - Composable's IBC-based transfer protocol._

## Background
The motivation behind establishing this blockchain is the realization that building bridges to each individual Cosmos chain is a time-consuming process. Currently, we need to wait for Cosmos chains to upgrade to SDK v0.47 and the IBC v7 update which includes the ICS-8 Wasm module (currently unreleased) and integrating the Grandpa light client written in CosmWasm. **Furthermore, this chain serves a critical role in our future IBC connections as we expand the reach of Centauri.**

If Cosmos chains choose to establish a direct connection and not to flow assets via the Centauri chain, it is more than possible. However, the drawback is that we would need to wait for Cosmos chains to upgrade to Cosmos SDK v0.47 and a later iteration of IBC v7 which includes the ICS-8 Wasm Client (currently unreleased). On the other hand, the Cosmos chains we connect with through the Centauri chain to Polkadot and Kusama enjoy the advantage of cross-ecosystem communication and an IBC connection as soon as the Centauri chain is launched. 

When assets are transferred through the Centauri chain, they will utilize a Picasso channel with the Centauri chain denom. Consequently, once Cosmos chains fulfill the requirements to support a direct connection to Picasso, we can smoothly migrate users' assets to the Picasso denom.

The Centauri chain will be compatible with any Cosmos chain running Tendermint consensus and [Picasso](./picasso-parachain-overview.md), thus acting as a hop chain between DotSama and Cosmos. Furthermore, the blockchain will power all IBC activities outside of Cosmos, facilitating transfers to Polkadot and Kusama upon launch, and later to ETH 2.0 and NEAR once we establish IBC on those blockchains. 

The chain's core functionality is powered by Strangelove’s [packet forward middleware](https://github.com/strangelove-ventures/packet-forward-middleware). There are no user-facing activities on the Centauri chain besides governance and the middleware allows us to construct one connection from Picasso to the Centauri chain. This chain will seamlessly connect our chain to the other Cosmos chains by deploying relayers and opening channels. The Centauri chain will not have its own native token but will share the total supply with Picasso and Composable’s native tokens, PICA & LAYR to secure and govern the chain.

## Rewards
The maximum size of the active validator set is 100, and our intention is to delegate 1 billion PICA tokens from Picasso to Centauri for validator staking and rewards. It's important to note that the staked percentage affects the annual percentage rate (APR) in an inverse manner, meaning that a higher staked percentage results in a lower APR.

As per [Council Motion 26](https://picasso.polkassembly.io/motion/26). This proposal is to enable the transfer for 1,066,669,217.17 PICA tokens on the Picasso network to be transferred to the Centauri chain upon its mainnet launch from the Picasso Treasury to an escrow address. Centauri will then mint 1,066,669,217.17 PICA. We will not be burning PICA on Picasso, but instead sending it to the escrow address and once the transfer protocol opens, they will only be unescrowed if the initial PICA supply minted on Centauri is sent back to Picasso.
 
Tokens will be directed as follows: validator genesis (2550.5 PICA), delegations + voting stake (1bn PICA) , and rewards (66,666,667). This operation will maintain the current total supply of PICA at 10bn.

Once the Picasso connection is established, the remaining 9 billion PICA tokens can also be transferred to Centauri, allowing users to delegate to those validators as well. The rewards mentioned earlier are distributed among the staked users, including both delegators and validators. Any new validator can join the active set and remove an existing genesis validator should they hold a higher stake.

A list of the active validator set of Centauri can be found [here](https://ping.pub/composable/staking).

## Governance
Similar to the way Cosmos chains operate, validators can vote using tokens they are delegated with. However, if the delegator votes themselves, it will override the validator’s vote. The parameters for the governance on the Centauri chain are as follows:

| Parameter                                          | Period/Number  |
|----------------------------------------------------|----------------|
| Total Deposit                           | 2 days          |
| Quorum          | 30%         |
| Voting Period | 5 days        |
| Threshold                | 50% |
| No-with-veto                             |  33%   |