# What is Picasso?

Picasso is a DeFi infrastructure-focused Layer 1 protocol that leads the industry in building the trust-minimized interoperability solution -**Cross-Ecosystem IBC**. Complementary to the interoperability work, Picasso is building the first **Generalized Restaking Layer** starting with deployment on Solana, and expand support for all IBC connected ecosystems.

In short, Picasso is the **first censorship-resistant, trust-minimized interoperability solution (Cross-Ecosystem IBC Hub), and also a platform for the first Generalized Restaking Layer.**

Picasso is built using the [Cosmos SDK](https://v1.cosmos.network/sdk) framework and acts as an [Inter-Blockchain Communication (IBC)](https://www.ibcprotocol.dev/) Protocol hub between Cosmos and non-Cosmos IBC-enabled chains. Picasso’s Generalized Restaking Layer, paired with IBC, can utilize any economically valuable assets to secure any use cases that require temporary or permanent security. Through this infrastructure, Picasso will deliver native protocol level security and permissionless interoperability for all Decentralized Finance (DeFi) users, and allow a diverse set of assets across all major ecosystems to be used as security for any blockchain services and [Proof of Stake (PoS)](https://blog.cosmos.network/understanding-the-basics-of-a-proof-of-stake-security-model-de3b3e160710) based protocols.

## Security & Consensus

Picasso is on mainnet deployment as a Cosmos SDK chain with 48 validators and runs the [CometBFT](https://cometbft.com/) consensus. The validators secure Picasso through (1) Picasso (PICA) token staking, (2) Picasso Generalized Restaking Layer detailed in a subsequent section, and (3) a collaboration with [Ethos](https://ethosstake.com/) bringing [EigenLayer](https://www.eigenlayer.xyz/) security to Cosmos.

There are plans to modify the consensus of Picasso to Kauri, a Byzantine Fault Tolerant (BFT) communication abstraction that leverages dissemination/aggregation trees for load balancing and scalability. The Composable Research team is working to make Kauri Tendermint-enabled, as described [here](https://research.composable.finance/t/rfp-5-a-fast-consensus-for-cosmos-sdk-chains/304). Thus, Kauri will be able to operate with IBC.

## Key Features

### Cross-Ecosystem IBC 

The IBC Protocol is used for trust-minimized communication between different blockchain networks/ecosystems. It relies on security at the consensus level of the two transacting chains, and this is a large reason why IBC was selected as a cornerstone of Picasso's efforts to connect ecosystems, in addition to its performance and standardization. 

Originally, IBC was tailored for native connections between Cosmos SDK chains. Extending this to include more networks required significant development effort. In fact, IBC on Solana, Ethereu, Polkadot and Kusama all require respective customization from implementation of light client to achieving state proof (finality), in order to not rely on any centralized intermediary and uphold the IBC standards and values of censorship resistance.

These connections allow the transfer of both fungible (e.g Tokens) and non-fungible tokens (NFTs), generic message passing, cross-chain contract calls, cross-chain fee payments, and cross-ecosystem collateralization, all executed in a natively secured environment.

### Generalized Restaking Layer

Restaking enables users to provide blockchain security through collateralizing the economic value of their liquid staked tokens and yield-bearing assets, in exchange for validating PoS protocols and services, also called Actively Validated Services (AVSes) that seek security.

Picasso expands upon the concept of restaking popularized by [Eigenlayer](https://docs.eigenlayer.xyz/eigenlayer/overview/) to deliver Generalized Restaking: restaking of assets on multiple PoS networks to establish **cross-ecosystem pooled security**. This is made possible by Picasso’s IBC connections and the flexible architecture of Generalized Restaking Layer on Picasso.

We believe that all assets that have underlying utility, community, and liquidity, given long enough horizon, will have various level of economic value that can be used to enhance security guarantees for many use-cases (interoperability, oracles, sequencers and more) built upon the Proof of Stake consensus mechanism.

## Governance
Similar to the way that majority of Cosmos SDK chains operate, validators can vote using tokens they are delegated with. However, if a delegator votes themselves, their voting decision will override the validator’s vote if the decision differs. The parameters for the governance on Picasso are as follows:

| Parameter                                          | Period/Number  |
|----------------------------------------------------|----------------|
| Total Deposit                           | 2 million PICA          |
| Quorum          | 30%         |
| Voting Period | 1 day        |
| Threshold                | 50% |
| No-with-veto                             |  33%   |