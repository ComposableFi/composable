# Ethereum IBC

:::tip
The inaugral implementation of IBC on Ethereum is now live on mainnet. Users can utilise [Mantis.app](https://games.mantis.app/) to bridge between Ethereum and Cosmos chains via Picasso.
:::

Ethereum's integration with the IBC protocol expands the ability to offer novel and valuable DeFi use cases and enhance opportunities for participants across diverse ecosystems. In line with prior IBC extensions, incorporating essential components such as a light client, relayer, and IBC implementation remains a prerequisite. However, when extending IBC compatibility to Ethereum, it became imperative to supplement the relayer with a ZK-Circuit. 

Previous IBC extensions relied on ibc-rs, whereas, this particular integration utilises an IBC implementation in Solidity. The essential components powering the Ethereum IBC connection are described in the following sections.

## Light Clients
Before Ethereum transitioned to Proof of Stake (PoS) from its former consensus mechanism, Proof of Work (PoW), developing a light client was notably challenging. Building a light client for any PoW blockchain presents difficulties due to the need for resource-intensive validation of PoW, storage requirements for large block sizes, slow and bandwidth-intensive syncing and having non-deterministic finality - this means that transactions are never truly finalised so there is always the potential for someone to create a longer chain originating from a block preceding the current one, excluding it from the valid chain. These challenges arise from PoW's computational complexity and resource demands, making it more intricate to implement a light client compared to blockchains using PoS consensus mechanisms.

Composable has made substantial progress in developing light clients for networks that previously lacked them, conducting extensive research and development in this domain. In the IBC extension to Ethereum, the **Casper light client for the [Ethereum Beacon Chain](https://ethereum.org/en/roadmap/beacon-chain/#what-is-the-beacon-chain)** is used in production. This light client will be deployed on Picasso, which is already connected to the Cosmos and Polkadot ecosystems.

:::info
Casper is Ethereum's PoS consensus protocol implemented within ETH 2.0. In Casper, validators participate in block creation and validation based on the amount of stake they lock up as collateral. This PoS system was adopted to enhance the security, scalability, and energy efficiency of Ethereum by reducing the need for computationally intensive mining while rewarding validators with transaction fees.
:::

The Casper Light Client relies on the **Sync Committee**, which comprises 512 validators who undergo random selection once every sync committee period, approximately equal to one day. While a validator is actively participating in the sync committee, their primary responsibility entails consistently signing the block header representing the current chain head during each slot. The Sync Committee is a succinct way of using a sample to verify a subset of the signatures of Ethereum. 

Previously, validating the Tendermint consensus protocol within the EVM posed a challenge due to the absence of an Ed25519 precompile, the default signature scheme used in Cosmos chains using the Tendermint consensus. Existing Solidity implementations for this verification incurred gas costs averaging around 25 million. However, zero-knowledge proofs serve as a precompile for the EVM, enabling developers to integrate highly intricate computations seamlessly. Picasso's zkIBC bridge to Ethereum employs [TendermintX](https://github.com/succinctlabs/tendermintx) to verify all signatures in succinct proofs. 

The deployment of smart contracts and circuits by Succinct for TendermintX is operational in production, with each client update for the IBC implementation on Ethereum costing approximately 650,000 gas. These client updates include headers of Picasso and data such as block hashes, timestamps, and state roots. 

:::info
The Ed25519 signature scheme is a cryptographic algorithm used for digital signatures and adopted by majority of Blockchains. It is based on the elliptic curve cryptography and provides strong security with relatively short key sizes. The scheme is named after the Edwards curve Curve25519, which is used as the underlying mathematical structure. Ed25519 offers efficient signing and verification operations, making it popular for various applications such as secure communication protocols, cryptocurrencies, and digital authentication systems.
:::

## IBC implementation in Solidity
Building upon the existing IBC implementation in Solidity by [Hyperledger Labs](https://github.com/hyperledger-labs/yui-ibc-solidity), the contracts that have been deployed are optimised for a production-ready environment tailored for Ethereum. 

The ownership of the IBC contracts deployed on Ethereum for Picasso is presently held by a [team multisig wallet](https://etherscan.io/address/0xcbcfccb93b14e5cc55917a56f67f419f259e0813), with plans to transition control to PICA governance in the forthcoming release. This decision reflects the unprecedented nature of implementing IBC for the first time on Ethereum, emphasizing the need for a stable initial launch. Updates regarding any contract upgrades will be shared in our Discord community.
