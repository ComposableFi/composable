# Inter-Blockchain Communication Protocol (IBC)

In pursuit of its core mission, Composable has made significant strides in developing trust-minimised bridges to facilitate decentralised and non-custodial cross-chain transactions. Composable's approach involves pioneering the extension of the Inter-Blockchain Communication (IBC) protocol and thus establish the framework as the industry standard for cross-ecosystem communication.

While IBC was initially designed for trust-minimised bridging for Cosmos SDK chains, Composable is the first team to extend this protocol to different ecosystems, marking a significant milestone in the cross-chain communication landscape. Composable's innovation is not limited to the Cosmos ecosystem but rather actively leveraging and extending the IBC protocol beyond Cosmos, pushing its boundaries beyond its original scope. 

Composable stands as a pioneer in successfully implementing IBC connectivity between Polkadot and Kusama to Cosmos, including a testnet connection to Ethereum. Future endeavors are aimed at implementing IBC across various ecosystems, ensuring that trust-free cross-chain interactions are accessible to a broader developer and user base. The roadmap includes integrating with user-intensive blockchains such as Solana and Layer 2 networks.

:::tip Trustless Zone Is Live

Use [Trustless Zone](https://app.trustless.zone/) now to transfer assets between Polkadot, Kusama and the Cosmos. IBC to Ethereum is currently in Testnet stages and IBC to Solana is in active development.
:::


IBC supports asset transfers (fungible tokens, non-fungible tokens), generic message passing, cross-chain contract calls, cross-chain fee payments, interchain collateralization and more in a trustless manner. The trustless condition of IBC is due to the fact it is:

- built upon light clients that communicate with each other and updates the state of a counterparty chain. These are lightweight versions of blockchain nodes that can verify specific information on counterparty chains without downloading the entire blockchain history. This allows for trustless verification of data and state on external blockchains.
- typically used by Proof of Stake (PoS) blockchains which provide a high level of security, reducing the need to trust a single entity or centralized authority
- utilising on-chain verification of transactions and messages. This means that the counterparty chain can independently validate the correctness of incoming messages and transactions using its own consensus rules, eliminating the need to trust external sources
- able to upgrade the state of a chain through sending finality proofs, other types of transactions and packets that can be verified
- employs mechanisms to prevent double-spending of assets across blockchains


To implement IBC on a blockchain, three essential components, collectively referred to as the IBC stack are required. First, you require an implementation of the IBC protocol in the programming language that the underlying blockchain uses. Second, a light client is essential and finally, you'll need the support of an IBC relayer.

## IBC Implementations
This refers to the connection and packet level integration of the IBC implementation. It enables the protocol to establish handshake-based connections to securely process and interpret IBC opaque data packets between different blockchains. In production, there are only two implementations of this core IBC functionality. The first implementation is the original `ibc-go`, which is specifically designed for Cosmos SDK chains written in Golang. The second implementation is [`ibc-rs`](https://github.com/ComposableFi/centauri/tree/master/ibc/modules), an implementation of the IBC protocol for Rust based blockchains. It's worth noting that, among all the IBC-enabled chains in production, the majority use `ibc-go`, with the exceptions being Composable Polkadot, Picasso, and Composable Cosmos.

## [Light Clients](./composable-ibc/light-clients.md) 

Light clients serve as a lightweight, trustless mechanism for verifying the state of connected blockchains. They are essential components of the IBC protocol as they facilitate secure and efficient cross-chain interactions without the necessity of fully synchronizing and managing the complete history of every connected blockchain. Composable possesses extensive experience in writing multiple light clients for various blockchains with the intention of enabling communication via the IBC protocol.

Composable's implementation of IBC on Picasso and Composable utilizes the [grandpa light client](https://github.com/ComposableFi/centauri/tree/master/light-clients/ics10-grandpa). The Grandpa protocol is Polkadot and Kusama's consensus mechanism used to finalize blocks on its relay chains. `GRANDPA` enables the verification of finality proofs of Parachain headers. 

The `ICS-8 client` enables light client implementations written in CosmWasm to run natively on blockchains built with the Cosmos SDK. The Grandpa light client is constructed using the `GRANDPA` protocol and written in CosmWasm, therefore, enabling the tracking of finality for Polkadot and Kusama parachains on Cosmos chains through CosmWasm contracts.

## [Hyperspace Relayer](./composable-ibc/hyperspace-relayer.md)

Relayers act as intermediaries responsible for relaying messages, transactions, and state updates across interconnected blockchains within an IBC connection. Hyperspace is a custom-built relayer implementation that allows for transferring arbitrary packets on non-Cosmos blockchains using the IBC protocol. In the future, we anticipate that other relayer solutions will add support for cross-ecosystem message passing through IBC. However, as of now, Hyperspace is the only relayer implementation that has this functionality.