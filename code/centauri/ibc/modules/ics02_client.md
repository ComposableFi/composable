## ICS02_CLIENT

ICS02 defines the light client specification for the protocol.

### Defining a light client

To define a light client, there are a few things that must be available  
- Proto file specifying client state, consensus state, header, misbehaviour and client message
- Compile the proto files to rust
- Define the rust equivalents of the structures in the compiled proto files
- Define conversions between equivalent structs in compiled proto and manually defined
- Implement `Protobuf` trait for these structs
- Implement the `ClientState`, `ConsensusState` and `ClientMessage` for the appropriate structs
- Implement `ClientDef`


**Sample Proto file**
```text
    syntax = "proto3";  

    package ibc.lightclients.test.v1;  

    import "google/protobuf/timestamp.proto";

    message Authority {
        // ed25519 public key of the authority
        bytes public_key = 1;
        // authority weight
        uint64 weight = 2;
    }

    // ClientState
    message ClientState {
        // Latest chain height
        uint32 latest_height = 1;

        // current authority set id
        uint64 current_set_id = 2;

        // Block height when the client was frozen due to a misbehaviour
        optional uint64 frozen_height = 3;

        // Light Client Protocol Revision number
        uint32 revision_number = 4;

        // Current authorities
        repeated Authority current_authorities = 5;
    }


    // ConsensusState.
    message ConsensusState {
        // timestamp that corresponds to the block height in which the ConsensusState
        // was stored.
        google.protobuf.Timestamp timestamp = 1;
        // packet commitment root
        bytes root = 2;
    }

    //  misbehaviour type
    message Misbehaviour {
        // The set_id of the equivocations
        uint64 set_id = 1;
        // SCALE-encoded array of equivocations, ideally each belonging to a distinct authority.
        bytes equivocations = 2;
    }
    
    //  Light client header
    message Header {
        // Consensus proof
        bytes validity_proof = 1;
        // Signatures of authorities
        repeated bytes signatures = 2;
        // Encoded signed header
        bytes signed_header = 3;
    }

    // ClientMessage 
    message ClientMessage {
        oneof message {
            Header header = 1;
            Misbehaviour misbehaviour = 2;
        }
    }
```

**Defining equivalent rust structs**
- The next step is to define equivalent structs in rust for what is specified in the proto files and implement the appropriate traits  
- Define conversions between these structs and the compiled proto equivalents

```rust
    pub struct Authority {
        pub public_key: Ed25519Public,
        pub weight: u64
    }

    pub ClientState {
        pub latest_height: u32,
        pub current_set_id: u32,
        pub frozen_height: Option<u64>,
        pub revision_number: u32,
        pub current_authorities: Vec<Authority>
    }
    
    pub ConsensusState {
        pub timestamp: Timestamp,
        pub root: CommitmentRoot
    }
    
    pub Misbehaviour {
        pub set_id: u64,
        // Decoded equivocation
        pub equivocations: Equivocation
    }
    
    pub Header {
        // Decoded validity proof
        pub validity_proof: ValidityProof,
        // Decoded authority signatures
        pub signatures: Vec<AuthoritySignature>,
        // Decoded signed header
        pub signed_header: SignedHeader
    }
    
    pub enum ClientMessage {
        Header(Header),
        Misbehaviour(Misbehaviour)
    }
    
    impl ClientStateT for ClientState { ... }
    
    impl ConsensusStateT for ConsensusState { ... }
    
    impl ClientMessageT for ClientMessage { ... }
    
    impl TryFrom<RawClientState> for ClientState { ... }
    
    impl TryFrom<RawConsensusState> for ConsensusState { ... }
    
    impl TryFrom<RawHeader> for Header { ... }
    
    impl TryFrom<RawClientMessage> for ClientMessage { ... }
    
    impl From<ClientState> for RawClientState { ... }
    
    impl From<ConsensusState> for RawConsensusState { ... }
    
    impl From<Header> for RawHeader { ... }
    
    impl From<ClientMessage> for RawClientMessage { ... }
    
    // Implement protobuf for the structs 
    
    impl Protobuf<RawClientState> for ClientState {}

    impl Protobuf<RawConsensusState> for ConsensusState {}

    impl Protobuf<RawHeader> for Header {}

    impl Protobuf<RawClientMessage> for ClientMessage {} 
```

**Define the Light Client struct and implement ClientDef**

