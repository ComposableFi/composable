# Composable Cosmos

_This page provides an overview of the Composable Cosmos chain, which is a Tendermint-based blockchain serving as a core component of [Composable IBC](../technology/composable-ibc.md)._

## Background
The motivation behind establishing this blockchain is the realization that building new IBC connections to each individual Cosmos chain is a time-consuming process. Initially, we were required to wait for Cosmos chains to upgrade to SDK v0.47 and a future IBC v7 update which includes the ICS-8 Wasm module (currently unreleased) in order to integrate the Grandpa light client written in CosmWasm. **Furthermore, this chain serves a critical role for future IBC connections as the reach of Composable IBC is expanded.**

If Cosmos chains choose to establish a direct connection and not to flow assets via Composable Cosmos, it is more than possible. However, it is important to note that the benefit of connecting new IBC connections via Composable Cosmos allows existing Cosmos chains to instantly enjoy the advantage of cross-ecosystem IBC communication and without requiring any chain upgrades or changes.  

Composable Cosmos is compatible with any IBC-enabled Cosmos chain running the Tendermint consensus, [Picasso](./picasso-parachain-overview.md) and Ethereum. **Thus acting as an IBC Hub between Cosmos chains and non-Cosmos IBC-enabled blockchains.**

The chain's core functionality is to serve as a hop chain between IBC Cosmos and utilises Strangelove’s [packet forward middleware](https://github.com/strangelove-ventures/packet-forward-middleware) for a seamless user-experience. There are no user-facing activities on the Composable Cosmos besides governance. Composable Cosmos will not have its own native token but will be powered by the native token of Picasso, PICA.

## Rewards
The maximum size of the active validator set is 100. The Composable Foundation will delegate PICA tokens from Picasso to Composable Cosmos for validator staking and rewards. It's important to note that the staked percentage affects the annual percentage rate (APR) in an inverse manner, meaning that a higher staked percentage results in a lower APR.

As per [Council Motion 26](https://picasso.polkassembly.io/motion/26). This proposal is to enable the transfer for 1,066,669,217.17 PICA tokens on the Picasso network to be transferred to the Composable Cosmos upon its mainnet launch from the Picasso Treasury to an escrow address. Centauri will then mint 1,066,669,217.17 PICA. PICA was not burned on Picasso, but instead sent to the escrow address and once the transfer protocol opens, they will only be unescrowed if the initial PICA supply minted on Composable Cosmos is sent back to Picasso.
 
During the genesis event, tokens were directed as follows: validator genesis (2550.5 PICA), delegations + voting stake (1bn PICA) , and rewards (66,666,667). This operation will maintain the current total supply of PICA at 10bn.

As the Picasso connection has now been established, the remaining 9 billion PICA tokens can also be transferred to Composable Cosmos, allowing users to delegate to those validators as well. A guide for this process can be found [here](../user-guides/composable-cosmos-staking.md).The rewards mentioned earlier are distributed among the staked users, including both delegators and validators. Any new validator can join the active set and remove an existing genesis validator should they hold a higher stake.

A list of the active validator set of Composable Cosmos can be found [here](https://ping.pub/composable/staking).

## Governance
Similar to the way majority of Cosmos chains operate, validators can vote using tokens they are delegated with. However, if a delegator votes themselves, their voting decision will override the validator’s vote in the case where the . The parameters for the governance on the Composable Cosmos are as follows:

| Parameter                                          | Period/Number  |
|----------------------------------------------------|----------------|
| Total Deposit                           | 2 days          |
| Quorum          | 30%         |
| Voting Period | 5 days        |
| Threshold                | 50% |
| No-with-veto                             |  33%   |