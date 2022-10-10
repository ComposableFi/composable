### Pallet IBC

Pallet IBC is a thin wrapper around [`ibc-rs`](/code/centauri/ibc) that satisfies the runtime requirements for communicating over the IBC protocol from a substrate runtime.

- [`Config`](/code/parachain/frame/ibc/src/lib.rs#L204)
- [`Call`](/code/parachain/frame/ibc/src/lib.rs#L533)
- [`Pallet`](/code/parachain/frame/ibc/src/lib.rs#L268)

### Dispatchable functions

- `deliver` - Receives a batch of ibc messages and processes them
- `transfer` - This initiates an ics20 token transfer from the caller to an account on a connected chain via the ICS20 application
- `set_params` - Sets parameters that determine whether token transfer or receipt is allowed in ICS20
- `upgrade_client` - Sets the new consensus state and client state for client upgrades to be executed on connected chains

### Terminology

- **ClientState:** This represents a connected chain's light client parameters, required for header verification.

- **ConsensusState:** This represents the timestamp and state root of a connected chain's light client at a particular block height, 
  it is extracted from the block header on successful processing of a ClientUpdate message, its major purpose is for proof verification.

- **PacketCommitment:** The sha256 hash of packet data, sequence,  timeout timestamp and timeout height committed to the runtime storage

- **PacketReceipt:** An opaque value committed to storage after a message receive packet has been successfully processed

- **PacketAcknowledgement:** A sha256 hash of packet acknowledgement committed to runtime storage by modules

- **NextSequenceSend:** A u64 value representing the next packet sequence to be sent on a channel

- **NextSequenceReceive:** A u64 value representing the next packet sequence to be received on a channel

- **NextSequenceAck:** A u64 value representing the next acknowledgement sequence to be received on a channel

### Provable store implementation (ICS23)

The IBC protocol's ICS23 specification requires the host machine to be able to commit values into its storage using a set of standard keys and 
subsequently be able to provide verifiable proofs of existence or non-existence for those values given these standard key, this is called the provable store
The protocol requires the host to store specific values using the following keys

- `ConsensusState` - `"clients/{identifier}/consensusStates/{height}"`
- `ClientState` - `"clients/{identifier}/clientState"`
- `ConnectionEnd` - `"connections/{identifier}"`
- `ChanelEnd` - `"channelEnds/ports/{identifier}/channels/{identifier}"`
- `PacketCommitment` - `"commitments/ports/{identifier}/channels/{identifier}/sequences/{sequence}"`
- `PacketReceipt` - `"receipts/ports/{identifier}/channels/{identifier}/sequences/{sequence}"`
- `PacketAcknowledgement` - `"acks/ports/{identifier}/channels/{identifier}/sequences/{sequence}"`
- `NextSequenceSend` - `"nextSequenceSend/ports/{identifier}/channels/{identifier}"`
- `NextSequenceReceive` - `"nextSequenceRecv/ports/{identifier}/channels/{identifier}"`
- `NextSequenceAcknowledge` - `"nextSequenceAck/ports/{identifier}/channels/{identifier}"`

The approach we take to implement this is to make use of the [`child trie API`](https://github.com/paritytech/substrate/blob/master/frame/support/src/storage/child.rs), the child trie API affords us a couple benefits, listed below
- The child trie API is a lower level storage API that allows us to insert values into our runtime storage using
  the custom key paths provided by the IBC protocol.
- it to allows the light client on the counterparty chain use the global state root of the host chain to verify state proofs.
- Optimizes this pallet by removing the need to have an expensive computation that would have been incurred if we had to 
  generate a commitment root for the ibc state in this pallet's on_finalize hook

**ICS23 Implementation** 

For the [`ics23`](/code/parachain/frame/ibc/src/ics23) implementation 
Each member of the provable store is defined as a sub-module.
A couple methods are implemented for each struct representing a provable store element, each method has a strongly typed interface.
These methods are `insert`, `get` and `remove` in some contexts.
Notice that we have an `iter` method defined in some of the provable store implementations, usage of this function should be avoided in on-chain contexts as it increases the POV size of the parachain blocks.
- **Pruning** : Eventually there's going to be an implementation to effectively prune storage of outdated commitments to reduce bloat on our runtime storage.
  Clients, Connections or channels should not be deleted after they are created.

### Packet and Acknowledgement Storage

In this iteration of the pallet, packets and acknowledgements are stored on chain, but eventually the goal is to store them offchain using the indexing API

### Routing (ICS26) and callback handlers

The IBC protocol requires the existence of a router that routes packets to the correct module for processing based on the destination port.
The implementation of the router in this pallet statically matches over module id strings and returns the correct handler for such module.
This means that each ibc application must statically define a unique module id and port id to be used in the module router.

**Plugging a new pallet to ibc**

- Implement the [`Module`](/code/centauri/ibc/modules/src/core/ics26_routing/context.rs#L95) trait for struct defined in the pallet
- Implement the [`CallbackWeight`](/code/parachain/frame/ibc/primitives/src/lib.rs#L387) trait for a struct defined in the pallet.
- Define a unique port id and module id as static strings
- Add the Module handler to the [`ibc router`](/code/parachain/frame/ibc/src/routing.rs#L33)
- Add the callback weight handler to the [`weight router`](/code/parachain/frame/ibc/src/weight.rs#L150)
- Add the module id to the `lookup_module_by_port` implementation

**Ibc Handler**

This pallet provides a public interface behind the [`IbcHandler`] trait, that allows modules to access the protocol
It provides methods for
- Opening channels `IbcHandler::open_channel`
- Registering a Send packet `IbcHandler::send_packet`
- Writing Acknowledgements `IbcHandler::write_acknowledgemnent`

### Benchmarking implementation

For `transfer`, `set_params` and `upgrade_client` extrinsics we have pretty familiar substrate benchmarks, but for the `deliver` extrinsic
we implement a non-trivial benchmark for different light clients.
To effectively benchmark the `deliver` extrinsic, we need to individually benchmark the processing of each ibc message type using all available light clients,
this is because different light clients have different header and proof verification algorithms that would execute in the runtime with distinct speeds.

Also, all pallets plugged into ibc are required to benchmark their callbacks and
provide a handler that implements the `CallbackWeight` trait which specifies methods that return the weight of each callback method.

The benchmarking infrastructure for the [`deliver`](/code/parachain/frame/ibc/src/weight.rs#L178) extrinsic defines a weight router that collects a batch of ibc messages, and calculates the total weight of processing the message batch, 
based on the light client needed for proof verification and the specific module callback for handling each message.

### ICS20 implementation

The IBC protocol defines an inter-chain token transfer standard that specifies how token transfers should be executed across connected chains
ICS20 is an ibc application which can be implemented as a standalone pallet nevertheless, it is implemented as a sub-module of the ibc pallet [`here`](/code/parachain/frame/ibc/src/ics20).
The core ics20 logic is already implemented in [`ibc-rs`](/code/centauri/ibc/modules/src/applications/transfer), all that's required to integrate this is to implement the callback handlers for ics20 
and implement the [`Ics20Context`](/code/centauri/ibc/modules/src/applications/transfer/context.rs#l118) trait.

This `Ics20Context` implementation is dependent on [`assets`](/code/parachain/frame/assets) and [`asset-registry`](/code/parachain/frame/assets-registry) pallets for token registration, minting, transfers and burning of tokens,
these can be easily swapped out for any other modules that satisfy the requirements.

### Rpc Interface

The [`Rpc interface`](/code/parachain/frame/ibc/rpc/src/lib.rs) is designed to allow querying the state of the ibc store with membership or non-membership proofs for the result.

- `query_send_packets` - Returns send packets for the provided sequences
- `query_recv_packets` - Returns receive packets for the provided sequences
- `query_client_update_time_and_height` - Returns the time and block height at which a client was updated
- `query_proof` - Returns the proof for the given key, it returns a membership proof if a value exists at that location in storage, otherwise a non-membership proof is returned
- `query_balance_with_address` - Returns the native balance of an address
- `query_client_state` - Returns the state of a client with a membership proof
- `query_client_consensus_state` - Returns the consensus state of a client with a membership proof
- `query_upgraded_client` -  Returns the state of an upgraded client with proof
- `query_upgraded_cons_state` - Returns the consensus state of an upgraded client with proof
- `query_clients` -  Returns the states of all clients on chain
- `query_connection` - Returns the connection end for the provided connection Id with a proof
- `query_connections` - Returns all the connection ends on chain
- `query_connection_using_client` - Returns the connections linked with a particular client
- `query_channel`- Returns the chanel end for then provided channel id with a proof
- `query_channel_client` - Returns the client linked to the provided channel id
- `query_connection_channels` -  Returns all channels linked to the provided connection id
- `query_channels` - Returns all channels on chain
- `query_packet_commitments` - Returns all packet commitments for a channel and port combination
- `query_packet_acknowledgements` - Returns all packet acknowledgements for a channel and port combination
- `query_unreceived_packets` - Filters out the sequences for packets that have not been received from a provided list of sequences
- `query_unreceived_acknowledgements` - Filters out the sequences for acknowledgements that have not been received from a provided list of sequences
- `query_next_seq_recv` - Returns the next sequence to be received on a channel with a proof
- `query_packet_commitment` - Returns a packet commitment with a proof
- `query_packet_acknowledgement` - Returns a packet acknowledgement commitment with a proof
- `query_packet_receipt` - Returns a packet receipt with either a membership or a non-membership proof.
- `query_denom_trace` - Query the ibc denom trace for the provided local asset id
- `query_denom_traces` - Query all ibc denom traces that exist on chain
- `query_events` - Returns all ibc events from a block.

#### Runtime API

A set of runtime apis are specified to enable the rpc interface, these are defined here and should be implemented for the runtime for the rpc interface to work correctly.
The runtime interface is defined [`here`](/code/parachain/frame/ibc/runtime-api/src/lib.rs).
Identical methods are implemented for the pallet to be called in the runtime interface implementation [`here`](/code/parachain/frame/ibc/src/impls.rs#L112)

### IBC Protocol coverage

[x] ICS02 - Light client implementations  
[x] ICS03 - Connections  
[x] ICS04 - Channels and Ports  
[x] ICS23 - Vector commitments  
[x] ICS26 - Routing and callback handlers  
[x] ICS20 - Interchain token transfer  

### References

Official IBC specification docs [`ibc-spec`](https://github.com/cosmos/ibc)