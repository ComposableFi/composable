# IBC module

[![Apache 2.0 Licensed][license-image]][license-link]

Implementation of the Inter-Blockchain Communication Protocol ([IBC]) in rust.

### Module Structure

**Core** contains the traits and handlers that enable the ibc protocol.  
**Applications** contains sub protocols that are built off the core ibc.  
**Mock** contains implementations of the core ibc protocol for testing purposes.  

### Architecture

The design of this crate is geared towards making it as performant as possible and reducing runtime overhead to the bear minimum.  
This is a major focus because it is expected that this crate will be executed inside blockchain runtimes, which are extremely resource constrained environments  
and performance really matters.  
To achieve these noble goals, this crate is designed to use static dispatch over dynamic dispatch, while the former is less flexible from a development perspective, it helps us meet performance 
requirements for the resource constrained environment that is blockchain runtimes. We also provide a couple procedural macros in the [`ibc-derive`](/code/centauri/ibc/derive) crate, that abstracts away a lot of boilerplate code for the end user.

The framework is mostly defined as a set of traits that need to be implemented to make use of the message handling capabilities.

### Terminology

A couple definitions to help understand the architecture of this framework
- **Reader** - A `Reader` is a trait that defines methods that provide read access to the underlying storage of the host.
- **Keeper** - A `Keeper` trait is one that defines methodsa that provide write access to the underlying storage of the host.
- **Context** - The context is a type that implements all the Reader and Keeper traits that control access to the storage of the underlying host.
- **Handler** - A handler is a function that handles processing of an ibc message type.
- **Event** - A struct that which when emitted signifies successful processing of a message.
- **Router** - A type that helps channel packets to the correct module for handling.

### ICS02 Client Definitions

The client module hosts the trait definitions, messages and handlers for light client implementation.

#### Light client

The Ibc protocol is designed to work on top of light clients, light clients are the foundation on which the protocol is built and frankly.  
A light client in the simplest terms, is a construct that is able to verify the state of a blockchain using information extracted from a block header.  
For the former to be a possibility, the blockchain whose light client is being constructed  is required to have a finality protocol(a finality protocol is a means by which a blockchain expresses that state transitions within a block are safe and have a very low probability of been reverted)  
the light client needs to be continuously updated with a stream of finalized block headers, verifying correctness of the state transitions in the  
headers and extracting information that can be used to verify state proofs.

#### Defining a light client
A light client in this protocol is required to have a Client definition,  Client state, Consensus state, and Client message.

To define a light client, the following traits need to be implemented for distinct structs
- [`ClientDef`](/code/centauri/ibc/modules/src/core/ics02_client/client_def.rs)
  - This trait defines all the methods for header and state verification, it also specifies methods for checking and handling misbehaviours
- [`ClientState`](/code/centauri/ibc/modules/src/core/ics02_client/client_state.rs)
  - This trait defines all the methods for dealing with the client state for a light client
- [`ConsensusState`](/code/centauri/ibc/modules/src/core/ics02_client/client_consensus.rs)
  - This trait defined methods for interacting with the Consensus state
- [`ClientMessage`](/code/centauri/ibc/modules/src/core/ics02_client/client_message.rs)
  - This trait defines methods for downcasting to the type contained in the client message enum variants
  
**The Client Context**