```rust
    pub struct TestLightClient;
    
    impl ClientDef for LightClient {
        type ClientState = ClientState;
        type ConsensusState = ConsensusState;
        type ClientMessage = ClientMessage;
        
        // Implement all required methods ...
    }
```

### Client Context

The client context is a trait encapsulates all the methods that allow access client and consensus state in the handlers.
To satisfy the client context, the Context object must implement the traits in the example code below
```text
    impl ClientReader for Context { ... }
    
    impl ClientKeeper for Context { ... }  
```

### Messages and Events
When client messages are successfully handled, events are emitted
- `CreateClient` -  A `MsgCreateClient` was handled without any errors and a light client has been created.
- `UpdateClient` - A `MsgUpdateClient` was handled without any errors and the new Client and Consensus states have been extracted and stored 
- `UpgradeClient` - A `MsgUpgradeClient` has been handled without any errors, the  client upgrade proof has been verified correctly and the Client and Consensus states have been updated
- `ClientMisbehaviour` -  A `MsgSubmitMisbehaviour` has been processed and the client has been frozen.

The client events are defined [`here`](/code/centauri/ibc/modules/src/core/ics02_client/events.rs)

### Upgrading a Client

A client upgrade is required when there is a breaking change in a chain's light client protocol.  

To upgrade a client, a client upgrade path for both the client state and consensus state must have been predefined in either the client state or as a constant defined when constructing the light client

The chain undergoing the upgrade should then commit the upgraded client and consensus states to its storage using the expected upgrade paths.  

The `MsgUpgradeClient` can now be submitted with the proof for the upgrade.

### Supporting Multiple Light Client Types

The code below describes how to allow support for multiple light client types using the macros provided in `ibc-derive` crate.

```rust
#[derive(Clone, Debug, PartialEq, Eq, ibc_derive::ClientDef)]
pub enum AnyClient {
	Grandpa(ics10_grandpa::client_def::GrandpaClient),
	Beefy(ics11_beefy::client_def::BeefyClient),
	Tendermint(ics07_tendermint::client_def::TendermintClient),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AnyUpgradeOptions {
	Grandpa(ics10_grandpa::client_state::UpgradeOptions),
	Beefy(ics11_beefy::client_state::UpgradeOptions),
	Tendermint(ics07_tendermint::client_state::UpgradeOptions),
}

#[derive(Clone, Debug, PartialEq, Eq, ibc_derive::ClientState, ibc_derive::Protobuf)]
pub enum AnyClientState {
	#[ibc(proto_url = "GRANDPA_CLIENT_STATE_TYPE_URL")]
	Grandpa(ics10_grandpa::client_state::ClientState<HostFunctionsManager>),
	#[ibc(proto_url = "BEEFY_CLIENT_STATE_TYPE_URL")]
	Beefy(ics11_beefy::client_state::ClientState<HostFunctionsManager>),
	#[ibc(proto_url = "TENDERMINT_CLIENT_STATE_TYPE_URL")]
	Tendermint(ics07_tendermint::client_state::ClientState<HostFunctionsManager>),
}

#[derive(Clone, Debug, PartialEq, Eq, ibc_derive::ConsensusState, ibc_derive::Protobuf)]
pub enum AnyConsensusState {
	#[ibc(proto_url = "GRANDPA_CONSENSUS_STATE_TYPE_URL")]
	Grandpa(ics10_grandpa::consensus_state::ConsensusState),
	#[ibc(proto_url = "BEEFY_CONSENSUS_STATE_TYPE_URL")]
	Beefy(ics11_beefy::consensus_state::ConsensusState),
	#[ibc(proto_url = "TENDERMINT_CONSENSUS_STATE_TYPE_URL")]
	Tendermint(ics07_tendermint::consensus_state::ConsensusState),
}

#[derive(Clone, Debug, ibc_derive::ClientMessage)]
#[allow(clippy::large_enum_variant)]
pub enum AnyClientMessage {
	#[ibc(proto_url = "GRANDPA_CLIENT_MESSAGE_TYPE_URL")]
	Grandpa(ics10_grandpa::client_message::ClientMessage),
	#[ibc(proto_url = "BEEFY_CLIENT_MESSAGE_TYPE_URL")]
	Beefy(ics11_beefy::client_message::ClientMessage),
	#[ibc(proto_url = "TENDERMINT_CLIENT_MESSAGE_TYPE_URL")]
	Tendermint(ics07_tendermint::client_message::ClientMessage),
}

impl Protobuf<Any> for AnyClientMessage {}

impl TryFrom<Any> for AnyClientMessage {
	type Error = ics02_client::error::Error;

	fn try_from(value: Any) -> Result<Self, Self::Error> {
		match value.type_url.as_str() {
			GRANDPA_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Grandpa(
				ics10_grandpa::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			BEEFY_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Beefy(
				ics11_beefy::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			TENDERMINT_CLIENT_MESSAGE_TYPE_URL => Ok(Self::Tendermint(
				ics07_tendermint::client_message::ClientMessage::decode_vec(&value.value)
					.map_err(ics02_client::error::Error::decode_raw_header)?,
			)),
			_ => Err(ics02_client::error::Error::unknown_consensus_state_type(value.type_url)),
		}
	}
}

impl From<AnyClientMessage> for Any {
	fn from(client_msg: AnyClientMessage) -> Self {
		match client_msg {
			AnyClientMessage::Grandpa(msg) => Any {
				type_url: GRANDPA_CLIENT_MESSAGE_TYPE_URL.to_string(),
				value: msg.encode_vec(),
			},
			AnyClientMessage::Beefy(msg) =>
				Any { type_url: BEEFY_CLIENT_MESSAGE_TYPE_URL.to_string(), value: msg.encode_vec() },
			AnyClientMessage::Tendermint(msg) => Any {
				type_url: TENDERMINT_CLIENT_MESSAGE_TYPE_URL.to_string(),
				value: msg.encode_vec(),
			}
		}
	}
}

// Then we can go ahead and use them like this

impl<T: Config> ClientTypes for Context<T> {
    type AnyClientMessage = AnyClientMessage;
    type AnyClientState = AnyClientState;
    type AnyConsensusState = AnyConsensusState;
    type ClientDef = AnyClient;
}

```

