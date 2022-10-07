### Pallet IBC

Pallet IBC is a wrapper around [`ibc-rs`](/code/centauri/ibc) that provides the required runtime features for communicating over the IBC protocol in from substrate runtime.

- [`Config`](/code/parachain/frame/ibc/src/lib.rs#L204)
- [`Call`](/code/parachain/frame/ibc/src/lib.rs#L533)
- [`Pallet`](/code/parachain/frame/ibc/src/lib.rs#L268)

### Dispatchable functions

- `deliver` - Receives a batch of ibc messages and processes them
- `transfer` - This initiates an ics20 token transfer from the caller to an account on a connected chain via the ICS20 application
- `set_params` - Sets params that determine whether token transfer or receipt is allowed in ICS20
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

The IBC protocol's ICS23 specification requires the host machine to be able to commit values into its storage using a set of standard key paths and 
subsequently be able to provide verifiable proofs of existence or non-existence for those values given these standard key paths, this is called the provable store
The protocol requires the host to store specific values using the following key paths

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
- Optimizes this pallet by removing the need to have an expensive calculation that would have been incurred if we had to 
  generate a commitment root for the ibc state in this pallet's on_finalize hook

**ICS23 Implementation** 
For the [`ics23`](/code/parachain/frame/ibc/src/ics23) implementation we have a couple methods implemented for each struct representing a provable store element, each method has a strongly typed interface as opposed to raw bytes interface
These methods are `insert`, `get` and `remove` in some contexts;
Notice that we have `iter` method that exists in some implementations, but it's use should be avoided in on-chain contexts as it increases the POV size of the parachain blocks.
- **Pruning** : Eventually there's going to be an implementation to effectively prune storage of outdated commitments to reduce bloat on our runtime storage.

### Packet and Acknowledgement Storage
In this iteration of the pallet, packets and acknowledgements are stored on chain, but eventually the goal is to store them offchain using the indexing API

### Routing (ICS26) and callback handlers
The IBC protocol requires the existence of a router that routes packets to the correct module for processing based on the destination port.
The implementation of the router in this pallet statically matches over module id strings and returns the correct handler for such module.
This means that each ibc application must statically define a unique module id and port id to be used in the module router.

**Adding a new pallet to ibc**
- Implement the [`Module`](/code/centauri/ibc/modules/src/core/ics26_routing/context.rs#L95) trait for struct defined in the pallet
- Implement the [`CallbackWeight`](/code/parachain/frame/ibc/primitives/src/lib.rs#L387) trait for a struct defined in the pallet.
- Define a unique port id and module id as static strings
- Add the Module handler to the [`ibc router`](/code/parachain/frame/ibc/src/routing.rs#L33)
- Add the callback weight handler to the [`weight router`](/code/parachain/frame/ibc/src/weight.rs#L150)

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


### Rpc Interface

### IBC Protocol coverage