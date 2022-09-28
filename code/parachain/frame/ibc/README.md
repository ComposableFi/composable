# IBC

The IBC pallet provides functionality for trustless crosschain messaging (bridging) used for token transfer and 
arbitrary data.

---

# Overview

Pallet IBC is an implementation of the core IBC features for substrate runtimes based of 
[Interchain Standard(ICS)](https://github.com/cosmos/ibc), and serves to connect two separate chains to facilitate 
crosschain user functionality.

## Workflows

We first `deliver` a request to create a client for a connection to the counterparty chain.
This request must be signed and contains the [serialized data structure](https://developers.google.com/protocol-buffers) 
that represents the contents of our message: its routing as the `type_url` and the data `value`.

This is followed by creating the client for the connection on our chain. 
`create_client` doesn't require any consensus proof, but does require `AdminOrigin` to be called.

Once the client is running we can `initiate_connection` by reading the client state and configuring 
`Commitment_Prefix` with the following connection parameters:
* `client_id`
* `counterparty`
* Feature `version`
* `delay period`
* `signer`

The feature version is essential as it describes the set of features needed for the two chains to properly connect.
Once a channel has been opened we can start making transfers. 

Packets are sent in sequences 

## Consensus

When a block is finalized we extract the commitment root comprised of the stateroot and timestamp at the 
current blockheight and verify it on our chain. 

The commitment state describes the actual data of which we are trying 
to verify the existence or non-existence on chain.

The commitment path is the packet containing the keypath to verify the commitment sent by our chain to the 
counterparty chain to verify the commitment packets existence on our chain.


### consensus state

Stateroot and timestamp at blockheight comprises consensus state
needed to verify all ibc transactions.

The Commitment state describes data which we are trying to verify the existence of on chain.
Furthermore, the commitment root as specified by IBC is also expected as the path/key value when inserting the 
consensus state into the keystore.

commitment path is a packet sent by our chain to the counterparty chain with the keypath to verify the commitment 
packet for the exists on our chain

constant string as key values in storage to proof existence or non-existence
the data structure specified by ibc is expected as the path/key value when inserting into the keystore

