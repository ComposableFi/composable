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

