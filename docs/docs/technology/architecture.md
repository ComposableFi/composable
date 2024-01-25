# Architecture

Composable Finance is dedicated to **enhancing the cross-blockchain infrastructure in decentralized finance via trust-minimized bridging and order flow optimization**.

Composable’s architecture is thoughtfully constructed to actualize this mission. This includes three different blockchains serving their own unique purposes. The architecture also includes trust-minimized bridging via the IBC protocol, which not only connects Composable’s blockchains, but connects other ecosystems as well. The Composable Technical stack also includes the Composable Virtual Machine (CVM) and MANTIS, which both serve roles in transaction execution.

These different components collectively facilitate a complete, end-to-end ecosystem for cross-chain DeFi. In brief, the core components of the technical architecture are as follows:

## [MANTIS](mantis.md)
Multichain Agnostic Normalized Trust-minimized Intent Settlement (MANTIS) is an ecosystem-agnostic intent settlement framework. It is the culmination of all other elements mentioned in this page; it uses Picasso's trust minimized bridge (IBC connections) as well as the CVM to facilitate cross-chain execution of user intents. As a result, users can interact with a novel, intuitive, and streamlined experience for participating in cross-chain DeFi. 
## [Composable VM](cvm.md)
In the blockchain industry, virtual machines play a key role in executing smart contracts, processing transactions, and ensuring the overall integrity of the chain. Existing virtual machines are designed for specific blockchains, such as the Ethereum Virtual Machine (EVM) being designed for the Ethereum chain.
 
The Composable Virtual Machine (CVM) plays a different role in that it was constructed as a solution to carry out cross-chain operations. The CVM is an orchestration language and execution runtime for cross-chain program execution and intent settlement over IBC. With the CVM, developers are no longer restricted to one blockchain ecosystem and instead are able to execute cross-chain operations in one user-signed transaction.

## [The IBC Protocol](ibc.md)

Dozens of bridging solutions exist in DeFi, but only two transport layers live in production are actually trust-minimised, XCMP and IBC. [The Inter-Blockchain Communication (IBC) Protocol](https://www.ibcprotocol.dev/) is one such superior bridging solution; this protocol facilitates trust-minimized cross-chain bridging. While it was originally designed to bridge Cosmos-native chains (e.g. Cosmos SDK chains and the Cosmos Hub), the trust-minimized nature of the IBC in addition to a number of other benefits motivated Composable to leverage this protocol as the basis of [cross-chain bridges](https://www.trustless.zone/) - and therefore the primary means of connection between the various components in the Composable ecosystem.

## [Composable Cosmos](../networks/composable-cosmos.md) 

Composable opted to deploy a Cosmos SDK chain as it enabled the customisation of consensus and other feature sets. Moreover, a Cosmos chain is the most seamless method of interacting with other existing Cosmos chains. Composable Cosmos enables far more efficient connections between IBC outside of Cosmos and existing Cosmos-based chains. Composable Cosmos currently serves as the IBC hub of the ecosystem, connecting Cosmos and expanding the Interchain with chains that are otherwise IBC-incompatible, such as those with non-Tendermint consensus implementations like Polkadot, Ethereum and Solana. Composable Cosmos's native token is PICA.

## [Picasso](../networks/picasso-parachain-overview.md)

Picasso is Composable’s Kusama parachain. Having a Kusama parachain enabled an IBC bridge to Kusama. Moreover, Picasso serves as the infrastructure layer powering the rest of the technical stack, offering primitives that support the ecosystem such as the [Pablo decentralized exchange](https://www.pablo.finance/).

Picasso’s native token is also PICA. Its use current cases include collator staking on Picasso, Apollo staking, and validator staking. In the future, use cases will also include liquid staking revenue and bridging revenue. Moreover, [PICA is used for governing Picasso](../networks/picasso/governance.md).

## [Composable Polkadot](../networks/composable-parachain-overview.md)

Composable Polkadot is a parachain on the Polkadot network. Similarly to Picasso enabling us to seamlessly connect a trust-minimized bridge to Kusama, Composable Polkadot allows us to seamlessly connect the bridge to Polkadot. Moreover, Composable Polkadot inherits the security of the Polkadot relay chain, and thus offers an incredibly secure environment. Furthermore, having both a Kusama and Polkadot parachain has allowed us to connect the DotSama space at a greater level than ever before.

Composable Polkadot’s native token is [LAYR](../networks/composable/LAYR-tokenomics.md). Its use cases include serving as the universal gas token for Composable, facilitating flash loans and borrowing, and participating in restaking, collator staking, and OpenGov on Composable Polkadot. 

