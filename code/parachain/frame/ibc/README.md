### Pallet IBC

Pallet IBC is a thin wrapper around [`ibc-rs`](/code/centauri/ibc) that satisfies the runtime requirements for communicating over the IBC protocol from a substrate runtime.

- [`Config`](/code/parachain/frame/ibc/src/lib.rs#L204)
- [`Call`](/code/parachain/frame/ibc/src/lib.rs#L533)
- [`Pallet`](/code/parachain/frame/ibc/src/lib.rs#L268)

### Dispatchable functions

- `deliver` - Receives a batch of ibc transactions and executes them in the same order as they were sent.
- `transfer` - This initiates an ics20 token transfer from the caller to an account on a connected chain via the ICS20 protocol
- `set_params` - Sets parameters that determine whether token transfer or receipt is allowed in ICS20
- `upgrade_client` - Sets the new consensus state and client state for client upgrades to be executed on connected chains

### Adding Ibc to a substrate runtime

Implementing the ibc config trait for a substrate runtime
```rust
parameter_types! {
	pub const ExpectedBlockTime: u64 = 12000;
	pub const RelayChainId: light_client_common::RelayChain = light_client_common::RelayChain::Rococo;
}

impl pallet_ibc::Config for Runtime {
	type TimeProvider = Timestamp;
	type Event = Event;
	type Currency = Balances;
	const INDEXING_PREFIX: &'static [u8] = b"ibc/";
	const CONNECTION_PREFIX: &'static [u8] = b"ibc/";
	const CHILD_TRIE_KEY: &'static [u8] = b"ibc/";
	const LIGHT_CLIENT_PROTOCOL: pallet_ibc::LightClientProtocol = pallet_ibc::LightClientProtocol::Grandpa; // Finality protocol this chain will be using
	type ExpectedBlockTime = ExpectedBlockTime; // Expected block time in milliseconds
	type MultiCurrency = Assets; // Add a module that implements the Transfer, Mutate and Inspect traits defined in frame_support::traits::fungibles
	type AccountIdConversion = ibc_primitives::IbcAccount;
	type AssetRegistry = AssetsRegistry; // Add a module that implements RemoteAssetRegistryMutate  and RemoteAssetRegistryInspect traits defined in composable_traits::xcm::assets 
	type CurrencyFactory = CurrencyFactory; // Add a module that implements CurrencyFactory trait, defined in composable_traits::currency;
	type WeightInfo = crate::weights::pallet_ibc::WeightInfo<Self>;
	type ParaId = parachain_info::Pallet<Runtime>;
	type RelayChain = RelayChainId;
	type AdminOrigin = EnsureRoot<AccountId>;
}
```

### Terminology

- **ClientState:** This represents a connected chain's light client parameters, required for header verification.

- **ConsensusState:** This represents the timestamp and state root of a connected chain's light client at a particular block height, 
  it is extracted from the block header on successful processing of a ClientUpdate message, its major purpose is for proof verification.

- **PacketCommitment:** The sha256 hash of packet data, sequence,  timeout timestamp and timeout height committed to the runtime storage.

- **PacketReceipt:** An opaque value committed to storage after a message receive packet has been successfully processed.

- **PacketAcknowledgement:** A sha256 hash of packet acknowledgement committed to runtime storage by modules.

- **NextSequenceSend:** A u64 value representing the next packet sequence to be sent on a channel.

- **NextSequenceReceive:** A u64 value representing the next packet sequence to be received on a channel.

- **NextSequenceAck:** A u64 value representing the next acknowledgement sequence to be received on a channel.

### Provable store implementation (ICS23)

The IBC protocol's ICS23 specification requires the host machine to be able to commit values into its storage using a set of standard keys and
subsequently be able to provide verifiable proofs of existence or non-existence for those values given these standard keys, this is called the provable store.  
The protocol requires the host to store specific values using the following keys:

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

For the [`ics23`](/code/parachain/frame/ibc/src/ics23) implementation, 
each member of the provable store is defined as a sub-module.  
A couple methods are implemented for each struct representing a provable store element, each method has a strongly typed interface.
These methods are `insert`, `get` and `remove` in some contexts.  
Notice that we have an `iter` method defined in some provable store implementations, usage of this function should be avoided in on-chain contexts as it increases the POV size of the parachain blocks.
- **Pruning** : Eventually there's going to be an implementation to effectively prune storage of outdated commitments to reduce bloat on our runtime storage.  
  Clients, Connections or channels should not be deleted after they are created.

### Packet and Acknowledgement Storage

In this iteration of the pallet, packets and acknowledgements are stored on chain, but eventually the goal is to store them offchain using the indexing API

### Routing (ICS26) and callback handlers

The IBC protocol requires the existence of a router that routes packets to the correct module for processing based on the destination port.  
The implementation of the router in this pallet statically matches over module id strings and returns the correct handler for such module.  
This means that each ibc application must statically define a unique module id and port id to be used in the module router.

**Plugging a new pallet to ibc**

- Implement the [`Module`](/code/centauri/ibc/modules/src/core/ics26_routing/context.rs#L95) trait for a struct defined in the pallet.
- Implement the [`CallbackWeight`](/code/parachain/frame/ibc/primitives/src/lib.rs#L387) trait for a struct defined in the pallet.
- Define a unique port id and module id as static strings.
- Add the Module handler to the [`ibc router`](/code/parachain/frame/ibc/src/routing.rs#L33).
- Add the callback weight handler to the [`weight router`](/code/parachain/frame/ibc/src/weight.rs#L150).
- Add the module id to the `lookup_module_by_port` implementation.

**Ibc Handler**

This pallet provides a public interface behind the [`IbcHandler`] trait, that allows modules to access the protocol.  
It provides methods for: 
- Opening channels `IbcHandler::open_channel`
- Registering a Send packet `IbcHandler::send_packet`
- Writing Acknowledgements `IbcHandler::write_acknowledgement`

**Defining an example ibc compliant pallet**
```rust
    use ibc_primitives::IbcHandler as IbcHandlerT;
    const PORT_ID: &'static str = "example";
    const MODULE_ID: &'static str = "pallet_example";
    pub trait Config: frame_system::Config {
        IbcHandler: IbcHandlerT;
        WeightInfo: WeightInfo;
    }

    #[pallet::call]
    impl<T: Config> Pallet<T> { 
        #[pallet::weight(0)]
        pub fn initiate_some_ibc_transaction(origin: OriginFor<T>, params: Params) -> DispatchResult {
            ensure_signed(origin)?;
            let send_packet = SendPacketData {
                data: params.data,
                timeout: params.timeout,
                port_id: port_id_from_bytes(PORT_ID.as_bytes().to_vec()).expect("Valid port id expected"),
                channel_id: params .channel_id,
            };
            T::IbcHandler::send_packet(send_packet)
            Ok(())
       }
   }
   
   #[derive(Clone, Eq, PartialEq)]
   pub struct IbcModule<T: Config>(PhantomData<T>);

   impl<T: Config> Default for IbcModule<T> {
        fn default() -> Self {
            Self(PhantomData::default())
        }
   }

   pub struct PalletExampleAcknowledgement(Vec<u8>);

   impl AsRef<[u8]> for PalletExampleAcknowledgement { 
       fn as_ref(&self) -> &[u8] {
           self.0.as_slice()
       }
   }

   impl GenericAcknowledgement for PalletExampleAcknowledgement {}

   impl<T: Config> core::fmt::Debug for IbcModule<T> {
       fn fmt(&self, f: &mut Formatter<'_>) -> core::fmt::Result {
           write!(f, MODULE_ID)
       }
   }

   // All these callbacks should be benchmarked
   impl<T: Config + Send + Sync> Module for IbcModule<T> {
       /// This is called when a channel init message is processed/// If this callback fails the counterparty will not receive the channel_open_try message
       fn on_chan_open_init(
           &mut self,
           _output: &mut ModuleOutputBuilder,
           _order: Order,
           _connection_hops: &[ConnectionId],
           _port_id: &PortId,
           _channel_id: &ChannelId,
           _counterparty: &Counterparty,
           _version: &Version,
       ) -> Result<(), Ics04Error> {
           // Do some stuff
           Ok(())
       }

       /// This is called after a channel_open_try message
       /// has been processed successfully, at this point, this module
       /// should verify that the counterparty's channel order, version and port matches what is expected 
       /// If this callback fails the counterparty will not receive the channel_open_ack message
       fn on_chan_open_try(
           &mut self,
           _output: &mut ModuleOutputBuilder,
           order: Order,
           _connection_hops: &[ConnectionId],
           port_id: &PortId,
           _channel_id: &ChannelId,
           counterparty: &Counterparty,
           version: &Version,
           counterparty_version: &Version,
       ) -> Result<Version, Ics04Error> {
           if counterparty_version.to_string() != *VERSION || version.to_string() != *VERSION { 
               return Err(Ics04Error::no_common_version())
           }

           if order != Order::Ordered {
	           return Err(Ics04Error::unknown_order_type(order.to_string()))
           }

           let example_port = PortId::from_str(PORT_ID).expect("PORT_ID is static and valid; qed");
           if counterparty.port_id() != &example_port || port_id != &ping_port {
	           return Err(Ics04Error::implementation_specific(format!(
		           "Invalid counterparty port {:?}",
		           counterparty.port_id()
	           )))
           }

           Ok(version.clone())
       }

       /// This is called after channel open acknowledgement is processed
       /// Execute any pallet specific logic that requires channel to be open
       /// If this callback fails the counterparty will not receive the channel_open_confirm message
       fn on_chan_open_ack(
           &mut self,
           _output: &mut ModuleOutputBuilder,
           port_id: &PortId,
           channel_id: &ChannelId,
           counterparty_version: &Version,
       ) -> Result<(), Ics04Error> {
           // Do some stuff
           Ok(())
       }

       /// Called after channel open confirm is processed
       fn on_chan_open_confirm(
           &mut self,
           _output: &mut ModuleOutputBuilder,
           port_id: &PortId,
           channel_id: &ChannelId,
       ) -> Result<(), Ics04Error> {
           // Do some stuff
           Ok(())
       }

       /// Called after channel close init messages is processed successfully
       /// If it fails channel close confirm will not be seen on the counterparty
       fn on_chan_close_init(
           &mut self,
           _output: &mut ModuleOutputBuilder,
           port_id: &PortId,
           channel_id: &ChannelId,
       ) -> Result<(), Ics04Error> {
           // Do some stuff
           Ok(())
       }

       /// Called when channel close is successfully processed
       /// Execute pallet specific logic that depends on channel closing
       fn on_chan_close_confirm(
           &mut self,
           _output: &mut ModuleOutputBuilder,
           port_id: &PortId,
           channel_id: &ChannelId,
       ) -> Result<(), Ics04Error> {
           // Do some stuff
           Ok(())
       }

       /// Called after message receive packet is successfully processed
       /// Execute pallet specific logic on packet data and
       /// write error or success Acknowledgement to storage
       fn on_recv_packet(
           &self,
           _output: &mut ModuleOutputBuilder,
           packet: &Packet,
           _relayer: &Signer,
       ) -> OnRecvPacketAck {
           // Do some custom logic and write acknowledgement
           let success = "success".as_bytes().to_vec();
           let data = String::from_utf8(packet.data.clone()).ok();
           let packet = packet.clone();
           OnRecvPacketAck::Successful(
	       Box::new(PalletExampleAcknowledgement(success.clone())),
	           Box::new(move |_| {
		           T::IbcHandler::write_acknowledgement(&packet, success)
			       .map_err(|e| format!("{:?}", e))
	           }),
           )
       }

       /// Called after acknowledgement message is  successfully processed
       /// Decode and handle acknowledgement for both success or error cases  
       fn on_acknowledgement_packet(
           &mut self,
           _output: &mut ModuleOutputBuilder,
           packet: &Packet,
           acknowledgement: &Acknowledgement,
           _relayer: &Signer,
       ) -> Result<(), Ics04Error> {
           // Do some custom logic stuff
           Ok(())
       }

       /// Called on packet timeout message or packet timeout on cose message  
       /// revert changes that were made when packet was sent  
       fn on_timeout_packet(
           &mut self,
           _output: &mut ModuleOutputBuilder,
           packet: &Packet,
           _relayer: &Signer,
       ) -> Result<(), Ics04Error> {
           // Do some stuff
           Ok(())
       }
   }

   pub struct WeightHandler<T: Config>(PhantomData<T>);
   impl<T: Config> Default for WeightHandler<T> {
       fn default() -> Self {
           Self(PhantomData::default())
       }
   }

   impl<T: Config> CallbackWeight for WeightHandler<T> { 
       /// Returns the weight from the benchmark of the `on_chan_open_init` callback
       fn on_chan_open_init(&self) -> Weight {
           T::WeightInfo::on_chan_open_init()
       }
       /// Returns the weight from the benchmark of the `on_chan_open_try` callback
       fn on_chan_open_try(&self) -> Weight {
           T::WeightInfo::on_chan_open_try()
       }

       /// Returns the weight from the benchmark of the `on_chan_open_ack` callback
       fn on_chan_open_ack(&self, port_id: &PortId, channel_id: &ChannelId) -> Weight {
           T::WeightInfo::on_chan_open_ack(port_id, channel_id)
       }
       /// Returns the weight from the benchmark of the `on_chan_open_confirm` callback
       fn on_chan_open_confirm(&self, port_id: &PortId, channel_id: &ChannelId) -> Weight {
           T::WeightInfo::on_chan_open_confirm(port_id, channel_id)
       }
       /// Returns the weight from the benchmark of the `on_chan_close_init` callback
       fn on_chan_close_init(&self, port_id: &PortId, channel_id: &ChannelId) -> Weight {
           T::WeightInfo::on_chan_close_init(port_id, channel_id)
       }
       /// Returns the weight from the benchmark of the `on_chan_close_confirm` callback
       fn on_chan_close_confirm(&self, port_id: &PortId, channel_id: &ChannelId) -> Weight {
           T::WeightInfo::on_chan_close_confirm(port_id, channel_id)
       }  
       /// Returns the weight from the benchmark of the `on_recv_packet` callback  
       /// The weight returned can take the size of the packet data into consideration if necessary  
       fn on_recv_packet(&self, packet: &Packet) -> Weight {
           T::WeightInfo::on_recv_packet(packet)
       }
       /// Returns the weight from the benchmark of the `on_acknowledgement_packet` callback
       /// The weight returned can take the size of the packet data and acknowledgement into consideration if necessary
       fn on_acknowledgement_packet(
		&self,
		packet: &Packet,
		acknowledgement: &Acknowledgement,
       ) -> Weight {
           T::WeightInfo::on_acknowledgement_packet(packet, acknowledgement)
       }
       /// Returns the weight from the benchmark of the `on_timeout_packet` callback
       /// The weight returned can take the size of the packet data into consideration if necessary
       fn on_timeout_packet(&self, packet: &Packet) -> Weight {
           T::WeightInfo::on_timeout_packet(packet)
       }
   }

```

Then add a snippet like this to the `look_up_module_by_port` implementation 
```rust
    pallet_example::PORT_ID => Ok(ModuleId::from_str(pallet_example::MODULE_ID)
				.map_err(|_| ICS05Error::module_not_found(port_id.clone()))?),
```

Add a snippet like this to the `get_route_mut` method in the router implementation and modify the `has_route` method as required  
```rust
    pallet_example::MODULE_ID => Some(&mut self.pallet_example)
```
### Benchmarking implementation

For `transfer`, `set_params` and `upgrade_client` extrinsics we have pretty familiar substrate benchmarks, but for the `deliver` extrinsic
we implement a non-trivial benchmark for different light clients.  
To effectively benchmark the `deliver` extrinsic, we need to individually benchmark the processing of each ibc message type using all available light clients,
this is because different light clients have different header and proof verification algorithms that would execute in the runtime with distinct speeds.

Also, all pallets plugged into ibc are required to benchmark their callbacks and
provide a handler that implements the `CallbackWeight` trait which specifies methods that return the weight of each callback method.

The benchmarking infrastructure for the [`deliver`](/code/parachain/frame/ibc/src/weight.rs#L178) extrinsic defines a weight router that collects a batch of ibc messages, and calculates the total weight of processing the message batch, 
based on the light client needed for proof verification and the specific module callback for handling each message.

#### Writing benchmarks for a light client
The essence of this kind of benchmark is to get an estimate of how much it would cost to verify headers and verify state proofs  
**To benchmark header verification(MsgUpdateClient)**
- Create a valid MsgCreateClient and submit it in the benchmark preparatory code
- Construct a valid light client header
- Construct a MsgUpdateClient from the header and submit it in the actual benchmark code
- Verify that the benchmark was successful by checking if the UpdateClient event was emitted
Note: the benchmark should be dependent on the number of signatures to be verified in the header
The pseudocode below describes roughly how the benchmark should look like
```rust
   update_client {
        let i in 0..5;
        let ctx = crate::routing::Context::default();
        let (client_state, consensus_state) = create_initial_client_state_and_consensus_state(); 
        let msg = MsgCreateClient {
            client_state,
            consensus_state,
            signer: Default::default()
        };
        let msg = Any {
            type_url: msg.type_url().to_string(),
            value: msg.encode_vec()
        };
        ibc::core::ics26_routing::handler::deliver(&mut ctx, msg).unwrap();
        let client_id = get_client_id(); // Get the of the newly created client
        let client_message = create_client_update_message(i); // where i is the number of signatures to be verified in the created header
        let msg = {
            client_id,
            client_message,
            signer: Default::default()
        };

        let msg = Any {
            type_url: msg.type_url().to_string(),
            value: msg.encode_vec()
        };

    }: deliver(RawOrigin::Signed(caller), vec![msg])
        // Assert that UpdateClientEvent was deposited
    }
```
**To benchmark an IBC message type with a light client**
- Create the light client and add it to storage using the context APIs
- Add the necessary values to storage that are required for the benchmark to pass
- Create a mock state tree for the counterparty chain and commit the values needed to prove the message type that is being benchmarked
  - In case of tendermint client the mock state tree will be an AVL tree, for Grandpa client the mock state tree will be a patricia merkle tree etc.
- Extract the root for this tree
- Store a consensus state for the light client created above with this extracted root as the commitment root
- Construct the ibc message with a proof extracted from the mock state tree
- Assert that the message was processed successfully  
Pseudocode demonstrating this process
The following sample is meant to benchmark the channel open ack message type
```rust
   channel_open_ack {
        let mut ctx = routing::Context::<T>::new();
        let now: <T as pallet_timestamp::Config>::Moment = TIMESTAMP.saturating_mul(1000);
        pallet_timestamp::Pallet::<T>::set_timestamp(now);
        // Create initial client state and consensus state
        let (mock_client_state, mock_cs_state) = create_initial_client_state_and_consensus_state();
        let client_id = ClientId::new(&mock_client_state.client_type(), 0).unwrap();
        let counterparty_client_id = ClientId::new("11-beefy", 1).unwrap();
        ctx.store_client_type(client_id.clone(), mock_client_state.client_type()).unwrap();
        ctx.store_client_state(client_id.clone(), mock_client_state).unwrap();
        ctx.store_consensus_state(client_id.clone(), Height::new(0, 1), mock_cs_state).unwrap();

        // Successful processing of channel open ack requires an open connection and a channel end to exist with a state of INIT
        let (connection_id, connection_end) = get_open_connection();
        let (port_id, channel_id, channel_end) = get_channel_end_with_init_state(connection_id);
        ctx.store_connection(connection_id, connection_end);
        ctx.store_channel((port_id, channel_id), channel_end);
        
        // Generate a mock state tree of the counterparty chain
        // Insert a channel end that is in TRYOPEN state using the ibc key path for channels
        // Extract the root and proof for the channel
        // Update the light client consensus state so it can have the required state root required to process
        // the proof that will be submitted
        let (counterparty_channel_id, proof, root): (ChannelId, Vec<u8>, Vec<u8>) = create_and_insert_values_in_mock_state_tree();
        let cs_state = construct_consensus_state_from_root(root);
        ctx.store_consensus_state(client_id, Height::new(0,2), cs_state);
        let msg = MsgChannelOpenAck {
            port_id,
            channel_id,
            counterparty_channel_id,
            counterparty_version: ChannelVersion::new(pallet_example::VERSION.to_string()),
            proofs: Proofs::new(proof.try_into().unwrap(), None, None, None, Height::new(0, 2)).unwrap(),
            signer: Default::default() // Use a valid value here,
        };

        let msg = Any {
            type_url: msg.type_url().to_string(),
            value: msg.encode_vec()
        };
    }: deliver(RawOrigin::Signed(caller), vec![msg])
        // Assert that channel state is now open
    }
```

### ICS20 implementation

The IBC protocol defines an inter-chain token transfer standard that specifies how token transfers should be executed across connected chains.  
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

```rust

impl ibc_runtime_api::IbcRuntimeApi<Block> for Runtime {
    fn para_id() -> u32 {
        <Runtime as cumulus_pallet_parachain_system::Config>::SelfParaId::get().into()
    }

    fn child_trie_key() -> Vec<u8> {
        <Runtime as pallet_ibc::Config>::CHILD_TRIE_KEY.to_vec()
    }

    fn query_send_packet_info(channel_id: Vec<u8>, port_id: Vec<u8>, seqs: Vec<u64>) -> Option<Vec<ibc_primitives::PacketInfo>> {
        Ibc::get_send_packet_info(channel_id, port_id, seqs).ok()
    }

    fn client_consensus_state(client_id: Vec<u8>, revision_number: u64, revision_height: u64, latest_cs: bool) -> Option<ibc_primitives::QueryConsensusStateResponse> {
        Ibc::consensus_state(client_id, revision_number, revision_height, latest_cs).ok()
    }

    // Implement remaining methods using the ibc identical functions in the pallet implementation
  

    fn block_events(extrinsic_index: Option<u32>) -> Vec<pallet_ibc::events::IbcEvent> {
        let mut raw_events = frame_system::Pallet::<Self>::read_events_no_consensus().into_iter();
        if let Some(idx) = extrinsic_index {
            raw_events.find_map(|e| {
                let frame_system::EventRecord{ event, phase, ..} = *e;
                match (event, phase) {
                    (Event::Ibc(pallet_ibc::Event::Events{ events }), frame_system::Phase::ApplyExtrinsic(index)) if index == idx => Some(events),
                     _ => None
                }
            }).unwrap_or_default()
        }
        else { 
            raw_events.filter_map(|e| {
                let frame_system::EventRecord{ event, ..} = *e;

                match event {
                    Event::Ibc(pallet_ibc::Event::Events{ events }) => {
                        Some(events)
                    },
                    _ => None
                }
            }).flatten().collect()
        }
    }
}
```

### IBC Protocol coverage

- [x] ICS02 - Light client implementations  
   **Light clients supported**
  - [x] ICS07 - Tendermint Light Client
  - [x] ICS10 - Grandpa Light Client
  - [x] ICS11 - Beefy Light Client
  - [x] ICS13 - Near Light Client
  - [ ] Ethereum Light Client
- [x] ICS03 - Connections  
- [x] ICS04 - Channels and Ports  
- [x] ICS023 - Vector commitments  
- [x] ICS026 - Routing and callback handlers  
- [x] ICS020 - Fungible token transfer
- [ ] ICS027 - Interchain accounts
- [ ] ICS028 - Cross chain validation
- [ ] ICS029 - Fee payment
- [ ] ICS030 - Middleware
- [ ] ICS031 - Crosschain queries
- [ ] ICS721 - Non-fungible token transfer

### References

Official IBC specification docs [`ibc-spec`](https://github.com/cosmos/ibc)