The client context is defined by the [`ClientReader`](/code/centauri/ibc/modules/src/core/ics02_client/context.rs#L24), [`ClientKeeper`](/code/centauri/ibc/modules/src/core/ics02_client/context.rs#L106)
and [`ClientTypes`](/code/centauri/ibc/modules/src/core/ics02_client/context.rs#L92) traits.  
These traits control access to the client state, consensus state and other client specific requirements.

**Handlers**
The client handlers process the different client message types
- Update Client - Handles `MsgUpdateClient`
- Create Client - Handles `MsgCreateClient`
- Upgrade Client - Handles `MsgUpgradeClient`
- Misbehaviours - Handles `MsgSubmitMisbehaviour`

**Events**
The events emitted by the client handlers are
 - `CreateClient`
 - `UpdateClient`
 - `UpgradeClient`
 - `ClientMisbehaviour`


### ICS03 Connection

A Connection is a link between two chains, there should ideally be only one connection between two specific chains.  
Connections are built on top of light clients.
Connections cannot be closed or deleted to prevent replay attacks.

**Connection Context**
The Connection context is defined by the [`ConnectionReader`] and [`ConnectionKeeper`] traits

**Handlers**
The client handlers process the different client message types
- connection open init - Handles `MsgConnectionOpenInit`
- connection open try - Handles `MsgConnectionOpenTry`
- connection open ack - Handles `MsgConnectionOpenAck`
- connection open confirm - Handles `MsgConnectionOpenConfirm`

**Events**
The events emitted by the connection handlers
- `OpenInitConnection`
- `OpenTryConnection`
- `OpenAckConnection`
- `OpenConfirmConnection`

### ICS04 Channel

Channels represent a link between identical deployments of an application on connected chains. Channels are built on top of connections.
**Channel Context**
The channel context is defined by the [`ChannelReader`] and [`ConnectionKeeper`] traits.

**Handlers**
The client handlers process the different client message types
- channel open init - Handles `MsgChannelOpenInit`
- channel open try - Handles `MsgChannelOpenTry`
- channel open ack - Handles `MsgChannelOpenAck`
- channel open confirm - Handles `MsgChannelOpenConfirm`
- channel close init - Handles `MsgChannelCloseInit`
- channel close confirm - Handles `MsgChannelCloseConfirm`
- receive packet - Handles `MsgReceivePacket`
- acknowledge packet - Handles `MsgAcknowledgePacket`
- timeout packet - Handles `MsgTimeoutPacket`
- timeout on close packet - Handles `MsgTimeoutOnClosePacket`


**Events**
The events emitted by the connection handlers
- `OpenInitChannel`
- `OpenTryChannel`
- `OpenAckChannel`
- `OpenConfirmChannel`
- `CloseInitChannel`
- `CloseConfirmChannel`
- `ReceivePacket`
- `SendPacket`
- `AcknowledgePacket`
- `TimeoutPacket`
- `TimeoutOnclosePacket`

### ICS26 Routing

The routing defines the entry point into the framework

**Routing Context**
The `Router` trait defines methods that determine how packets are routed to their destination modules in the host

### Applications

Ibc applications are sub protocols built on top of ibc-core

#### ICS020 Fungible Token transfer


## Divergence from the Interchain Standards (ICS)
This crate diverges from the [ICS specification](https://github.com/cosmos/ibc) in a number of ways. See below for more details.

### Module system: no support for untrusted modules
ICS 24 (Host Requirements) gives the [following requirement](https://github.com/cosmos/ibc/blob/master/spec/core/ics-024-host-requirements/README.md#module-system) about the module system that the host state machine must support:

> The host state machine must support a module system, whereby self-contained, potentially mutually distrusted packages of code can safely execute on the same ledger [...].

**This crate currently does not support mutually distrusted packages**. That is, modules on the host state machine are assumed to be fully trusted. In practice, this means that every module has either been written by the host state machine developers, or fully vetted by them.

### Port system: No object capability system
ICS 5 (Port Allocation) requires the host system to support either object-capability reference or source authentication for modules.

> In the former object-capability case, the IBC handler must have the ability to generate object-capabilities, unique, opaque references which can be passed to a module and will not be duplicable by other modules. [...]
> In the latter source authentication case, the IBC handler must have the ability to securely read the source identifier of the calling module, a unique string for each module in the host state machine, which cannot be altered by the module or faked by another module.

**This crate currently requires neither of the host system**. Since modules are assumed to be trusted, there is no need for this object capability system that protects resources for potentially malicious modules.

For more background on this, see [this issue](https://github.com/informalsystems/ibc-rs/issues/2159).

### Port system: transferring and releasing a port
ICS 5 (Port Allocation) requires the IBC handler to permit [transferring ownership of a port](https://github.com/cosmos/ibc/tree/master/spec/core/ics-005-port-allocation#transferring-ownership-of-a-port) and [releasing a port](https://github.com/cosmos/ibc/tree/master/spec/core/ics-005-port-allocation#releasing-a-port).

We currently support neither.

## License

Copyright © 2021 Informal Systems Inc. and ibc-rs authors.

Licensed under the Apache License, Version 2.0 (the "License"); you may not use the files in this repository except in compliance with the License. You may obtain a copy of the License at

    https://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/ibc.svg
[crate-link]: https://crates.io/crates/ibc
[docs-image]: https://docs.rs/ibc/badge.svg
[docs-link]: https://docs.rs/ibc/

[build-image]: https://github.com/informalsystems/ibc-rs/workflows/Rust/badge.svg
[build-link]: https://github.com/informalsystems/ibc-rs/actions?query=workflow%3ARust
[e2e-image]: https://github.com/informalsystems/ibc-rs/workflows/End%20to%20End%20testing/badge.svg
[e2e-link]: https://github.com/informalsystems/ibc-rs/actions?query=workflow%3A%22End+to+End+testing%22

[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/informalsystems/ibc-rs/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-stable-blue.svg
[rustc-version]: https://img.shields.io/badge/rustc-1.51+-blue.svg

[//]: # (general links)

[ibc-rs]: https://github.com/informalsystems/ibc-rs
[IBC]: https://github.com/cosmos/ibc
