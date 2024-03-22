# What is Picasso?

Picasso is a DeFi infrastructure-focused Layer 1 protocol that leads the industry in building the trust-minimized interoperability solution -**Cross-Ecosystem IBC**. Complementary to the interoperability work, Picasso is building the first **Generalized Restaking Layer** starting with deployment on Solana, and expand support for all IBC connected ecosystems.

In short, Picasso L1 is the **first censorship-resistant, trust-minimized interoperability solution (Cross-Ecosystem IBC Hub), and also a platform for the first Generalized Restaking Layer.**

The Picasso Layer 1 (L1) is a [Cosmos SDK](https://v1.cosmos.network/sdk) blockchain that acts as an [Inter-Blockchain Communication (IBC)](https://www.ibcprotocol.dev/) Protocol hub between Cosmos and non-Cosmos IBC-enabled chains. Picasso’s Generalized Restaking Layer, paired with IBC, can utilize any economically valuable assets to secure any use cases that require temporary or permanent security, on any IBC connected chain. Picasso L1 will also serve as the accounting hub for slashing and rewards. Through this infrastructure, Picasso will deliver native protocol level security and permissionless interoperability for all Decentralized Finance (DeFi) users, and allow more assets across all major ecosystems to be used as security for any Proof of Stake (POS) based protocols.

## Security & Consensus

The Picasso L1 is on mainnet deployment as a Cosmos SDK chain with 48 validators. It runs the [CometBFT](https://cometbft.com/) consensus, making it innately compatible with the Goloang implementation of the IBC protocol.

The validators will secure Picasso L1 through (1) Picasso (PICA) token staking, (2) Picasso Generalized Restaking Layer detailed in a subsequent section, and (3) a collaboration with Ethos bringing EigenLayer security to Cosmos.

The consensus of Picasso L1 will be a modification of Kauri, a Byzantine Fault Tolerant (BFT) communication abstraction that leverages dissemination/aggregation trees for load balancing and scalability. Composable is working to make Kauri Tendermint-enabled, as described here. Thus, Kauri will be able to operate with IBC.

## Key Features

### Cross-Ecosystem IBC 

The Inter-Blockchain Communication (IBC) Protocol is a protocol for natively secured communication between different blockchain networks/ecosystems. This reliance on the native protocol security of the two transacting chains is a core advantage of the IBC protocol, and a large reason why we selected the IBC as a cornerstone of Composable's efforts to connect ecosystems, in addition to its performance and scalability. 

Originally, the IBC Protocol was tailored for native connections within the Cosmos Ecosystem, specifically among Layer 1 protocols that adopted the Cosmos SDK and IBC capabilities. Extending this to include more networks required significant development effort.
In fact, Polkadot IBC (DOT IBC), Kusama IBC (KSM IBC), Ethereum IBC (ETH IBC), and Solana IBC (SOL IBC) all require respective customization from implementation of light client to  achieving state proof (finality), in order to not rely on any centralized intermediary and uphold the IBC values of censorship resistance and native security.

These connections allow the transfer of both fungible (e.g Tokens)  and non-fungible tokens (NFTs), generic message passing, cross-chain contract calls, cross-chain fee payments, and cross-ecosystem collateralization, all executed in a natively secured environment.

In the next phase of our development, the Picasso L1 will enable “cross-domain slots”: blocks created across multiple chains simultaneously. Multiple domains will be sequenced at the same time. Specifically, sequencers can build blocks and send them to the Picasso L1, creating corresponding blocks on multiple chains. This opens up many new opportunities for developers to perform seamless cross-ecosystem execution and settlement, and bring in a new age of Decentralized Finance (DeFi) that the industry has never seen before. 

## Generalized Restaking Layer

Restaking enables users to provide blockchain security through collateralizing the economic value of their liquid staked tokens and yield-bearing assets, in exchange for validating the transactions of Proof of Stake (POS) protocols, also called Actively Validated Services (AVSes) that seek security.

Picasso expands upon the concept of restaking popularized by Eigenlayer to deliver Generalized Restaking: restaking of diverse assets on multiple Proof of Stake (PoS) networks to establish cross-ecosystem pooled security. This is made possible by Picasso’s IBC connections (link to above) and the flexible architecture of Generalized Restaking Layer on Picasso.


We believe that all assets that have underlying utility, community, and liquidity, given long enough horizon, will have various level of economic value that can be used to enhance security guarantees for many use-cases (interoperability, oracles, sequencers and more) built upon the Proof of Stake consensus mechanism.