### Host Consensus state verification

It is a requirement of the ibc protocol for the host machine to verify its own consensus state during the connection handshake.  
This becomes an issue when the host machine cannot access its own consensus state.  
For consensus verification to be possible in such host machine, a couple apis must be available
- The host must provide access to a mapping of block numbers to block hash for at least the 256 most recent blocks.
- The Consensus Proof should be encoded such that apart from the proof, it contains the block header that was used to generate the consensus state being verified, alongside the timestamp with a proof.

With these criteria met it becomes trivial for the host machine to verify its own consensus state.  
The way that is done involves, getting the hash of the header decoded from the proof and verifying that the host has such a blockhash stored in its map of block numbers to block hashes.  
Also the timestamp provided  with its proof should be verified with information present in the block header.  
After both checks are done, the host can freely reconstruct the Consensus state from the block header and return it as the result of the function call.  
The following pseudocode describes how this could be achieved

```rust
    impl ClientReader for Context {

        fn host_consensus_state(
		    &self,
		    height: Height,
		    consensus_proof: Option<Vec<u8>>,
	    ) -> Result<AnyConsensusState, ICS02Error> {
		    let consensus_proof = consensus_proof.ok_or_else(|| {
			    ICS02Error::implementation_specific(format!("No host proof supplied"))
		     })?;
		
		    let block_number = u32::try_from(height.revision_height).map_err(|_| {
			    ICS02Error::implementation_specific(format!(
				    "[host_consensus_state]: Can't fit height: {} in u32",
				    height
			    ))
		    })?;
		    let header_hash = some_host_function_to_get_block_hash(block_number); 
		    // we don't even have the hash for this height (anymore?)
		    if header_hash == Default::default() {
			    Err(ICS02Error::implementation_specific(format!(
				    "[host_consensus_state]: Unknown height {}",
				    height
			    )))?
		    }

		    let consensus_proof: HostConsensusProof = decode_proof(consensus_proof)?;
		    let header = decode_header(consensus_proof.header)?;
		    if hash(header) != header_hash {
			    Err(ICS02Error::implementation_specific(format!(
				    "[host_consensus_state]: Incorrect host consensus state for height {}",
				    height
			    )))?
		    }
		    let timestamp = verify_timestamp(consensus_proof.timestamp, consensus_proof.timestamp_proof, consensus_proof.header)?;

		    // now this header can be trusted
		    let consensus_state = consnensus_state_from_header(consensus_proof.header, timestamp)?;
		    Ok(consensus_state)
	    }
	
    }
```
