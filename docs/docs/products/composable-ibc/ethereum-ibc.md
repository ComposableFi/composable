# Ethereum IBC

Ethereum's integration with the IBC protocol expands the ability to offer novel and valuable DeFi use cases and enhance opportunities for participants across diverse ecosystems.

In line with prior IBC extensions, the incorporation of essential components such as a light client, relayer, and IBC implementation remains a prerequisite. However, when extending IBC compatibility to Ethereum, it became imperative to supplement the relayer with a ZK-Circuit. Previous IBC extensions were reliant on ibc-rs, whereas, this particular integration utilises an IBC implementation in Solidity.

## Components

The essential components powering the Ethereum IBC connection are described in the following section.

### Light Clients
Before Ethereum's transition to Proof of Stake (PoS) from its former consensus mechanism, Proof of Work (PoW), the development of a light client was notably challenging. Building a light client for any PoW blockchain presents difficulties due to the need for resource-intensive validation of PoW, storage requirements for large block sizes, slow and bandwidth-intensive syncing and trust issues with potentially malicious full nodes. These challenges arise from PoW's computational complexity and resource demands, making it more intricate to implement a light client compared to blockchains using PoS consensus mechanisms.

Composable has made substantial progress in developing light clients for networks that previously lacked them, conducting extensive research and development in this domain. Composable sees the contribution made to Ethereum not only by connecting it to Cosmos, Polkadot, and other chains via IBC, but also by creating a **Casper light client for the [Ethereum Beacon Chain](https://ethereum.org/en/roadmap/beacon-chain/#what-is-the-beacon-chain)** within an accelerated timeline. This light client will be deployed on the Composable Cosmos chain which is already connected to the Cosmos and Polkadot ecosystems.

the sync committee is a succinct way of getting a sample to verify all the signatures of ethereum 

:::info
Casper is Ethereum's PoS consensus protocol implemented within ETH 2.0. In Casper, validators participate in block creation and validation based on the amount of stake they lock up as collateral. This PoS system was adopted to enhance the security, scalability, and energy efficiency of Ethereum by reducing the need for computationally intensive mining while rewarding validators with transaction fees.
:::

Composable will implement a Tendermint light client on Ethereum by deploying it as a smart contract. The Tendermint light client is responsible for verifying the validity of block headers from the Composable Cosmos chain and ensures that the consensus state of the source blockchain is consistent with the header being presented. These headers contain data such as block hashes, timestamps, and consensus state. It will be too expensive to verify signatures from  validator set of the Composable Cosmos chain therefore, ZK-Snarks are used to verify all signatures in succinct proofs. Read more on light clients [here](light-clients.md).

The ideal security to establish is for an attack to be as expensive as the smaller market cap of DOT and ETH. Unfortunately, only the bond of the few validators whose signatures are verified can be slased, so any attack attempt can be cheaper than the whole market cap. 

### IBC implemntation on Ethereum
Composable is building upon the existing IBC protocol implementation in Solidity by [Hyperledger Labs](https://github.com/hyperledger-labs/yui-ibc-solidity). The contracts have been iterated upon and optimised for a production ready environment tailored for Ethereum. 

### Hyperspace Relayer
Within the [Hyperspace](hyperspace-relayer.md) connection to Ethereum, a zero-knowledge (ZK) circuit is employed to optimise and sustain the cost of the relayer. A ZK circuit is a program capable of generating a proof when given a set of inputs. This proof can then be verified to ensure the correctness of each computational step executed within the circuit. 

The ZK ciruit involves a **prover** and **verifier**. Similar to Hyperspace, the prover is a permissionless off-chain component that can be run by anyone. The verifier lives on-chain as a smart contract on Ethereum. The role of the verifier is to verify proofs submitted by the prover, in this case, a succinct proof of client updates from the Composable Cosmos chain.

:::tip
Verifiers receive proofs from provers and subsequently validate these claims, resulting in the generation of zero-knowledge proofs. For an in-depth explanation of this process, [read here](https://ethereum.org/en/developers/docs/zksnarks).
:::