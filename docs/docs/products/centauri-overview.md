# Centauri

_Trustless IBC transfer protocol_

Centauri is an extension of the IBC protocol that will facilitate trustless cross-ecosystem communication among various blockchains, including Polkadot, Kusama, Ethereum, NEAR, and Cosmos.

:::tip Centauri Is Live

Use Centauri now to transfer assets between Polkadot and Kusama. Centauri is in advanced testnet stages to its Cosmos connection while we are actively working on implementing IBC to other blockchains.

:::

Centauri leverages and expands upon the existing Inter-Blockchain Communication Protocol (IBC) beyond Cosmos. The IBC protocol previously allowed for trustless bridging between Cosmos SDK chains; however, we are the first to extend IBC to other ecosystems. 

Similar to any IBC connection between two chains, Centauri supports asset transfers (fungible tokens, non-fungible tokens), generic message passing, cross-chain contract calls, cross-chain fee payments, interchain collateralization and more in a trustless manner. The trustless condition of Centauri is due to the fact it is:

- built upon light clients that communicate with each other and updates the state of a counterparty chain
- able to upgrade the state of a chain through sending finality proofs, other types of transactions and packets that can be verified

To implement IBC outside on chains outside of Cosmos, three essential components are required:

## Pallet IBC
Pallet IBC is referred to as the IBC stack for non-Cosmos chains and is composed of [IBC-rs](https://github.com/ComposableFi/centauri/tree/master/ibc/modules) and a [Tendermint light client (ics07)](https://github.com/ComposableFi/centauri/tree/master/light-clients/ics07-tendermint). IBC-rs is an implementation of IBC in Rust, which allows for IBC messages to be interpreted on Picasso (and other IBC-enabled chains). Together, these two components enable our parachains to process and interpret IBC packets.

## [Light Clients](./centauri/light-clients.md) 

Centauri's implementation of IBC on Picasso and Composable utilizes the [grandpa light client](https://github.com/ComposableFi/centauri/tree/master/light-clients/ics10-grandpa). GRANDPA is a protocol developed by Parity to verify finality proofs of parachain headers. The grandpa light client is based on the GRANDPA protocol and written in CosmWasm.

The ICS-8 client enables light client implementations written in CosmWasm to run natively on blockchains built with the Cosmos SDK. This allows the grandpa light client to track the finality of DotSama parachains on Cosmos chains as a CosmWasm contract.

## [Hyperspace relayer](./centauri/hyperspace-relayer.md)

Hyperspace is a custom-built relayer implementation that allows for transferring arbitrary packets on non-Cosmos blockchains using the IBC protocol.In the future, we anticipate that other relayer solutions will add support for cross-ecosystem message passing through IBC. However, as of now, Hyperspace is the only relayer implementation that has this functionality.
