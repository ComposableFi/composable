# Inter-Blockchain Communication Protocol (IBC)

The IBC protocol is structured into two layers: IBC/TAO, which governs packet transmission, authentication, and ordering, and IBC/APP, which specifies application handlers for processing the packets. These handlers may include token transfer handlers (ICS20 standard), NFT handlers (ICS721), and others.

**The transport layer (IBC/TAO) comprises light clients, connections, and channels**. Light clients are a succinct representation of the consensus of a counterparty chain with a verification algorithm, allowing the host chain to monitor the counterparty chain's state cost-effectively. The process initiates with the establishment of a connection built on top of a Client, encapsulating two connection ends formed by a successful handshake between the chains. Channels facilitate information exchange between applications on distinct chains, similarly established through a handshake protocol.

The application layer focuses on the business logic involved in processing packets, which can be highly complex and evolves to meet the demands of protocols and users for new functionalities. The IBC specification outlines handlers for standardized token transfers, NFT transfers, cross-chain queries, and atomic token swaps across chains.

In every communication between two applications, there is a **relayer**, an off-chain permissionless component responsible for constructing packets on one chain and submitting these packets to the counterparty chain. 

## Requirements to implement IBC on a network

To implement IBC on a blockchain, various components are required. This section highlights three essential components. First, an implementation of the IBC protocol in the programming language that the underlying blockchain uses. Second, a light client and finally, you'll need an IBC-Relayer designed to be compatible with the chain.

### IBC Implementations
This refers to the connection and packet level integration of the IBC implementation. It enables the protocol to establish handshake-based connections to securely process and interpret IBC opaque data packets between different blockchains. In production, there are only two implementations of this core IBC functionality. The first implementation is the original `ibc-go`, which is specifically designed for Cosmos SDK chains written in Golang. The second implementation is [`ibc-rs`](https://github.com/ComposableFi/centauri/tree/master/ibc/modules), an implementation of the IBC protocol for Rust based blockchains. It's worth noting that, among all the IBC-enabled chains in production, the majority use `ibc-go`, with the exceptions being Composable Polkadot, Picasso, and Composable Cosmos.

### [Light Clients](./ibc/light-clients.md) 

Light clients serve as a lightweight, trustless mechanism for verifying the state of connected blockchains. They are essential components of the IBC protocol as they facilitate secure and efficient cross-chain interactions without the necessity of fully synchronizing and managing the complete history of every connected blockchain. Composable possesses extensive experience in writing multiple light clients for various blockchains with the intention of enabling communication via the IBC protocol.

Composable's implementation of IBC on Picasso and Composable utilizes the [grandpa light client](https://github.com/ComposableFi/centauri/tree/master/light-clients/ics10-grandpa). The Grandpa protocol is Polkadot and Kusama's consensus mechanism used to finalize blocks on its relay chains. `GRANDPA` enables the verification of finality proofs of Parachain headers. 

The `ICS-8 client` enables light client implementations written in CosmWasm to run natively on blockchains built with the Cosmos SDK. The Grandpa light client is constructed using the `GRANDPA` protocol and written in CosmWasm, therefore, enabling the tracking of finality for Polkadot and Kusama parachains on Cosmos chains through CosmWasm contracts.

### [Hyperspace Relayer](./ibc/hyperspace-relayer.md)

Relayers act as intermediaries responsible for relaying messages, transactions, and state updates across interconnected blockchains within an IBC connection. Hyperspace is a custom-built relayer implementation that allows for transferring arbitrary packets on non-Cosmos blockchains using the IBC protocol. In the future, we anticipate that other relayer solutions will add support for cross-ecosystem message passing through IBC. However, as of now, Hyperspace is the only relayer implementation that has this functionality.