# Build with Composable

Composable is designed to provide a seamless and trust-minimized infrastructure for building and deploying decentralised applications across different ecosystems. This page outlines the many opportunities for developers and infrastructure providers to leverage Composable's infrastructure, focusing on the CVM and its three core blockchains: Picasso, Composable Polkadot, and Composable Cosmos.

## Developers
Composable provides a robust development environment for creating blockchain applications. Developers have access to a CosmWasm VM, which allows for the creation of smart contracts on Composable's blockchains. The environment is tailored to work seamlessly with the IBC protocol for cross-chain compatibility.

Developers can build applications on any of Composable's three blockchains, depending on their specific requirements. They can develop pallets or deploy smart contracts using the Composable CosmWasm VM and enjoy the benefits of trust-minimized interoperability with other supported chains.

### [Composable Virtual Machine](../technology/cvm.md)
One of Composable's strengths is its extensive support for blockchain interoperability. Developers can utilize Composable's infrastructure to connect with the following chains:

- Ethereum
- Solana
- Polkadot & Kusama parachains
- Cosmos app-chains

The IBC protocol plays a crucial role in connecting these ecosystems, enabling trust-minimized communication and asset transfer.

Composable's infrastructure is built on the foundation of three high-performance blockchains, each offering unique features for blockchain development:

### [Picasso](../networks/picasso-parachain-overview.md) 
- Picasso is a Kusama parachain integrated into Composable's ecosystem.
- Developers can build and deploy their applications on Picasso using the provided infrastructure.
- Picasso supports the IBC protocol for seamless cross-chain communication.
- It utilizes the CosmWasm VM, enabling developers to create smart contracts.

### [Composable Polkadot](../networks/composable-parachain-overview.md)
- Composable Polkadot is a dedicated blockchain within the Composable network.
- It supports the IBC protocol for interoperability.
- Developers can harness the power of the CosmWasm VM for smart contract development.

### [Composable Cosmos](../networks/centauri-chain.md)
- Composable Cosmos is another blockchain within the Composable ecosystem.
- It integrates the IBC protocol for cross-chain compatibility.
- Like the others, Composable Cosmos employs the CosmWasm VM for smart contracts.

## Collators
Collators hold a pivotal role in the operation of Composable Polkadot and Picasso. Collators can be likened to a decentralised version of "shared sequencers" within the context of Polkadot and Kusama. Each collator operates as a decentralized node responsible for proposing and sequencing new blocks on Picasso and Composable Polkadot. The security of these proposed blocks is backed by the relay chain's validators, and this shared security model ensures the integrity of the both networks. 

Unlike many Layer 2 networks that confront the challenges of centralization in their sequencer systems, Polkadot provides a well-established infrastructure of decentralized shared sequencers at its core. Collators' role as decentralised shared sequencers highlights their collective responsibility in maintaining the order and integrity of transactions within the broader blockchain ecosystem while operating on individual parachains. Their active participation is underpinned by incentivization through staked collateral, and they may also be engaged in the governance of parachains.

## Validators 
In contrast to the previous section, Cosmos chains operate independently with their validators, have their security and governance models, and communicate directly with other Cosmos chains via the Inter-Blockchain Communication (IBC) protocol, forming a more self-contained and sovereign blockchain network. Validators play a critical role in securing the Composable Cosmos chain. They are responsible for maintaining the stability and security of the network. 