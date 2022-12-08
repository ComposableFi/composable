```
Composable Finance
Hussein Ait Lahcen, Abdullah Eryuzlu, Karel L. Kubat, Antonio M. Larriba
2022-11-04
```

# Abstract
Smart contracts provide the capability for third parties to run deterministic, reproducible scripts on blockchains, triggered by extrinsics events such as transactions.
The most prevalent virtual machine for executing contracts is the Ethereum Virtual Machine (EVM), but many others exist.
Here we specify the inner workings of our CosmWasm Virtual Machine, an actor-based VM for web assembly contracts.

# Status of This Memo
This specification describes an already implemented virtual machine. New features are not accepted in this document but require a separate RFC.
Discrepancies between the specification and the implementation will be resolved, however.
It has been authored and approved by the core Composable team.

# Copyright Notice
Copyright (c) 2022 Composable Finance and the persons identified as the
document authors.  All rights reserved.

# Table of contents

- [Abstract](#abstract)
- [Status of This Memo](#status-of-this-memo)
- [Copyright Notice](#copyright-notice)
- [Table of Contents](#table-of-contents)
- [1. Overview](#1-overview)
  - [1.1. Document Structure](#11-document-structure)
  - [1.2. Terms and Definitions](#12-terms-and-definitions)
  - [1.3.  Notational Conventions](#13-notational-conventions)
- [2. Smart Contracts](#2-smart-contracts)
    - [2.1. Exports](#21-exports)
        - [2.1.1. Base exports](#211-base-exports)
        - [2.1.2. IBC exports](#212-ibc-exports)
    - [2.2. Interface Version](#22-interface-version)
    - [2.3. Allocating](#23-allocating)
    - [2.4. Deallocating](#24-deallocating)
    - [2.5. Instantiation](#25-instantiation)
        - [2.5.1. Environment Pointer](#251-environment-pointer)
        - [2.5.2. Information Pointer](#252-information-pointer)
        - [2.5.3. Regions](#253-regions)
    - [2.6. Execute](#26-execute)
    - [2.7. Query](#27-query)
    - [2.8. Migrate](#28-migrate)
    - [2.9. Reply](#29-reply)
    - [2.10. Sudo](#210-sudo)
    - [2.11. IBC Channel Open](#211-ibc-channel-open)
    - [2.12. IBC Channel Connect](#212-ibc-channel-connect)
    - [2.13. IBC Channel Close](#213-ibc-channel-close)
    - [2.14. IBC Packet Receive](#214-ibc-packet-receive)
    - [2.15. IBC Packet Ack](#215-ibc-packet-ack)
    - [2.16. IBC Packet Timeout](#216-ibc-packet-timeout)
- [3. Host Functions](#3-host-functions)
    - [3.1. DB Read](#31-db-read)
    - [3.2. DB Write](#32-db-write)
    - [3.3. DB Remove](#33-db-remove)
    - [3.4. DB Scan](#34-db-scan)
    - [3.5. DB Next](#35-db-next)
    - [3.6. Address Validate](#36-address-validate)
    - [3.7. Address Canonicalize](#37-address-canonicalize)
    - [3.8. Address Humanize](#38-address-humanize)
    - [3.9. Secp256k1 Verify](#39-secp256k1-verify)
    - [3.10. Secp256k1 Recovery](#310-secp256k1-recovery)
    - [3.11. Ed25519 Verify](#311-ed25519-verify)
    - [3.12. Ed25519 Batch Verify](#312-ed25519-batch-verify)
    - [3.13. Debug](#313-debug)
    - [3.14. Query Chain](#314-query-chain)
        - [3.14.1 Bank Query](#3141-bank-query)
        - [3.14.2 Wasm Query](#3142-wasm-query)
- [4. Virtual Machine](#4-virtual-machine)
    - [4.1. Gas Metering](#41-gas-metering)
        - [4.1.1. Gas to Weight](#411-gas-to-weight)
    - [4.2. Messaging](#42-messaging)
        - [4.2.1 Submessages](#421-submessages)
        - [4.2.2 Query](#422-query)
    - [4.3. Calling a Contract](#43-calling-a-contract)
    - [4.4. Contract Call Graph](#44-contract-call-graph)
    - [4.5. Custom Message](#45-custom-message)
    - [4.6. Custom Query](#46-custom-query)
- [5. Serialization Format](#5-serialization-format)
- [6. IBC](#6-ibc)
    - [6.1. Enabling IBC](#61-enabling-ibc)
    - [6.2. Host Functions](#62-host-functions)
        - [6.2.1. IBC Transfer](#621-ibc-transfer)
        - [6.2.2. IBC Packet Send](#622-ibc-packet-send)
        - [6.2.3. IBC Channel Close](#623-ibc-channel-close)
- [7. Pallet-CosmWasm](#7-pallet-cosmwasm)
    - [7.1. Gas Metering](#71-gas-metering)
    - [7.2. Uploading Contracts](#72-uploading-contracts)
        - [7.2.1. Definition](#721-definition)
        - [7.2.2. Execution Flow](#722-execution-flow)
        - [7.2.3. Fee](#723-fee)
    - [7.3. Instantiating a Contract](#73-instantiating-a-contract)
        - [7.3.1 Definition](#731-definition)
        - [7.3.2 Execution Flow](#732-execution-flow)
        - [7.3.3 Fee](#733-fee)
    - [7.4. Executing a Contract](#74-executing-a-contract)
        - [7.4.1. Definition](#741-definition)
        - [7.4.2 Execution Flow](#742-execution-flow)
        - [7.4.3 Fee](#743-fee)
- [8. Security considerations](#8-security-considerations)
- [9. Contributors](#9-contributors)

# 1. Overview
CosmWasm is a web assembly (wasm) based standard for smart contracts.
In this document, we define and provide a specification for our implementation of CosmWasm.

Wasm contracts combine an extraordinary performance with a wide support of languages that can be used as the contract language.
Rust is the best-supported programming language, but assembly script is often used as well.
Wasm contracts MUST expose certain functions and MAY make use of host functions provided by the virtual machine to interact with other contracts and the blockchain itself.
CosmWasm differentiates itself from [ink](https://github.com/paritytech/ink) by being actor based, and, by only communicating through asynchronous message passing.


The lifecycle of smart contracts consists of three phases: uploading, instantiation, and migration.
This specification does not impose guidelines on uploading, but defines the semantics of instantiation and migration,as well as calling contracts.
Instantiation is the process of creating a new contract singleton from the uploaded code and instantiation parameters, deriving a unique identifier to call the contract (the address).
Migrations provide a way to upgrade the smart-contract code of an already instantiated contract.
This functionality is opt-in and can be disabled by contract authors.

As of the moment of writing this document, state rent is not part of the CosmWasm standard or our implementation.

## 1.1. Document Structure
- Section 1 provides an overview of the specification and its notation.
  The rest of the manuscript, is structured as follows.
- Section 2 introduces the concept of smart contracts, the interfaces and functionalities they are REQUIRED to support in order to be compliant.
- Section 3 defines the host functions supported by our specification.
- Section 4 covers the virtualization details of the implementation, and, 
- Section 5 briefly defines the serialization requirements.
- Section 6 depicts the IBC compatibility mechanisms of our implementation.
- Section 7 provides specific details about the pallet implementation of our specification.
- Section 8 covers security considerations that every developer MUST follow when using our specification.
- Section 9 lists the contributors that made this document possible.

## 1.2. Terms and Definitions
The keywords "MUST", "MUST NOT", "REQUIRED", "SHALL", "SHALL NOT", "SHOULD", "SHOULD NOT", "RECOMMENDED", "MAY", and "OPTIONAL" in this document are to be interpreted as described in [RFC 2119](https://www.rfc-editor.org/rfc/rfc2119).

Commonly used terms in this document are described below.

`Extrinsic`: A state-altering operation on a blockchain, such as a transaction.

`Actor Model`: Design pattern used in decentralized systems and CosmWasm. Entities are presented as actors with a private state and communicate through messages.

`Message`: Basic unit of communication between actors. MAY affect the internal state of an actor.

`Query`: Read-only messages to query actors.

`Upload`: First step of a deploy-execute process. Uploads some optimized wasm code, no state nor contract address.

`Instantiate`: Second step of a deploy-execute process. Instantiates a code reference with an initial state. It creates a new address for the contract.

`Execute`: Third and final step of a deploy-execute process. Supports various different unprivileged calls to a previously instantiated contract. 

`Canonical Address`: Stable and unique binary representation of an address that is used internally by the blockchain. 

`Escrow Contract`: Contract that holds native tokens and releases them to a pre-defined beneficiary after some conditions are met. 

`Weight`: Substrate unit to measure computational cost of execution.

`Pallet`: Rust module that can be added to a Substrate-based blockchain runtime execution.

`Sudo`: Privileged execution called by privileged Cosmos modules. Similar to Substrate root origin.


## 1.3.  Notational Conventions
This document makes ample usage of Rust to describe web assembly interfaces. Refer to the Rust [book](https://doc.rust-lang.org/book) for an in depth explanation of the syntax.

# 2. Smart Contracts
Smart contracts are web assembly binaries that MUST expose the following mandatory functions.

## 2.1. Exports
Contracts MUST export certain functions and MAY export more.
The tables below provides an overview of each function. Later sections elaborate on each function individually.

### 2.1.1. Base exports

| export              | required | description                           |
|---------------------|----------|---------------------------------------|
| interface_version_8 | Yes      | Signals compliance with v1.0          |
| allocate            | Yes      | Allocates memory for messages         |
| deallocate          | Yes      | Removes previous allocations          |
| instantiate         | Yes      | The first call made to a contract     |
| execute             | Yes      | State altering calls                  |
| query               | Yes      | Read-only calls                       |
| migrate             | No       | Code or storage upgrades              |
| reply               | No       | Handle execution results              |

### 2.1.2. IBC exports

Contracts MUST export **all** the functions below to be considered **IBC capable**.
Section [6.] elaborates more on IBC integration within the VM.

| export              |  description                           |
|---------------------|----------------------------------------|
| ibc_channel_open    |  Callback on channel open              |
| ibc_channel_connect |  Callback on channel connect           |
| ibc_channel_close   |  Callback on channel close             |
| ibc_packet_receive  |  Callback on packet receive            |
| ibc_packet_ack      |  Callback on packet acknowledgment     |
| ibc_packet_timeout  |  Callback on packet timeout            |

## 2.2. Interface Version
The interface version is used by contract authors to signal to the virtual machine which version is supported.
If `interface_version_8` is present, the contract MUST support all other functionality in `1.0`.
The `interface_version_8` export MUST be present in the contract binary.

 ```rust
 extern "C" fn interface_version_8() -> () {}
 ```

If the virtual machine detects unsupported versions, instantiation and contract calls will fail.
Implementors MAY choose to verify the interface version during upload to avoid needless storage.

## 2.3. Allocating
The virtual machine will request the contract to allocate memory for messages and host-to-guest function calls.
The `allocate` export MUST be present in the contract binary.

```rust
extern "C" fn allocate(size: usize) -> u32;
```

The virtual machine will request an allocation by passing the required size.
The contract MUST pass a pointer to memory of at least the requested size.

## 2.4. Deallocating
Contracts MAY deallocate memory when instructed to do so by the virtual machine.
The contract is not required to wipe previously allocated memory.
It is only marked, by the virtual machine, as no longer in use.
The `deallocate` export MUST be present in the contract binary.

```rust
extern "C" fn deallocate(pointer: u32);
```

Compiling contracts with alternative allocators and never deallocating can yield significant performance improvements and reductions in contract size.

## 2.5. Instantiation
After uploading a contract, the virtual machine will instantiate it by passing the instantiation message.
Instantiate export MUST be present in the contract binary.

```rust
extern "C" fn instantiate(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;
```

The virtual machine MUST call `instantiate` before allowing `execute` and `query` access.
Implementors and contract authors MAY choose to lazily execute `instantiate` upon the first `execute` or `query` call.
Our virtual machine eagerly executes it.

The environment pointer (`env_ptr`) contains the current state information.
The message information pointer (info_ptr) contains the sender (public key) along with the tokens that have been sent for the contract call.
See sections [2.5.1.] and [2.5.2.] for the exact definitions of these pointers.

The message pointer (`msg_ptr`) is an arbitrary message (specific to the contract) containing additional information for the contract to execute the desired operation.
By convention, the data is JSON encoded.

### 2.5.1. Environment Pointer
Contracts need to be aware of their environment to properly execute its logic.
The [environment pointer](https://github.com/ComposableFi/cosmwasm-vm/blob/5a874f21cac750cba3dfa0888a72e504515f7e83/std/src/lib.rs#L856) provides a set of indirect pointers that provide information about the environment in which a message was executed.
The JSON-encoded `Env` struct defines the fields that allow contracts to be aware of the current state of the chain and themselves.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct Env {
    pub block: BlockInfo,
    pub transaction: Option<TransactionInfo>,
    pub contract: ContractInfo,
}
```

`block`, `transaction`, and `contract` are information pointers (See section [2.5.2.]) that contain references to the block, transaction and contract, respectively, from which the message was initiated.
`transaction` field MAY be unset when `MsgExecuteContract`/`MsgInstantiateContract`/`MsgMigrateContract` is not executed as part of a transaction.

### 2.5.2. Information Pointer
Different auxiliary `structs` act as pointers to enhance the information for a given message.

#### 2.5.2.1. Block Info
Block Info contains information on the block a message was initiated.
```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct BlockInfo {
    pub height: u64,
    pub time: Timestamp,
    pub chain_id: String,
}
```

`height` represents the number of blocks preceding the current block in the blockchain.
`time` contains the time of block creation in seconds (with nanoseconds precision) since the UNIX epoch (00:00:00 on 1970-01-01 UTC).
The source of time is the [BFT Time in Tendermint](https://github.com/tendermint/tendermint/blob/58dc1726/spec/consensus/bft-time.md).
The field `chain_id` identifies the blockchain to which the block belongs.

#### 2.5.2.2. Transaction Info
Transaction Info contains information about the transaction in which the message was executed.
```rust
pub struct TransactionInfo {
    pub index: u32,
}
```

`index` represents the position of the transaction in the block. The first transaction has index 0.
A unique transaction identifier, in a given chain, can be obtained by using the tuple `(env.block.height, env.transaction.index)`.

#### 2.5.2.3. Contract Info
Contract Info contains information about the smart contract the message interacts with.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct ContractInfo {
    pub address: Addr,
}
```

Where `address` references the smart contract address.

#### 2.5.2.4. Message Info
Message Info contains information on the message itself.
It provides additional information to the `MsgInstantiateContract` and `MsgExecuteContract` messages.
It also provides essential information for authorization: identity of the call, and payment.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct MessageInfo {
    pub sender: Addr,
    pub funds: Vec<Coin>,
}
```

`sender` contains the address that initiated the action. Note that `sender` is the contract address which created the submessage, at the time of its execution.
`funds` represents a list of funds that are sent to the contract prior to the execution. Please note that transfers are processed by bank before the execution so that new balances are visible during the execution.

### 2.5.3. Regions
Pointers returned in many functions do not point directly to their messages, as the size of the message is not known ahead of time.
Instead, a double indirection exists, where the pointer points to a `Region`.
Regions are readable and writable sections of wasm memory.

```rust
#[repr(C)]
pub struct Region {
    pub offset: u32,
    pub capacity: u32,
    pub length: u32,
}
```

`Regions` are decoded using their C representation, not by JSON deserialization.
They are safe to return over FFI boundaries.

A `Region`'s `offset` is the number of bytes from the start of the wasm linear memory.
The `capacity` is the maximum number of bytes that the virtual machine is allowed to write to that region, used for operations that write to memory.
The `length` is the number of bytes that MAY be read from the `offset`, prepared by the smart contract.
Implementors MUST never read past `length` bytes.

## 2.6. Execute

After instantiation, the contract MAY receive `execute` calls.
The `execute` export MUST be present in the contract binary.

```rust
extern "C" fn execute(env_ptr: u32, info_ptr: u32, msg_ptr: u32) -> u32;
```

The virtual machine MAY use `execute` when a transaction calls a contract, or when the contract is called by another contract or module.

All changes to storage MUST be actuated by the virtual machine during the `execute` call. See sections [2.5.1.] and [2.5.2.] for the exact definitions of the `env_ptr` and `info_ptr` pointers. The `msg_ptr` is defined by the contract itself and by convention, JSON encoded.

## 2.7. Query

After instantiation, the contract MAY receive `query` calls. The `query` export MUST be present in the contract binary. The query call is intended to provide information to public, well-defined contract info (See Section [2.5.2.3.].

```rust
extern "C" fn query(env_ptr: u32, msg_ptr: u32) -> u32;
```

The virtual machine MAY use `query` when a transaction calls a contract, or when the contract is called by another contract or module.

Virtual machine implementors MAY choose to make contract storage directly readable by foreign contracts, although that can be very error-prone due to the ability of contracts to migrate their code and storage.

Contracts MAY call host functions that alter storage during `query` calls. The virtual machine MUST ensure that these calls are non-operational during and after the `query` call.

## 2.8. Migrate

After instantiation, the contract MAY receive `migrate` calls. The `migrate` export MAY be present in the contract binary. The administrator of the contract MAY make `migrate` calls to contracts with a missing `migrate` export, which MUST be handled by the virtual machine in an error returned to the administrator. Migrate MUST be called *after* the new code has been swapped.

Contracts without a `migrate` export MUST never be altered by users or administrators. Contracts MAY be migrated to different code if metering needs to be updated, which will not be observable from the contract or users.

## 2.9. Reply

Contracts MAY want to catch the result of a submessage execution.
To enable this optional behaviour, the `reply` export is employed.

```rust
extern "C" fn reply(env_ptr: u32, msg_ptr: u32) -> u32;
```

Where `env_ptr` is a pointer that references the `Env`, and `msg_ptr` is pointer to the `Reply` struct.
```rust
pub struct Reply {
	pub id: u64,
	pub result: SubMsgResult
}
```
`Reply` has a corresponding `id` to identify replies. `result` field contains the `SubMsgResult` of the executed submessage.
Which contains either a `SubMsgResponse` if the  submessage's execution succeeds, or an error `Err` which contains string representation of the error.
```rust
pub enum SubMsgResult {
	Ok(SubMsgResponse),
	Err(String),
}

pub struct SubMsgResponse {
	pub events: Vec<Event>,
	pub data: Option<Binary>,
}
```
Where `events` contains a list of `Events`, and, `data` contains the binary representation of the response.

## 2.10. Sudo

Contracts MAY choose to expose a special entry point, which MAY only be called from privileged modules. This is the CosmWasm equivalent of the root origin in Substrate.

```rust
extern "C" fn sudo(env_ptr: u32, msg_ptr: u32) -> u32;
```

## 2.11. IBC Channel Open

After instantiation, **IBC capable** contracts (see section [2.1.2.](#212-ibc-exports)) MAY receive `ibc_channel_open` calls.
The `ibc_channel_open` export MUST be present in the contract binary.

```rust
extern "C" fn ibc_channel_open(env_ptr: u32, msg_ptr: u32) -> u32;
```

Where `msg_ptr` is a pointer to a region (see section [2.5.3.]) of [IbcChannelOpenMsg](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L252).
```rust
pub enum IbcChannelOpenMsg {
    OpenInit { channel: IbcChannel },
    OpenTry {
        channel: IbcChannel,
        counterparty_version: String,
    },
}
```

Where `OpenInit` represents the channel handshake init, and, `OpenTry` represents the channel handshake try.

The virtual machine MAY use `ibc_channel_open` when an IBC channel [handshake is being initiated](https://github.com/cosmos/ibc/blob/f6371ffd5de3787eb4b85f9fe77f81be4a5993a0/spec/core/ics-004-channel-and-packet-semantics/README.md#channel-lifecycle-management) between the contract (see section [6.]) and a counterparty.

Contracts MAY abort the channel opening process by returning an error from the call or [optionally overwrite the channel version](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L306).

## 2.12. IBC Channel Connect

After having passed the IBC channel handshake initialization (see section [2.11.](#211-ibc-channel-open)), **IBC capable** contracts (see section [2.1.2.](#212-ibc-exports)) MAY receive `ibc_channel_connect` calls.
The `ibc_channel_connect` export MUST be present in the contract binary.

```rust
extern "C" fn ibc_channel_open(env_ptr: u32, msg_ptr: u32) -> u32;
```

Where `msg_ptr` is a pointer to a region (see section [2.5.3.]) of [IbcChannelConnectMsg](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L317).
```rust
pub enum IbcChannelConnectMsg {
    OpenAck {
        channel: IbcChannel,
        counterparty_version: String,
    },
    OpenConfirm { channel: IbcChannel },
}
```

Where `OpenAck` represents the channel handshake acknowledgement, and, `OpenConfirm` represents the channel handshake confirmation.

The virtual machine MAY use `ibc_channel_connect` when an IBC channel [handshake is being finalized](https://github.com/cosmos/ibc/blob/f6371ffd5de3787eb4b85f9fe77f81be4a5993a0/spec/core/ics-004-channel-and-packet-semantics/README.md#channel-lifecycle-management) between the contract (see section [6.]) and a counterparty.

Contracts MAY return an error (aborting the channel handshake) or an [IbcBasicResponse](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L486) from the call.

```rust
pub struct IbcBasicResponse<T = Empty> {
    pub messages: Vec<SubMsg<T>>,
    pub attributes: Vec<Attribute>,
    pub events: Vec<Event>,
}
```

Where `messages` are the sub-messages to process.

Where `attributes` are the attributes associated with the default event that will be yielded for the call.

Where `events` are custom events that can be yielded along the default event.

## 2.13. IBC Channel Close

If a channel has been opened to a contract (see section [2.11.](#211-ibc-channel-open)), **IBC capable** contracts (see section [2.1.2.](#212-ibc-exports)) MAY receive `ibc_channel_close` calls.
The `ibc_channel_close` export MUST be present in the contract binary.

```rust
extern "C" fn ibc_channel_close(env_ptr: u32, msg_ptr: u32) -> u32;
```

Where `msg_ptr` is a pointer to a region (see section [2.5.3.]) of [IbcChannelCloseMsg](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L369).
```rust
pub enum IbcChannelCloseMsg {
    CloseInit { channel: IbcChannel },
    CloseConfirm { channel: IbcChannel },
}
```

Where `CloseInit` represents the channel closing initialization, and, `CloseConfirm` represents the channel closing confirmation.

The virtual machine MAY use `ibc_channel_close` when a previously opened IBC channel [is being closed](https://github.com/cosmos/ibc/blob/f6371ffd5de3787eb4b85f9fe77f81be4a5993a0/spec/core/ics-004-channel-and-packet-semantics/README.md#channel-lifecycle-management).

Contracts MAY return an error or an [IbcBasicResponse](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L486) from the call.

```rust
pub struct IbcBasicResponse<T = Empty> {
    pub messages: Vec<SubMsg<T>>,
    pub attributes: Vec<Attribute>,
    pub events: Vec<Event>,
}
```

Where `messages` are the sub-messages to process.

Where `attributes` are the attributes associated with the default event that will be yielded for the call.

Where `events` are custom events that can be yielded along the default event.

## 2.14. IBC Packet Receive

If a channel has been opened to a contract (see section [2.11.](#211-ibc-channel-open)), **IBC capable** contracts (see section [2.1.2.](#212-ibc-exports)) MAY receive `ibc_packet_receive` calls.
The `ibc_packet_receive` export MUST be present in the contract binary.

```rust
extern "C" fn ibc_packet_receive(env_ptr: u32, msg_ptr: u32) -> u32;
```

Where `msg_ptr` is a pointer to a region (see section [2.5.3.]) of [IbcPacketReceiveMsg](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L405).
```rust
pub struct IbcPacketReceiveMsg {
    pub packet: IbcPacket,
    #[cfg(feature = "ibc3")]
    pub relayer: Addr,
}
```

Where `packet` represents the packet sent by the counterparty.

Where `relayer` represents the actor that relayed the packet. Usually under which account the transaction is being executed.

The virtual machine MAY use `ibc_packet_receive` [when a packet is received](https://github.com/cosmos/ibc/blob/f6371ffd5de3787eb4b85f9fe77f81be4a5993a0/spec/core/ics-004-channel-and-packet-semantics/README.md#packet-flow--handling) over a previously opened IBC channel.

Contracts MUST enable the **ibc3** feature to have access to the `relayer` field of `IbcPacketReceiveMsg`.

Contracts MAY return an error or an [IbcReceiveResponse](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L628) from the call.

```rust
pub struct IbcReceiveResponse<T = Empty> {
    pub acknowledgement: Binary,
    pub messages: Vec<SubMsg<T>>,
    pub attributes: Vec<Attribute>,
    pub events: Vec<Event>,
}
```

Where `messages` are the submessages to process.

Where `attributes` are the attributes associated with the default event that will be yielded for the call.

Where `events` are custom events that can be yielded along the default event.

Where `acknowledgement` is the field where the contracts MUST store the acknowledgment (contract specific) for the counterparty that originally sent the packet.

## 2.15. IBC Packet Ack

If a channel has been opened to a contract (see section [2.11.](#211-ibc-channel-open)) and a packet has been sent over the channel (see section [2.14.]), **IBC capable** contracts (see section [2.1.2.](#212-ibc-exports)) MUST receive an `ibc_packet_ack` call.
The `ibc_packet_ack` export MUST be present in the contract binary.

```rust
extern "C" fn ibc_packet_ack(env_ptr: u32, msg_ptr: u32) -> u32;
```

Where `msg_ptr` is a pointer to a region (see section [2.5.3.]) of [IbcPacketAckMsg](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L426).
```rust
pub struct IbcPacketAckMsg {
    pub acknowledgement: IbcAcknowledgement,
    pub original_packet: IbcPacket,
    #[cfg(feature = "ibc3")]
    pub relayer: Addr,
}
```

Where `acknowledgement` is the acknowledgement sent by the counterparty (contract specific).

Where `original_packet` is the packet originally sent by the local contract.

Where `relayer` represent the relayer that relayed the acknowledgement (usually under which account the transaction is being executed).

The virtual machine MUST use `ibc_packet_ack` [when a previously sent packet, over a previously opened channel is acknowledged](https://github.com/cosmos/ibc/blob/f6371ffd5de3787eb4b85f9fe77f81be4a5993a0/spec/core/ics-004-channel-and-packet-semantics/README.md#packet-flow--handling).

Contracts MUST enable the **ibc3** feature to have access to the `relayer` field of `IbcPacketAckMsg`.

Contracts MAY return an error or an [IbcBasicResponse](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L486) from the call.

```rust
pub struct IbcBasicResponse<T = Empty> {
    pub messages: Vec<SubMsg<T>>,
    pub attributes: Vec<Attribute>,
    pub events: Vec<Event>,
}
```

Where `messages` are the sub-messages to process.

Where `attributes` are the attributes associated with the default event that will be yielded for the call.

Where `events` are custom events that can be yielded along the default event.

## 2.16. IBC Packet Timeout

If a channel has been opened to a contract (see section [2.11.](#211-ibc-channel-open)) and a packet has been sent over the channel (see section [2.14.]), **IBC capable** contracts (see section [2.1.2.](#212-ibc-exports)) MAY receive an `ibc_packet_timeout` call.
The `ibc_packet_timeout` export MUST be present in the contract binary.

```rust
extern "C" fn ibc_packet_timeout(env_ptr: u32, msg_ptr: u32) -> u32;
```

Where `msg_ptr` is a pointer to a region (see section [2.5.3.]) of [IbcPacketTimeoutMsg](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L459).
```rust
pub struct IbcPacketTimeoutMsg {
    pub packet: IbcPacket,
    #[cfg(feature = "ibc3")]
    pub relayer: Addr,
}
```

Where `packet` is the packet originally sent by the local contract.

Where `relayer` represent the actor that relayed the timeout. Usually under which account the transaction is being executed.

The virtual machine MUST use `ibc_packet_timeout` [when a previously sent packet, over a previously opened channel could not be processed in time](https://github.com/cosmos/ibc/blob/f6371ffd5de3787eb4b85f9fe77f81be4a5993a0/spec/core/ics-004-channel-and-packet-semantics/README.md#packet-flow--handling).

Contracts MUST enable the **ibc3** feature to have access to the `relayer` field of `IbcPacketTimeoutMsg`.

Contracts MAY return an error or an [IbcBasicResponse](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L486) from the call.

```rust
pub struct IbcBasicResponse<T = Empty> {
    pub messages: Vec<SubMsg<T>>,
    pub attributes: Vec<Attribute>,
    pub events: Vec<Event>,
}
```

Where `messages` are the sub-messages to process.

Where `attributes` are the attributes associated with the default event that will be yielded for the call.

Where `events` are custom events that can be yielded along the default event.

# 3. Host Functions
The virtual machine MUST provide the host functions listed below.
A host function is provided by the virtual machine and callable by contracts to interact with the blockchain itself, very much like syscalls in operating system contexts.
The next sections define each function in more depth.
The `read-only` column determines whether the operation is allowed to alter the storage or not -- a read-only operation altering the storage result in a read-only violation.

| import                   | required | description                                                                |
|--------------------------|----------|----------------------------------------------------------------------------|
| db_read                  | Yes      | Reads from contract storage                                                |
| db_write                 | Yes      | Writes to contract storage                                                 |
| db_remove                | Yes      | Deletes from contract storage                                              |
| db_scan                  | Yes      | Creates an iterator over storage                                           |
| db_next                  | Yes      | Fetches next item from iterator                                            |
| addr_validate            | Yes      | Checks if the string is a public key                                       |
| addr_canonicalize        | Yes      | Converts a human-readable address to canonical                             |
| addr_humanize            | Yes      | Converts a canonical address to human readable                             |
| secp256k1_verify         | Yes      | Verifies message hashes against a signature with a public key              |
| secp256k1_recover_pubkey | Yes      | Recover the public key from a signed message                               |
| ed25519_verify           | Yes      | Verifies a message against a signature with a public key                   |
| ed25519_batch_verify     | Yes      | Verifies a batch of messages against a batch of signatures and public keys |
| debug                    | Yes      | Writes a debug message (UFT-8 encoded) to VM logs                          |
| query_chain              | Yes      | Executes a query on the chain                                              |

## 3.1. DB Read
Reads a data associated with a given `key`.
If an entry with the provided `key` exists in the contract memory, a pointer to the corresponding `Region` MUST be returned.
Otherwise, `0` is returned.

```rust
fn db_read(key: u32) -> u32;
```

## 3.2. DB Write
Writes the `key`, `value` pair to the storage.
Where `value` is a pointer to the `Region` that redirects to the value.

```rust
fn db_write(key: u32, value: u32);
```

## 3.3. DB Remove
Removes the value associated to a given `key` from the storage.
```rust
fn db_remove(key: u32);
```

## 3.4. DB Scan
Storage MAY be iterated over using the `db_scan` function.
An iterator is created and stored by the virtual machine, which can be used in subsequent calls to obtain the next storage item.

```rust
fn db_scan(start_ptr: u32, end_ptr: u32, order: i32) -> u32;
```

`start_ptr` is a pointer to a region of `Option<&[u8]>`, indicating the key at which to start iterating. `end_ptr` is also a pointer to a region of `Option<&[u8]>`, providing the end of the iterator. If both are `None`, Iteration will start at the zero key and end when all keys are visited. `order` is a pointer to a region storing an `Order`, which can be used to determine the order in which keys are visited.

```rust
pub enum Order {
    Ascending,
    Descending,
}
```

`db_scan` returns an iterator handle, which can be used in the `db_next` function to resume iteration.
See section 3.5 on the semantics of this function.

Iterator handles are only valid during the lifetime of calls into a contract.
Contract authors MUST not store handles in permanent storage.


## 3.5. DB Next
After obtaining an iterator handle (section [3.4.]), the handle can be passed to the `db_next` function to obtain the next value, a pointer to a region storing the value.

```rust
fn db_next(iterator_id: u32) -> u32;
```

Contract authors MUST ensure that the iterator handle is obtained through the `db_scan` function during the current execution.

## 3.6. Address Validate
Function `addr_validate` takes as parameter a pointer to a `Region`
The `Region` stores an address in human-readable format and validates if it is valid.
If the validation succeeds, `0` is returned.
Otherwise, the string representation of the error MUST be written to the contract's memory, and the pointer to the corresponding `Region` MUST be returned.

Validation SHOULD make the following checks:

1.  The address is valid, in the sense that it can be converted to a canonical representation by the backend.
2.  The address is normalized, i.e.  `humanize(canonicalize(input)) == input`.


```rust
fn addr_validate(source_ptr: u32) -> u32;
```

`source_ptr` is a pointer to a region containing a UTF-8 encoded `String`.

Implementors MUST export the `addr_validate` function.

Implementors MUST verify that the address can be canonicalized (see section [3.7.]).

Implementors MUST verify that the address is normalized, such that the following equation hold: `addr_humanize(addr_canonicalize(address)) = address` (see section [3.7.] and [3.8.]).

The return pointer point to a region containing a JSON serialized `Result<(), String>`.

Implementors MUST not abort VM execution on validation error, instead, the error MUST be returned to the contract as an encoded UTF-8 `String`.

Contract authors MUST handle the result of the call and MAY abort the transaction if the address is not valid.

## 3.7. Address Canonicalize
Takes a pointer (`source_ptr`) to the `Region` that stores a human-readable address and converts the address into `CanonicalAddr` format.
If there is no error, `CanonicalAddr` MUST be written to a `Region` referenced by `destination_ptr` and `0` MUST be returned.
Otherwise, the string representation of the error MUST be written to the contract's memory and the pointer to the corresponding `Region` MUST be returned.

```rust
fn addr_canonicalize(source_ptr: u32, destination_ptr: u32) -> u32;
```

Contracts MAY request the host for address canonicalization.

A canonical address is a binary representation of a valid address (see section [3.6.]).

```rust
fn addr_canonicalize(source_ptr: u32, destination_ptr: u32) -> u32;
```

Implementors MUST export the `addr_canonicalize` function.

`source_ptr` is a pointer to a region containing a UTF-8 encoded `String`.
`destination_ptr` is a pointer to a region containing a UTF-8 encoded `String`.

Implementors MUST ensure that the `addr_canonicalize` function is the inverse of `addr_humanize` (see section [3.8.]) such that the following equation hold: `addr_canonicalize . addr_humanize = addr_humanize . addr_canonicalize = identity`.

The return pointer point to a region containing a JSON serialized `Result<(), String>`.

Implementors MUST not abort VM execution on canonicalization error, instead, the error MUST be returned to the contract as an encoded UTF-8 `String`.

Contract authors MUST handle the result of the call and MAY abort the transaction if the address could not be canonicalized.

## 3.8. Address Humanize

Contracts MAY request the host for address humanization.

A human address is a human-readable representation of a canonical address (see section [3.7.]).

```rust
fn addr_humanize(source_ptr: u32, destination_ptr: u32) -> u32;
```

Implementors MUST export the `addr_humanize` function.

Implementors MUST ensure that the `addr_humanize` function is the inverse of `addr_canonicalize` (see section [3.7.]) such that the following equations hold: `addr_humanize . addr_canonicalize = addr_canonicalize . addr_humanize = identity`.

The return pointer point to a region containing a JSON serialized `Result<(), String>`.

Implementors MUST not abort VM execution on humanization error, instead, the error MUST be returned to the contract as an encoded UTF-8 `String`.

Contract authors MUST handle the result of the call and MAY abort the transaction if the address could not be humanized.

## 3.9. Secp256k1 Verify
Integrity checks over a message hash and its signature can be done using the `secp256k1_verify` function.
The signature MUST be in Cosmos format:

| Field      | Format                                                                                                            | Size (bytes) |
|------------|-------------------------------------------------------------------------------------------------------------------|--------------|
| signature  | Serialized compressed signature                                                                                   | 64           |
| public key | [Serialized according to SEC 2](https://www.oreilly.com/library/view/programming-bitcoin/9781492031482/ch04.html) | 33 or 65     |

It verifies the digital signature created with the `ECDSA` algorithm and over the `secp256k1`elliptic curve (EC).

```rust
fn secp256k1_verify(message_hash_ptr: u32, signature_ptr: u32, public_key_ptr: u32) -> u32;
```

`message_hash_ptr` is a pointer to a Region (as defined in Section [2.5.3]) that contains the hash of the message to be verified. The hash function used SHOULD be SHA-256.

`signature_ptr` is a pointer to a Region that contains the digital signature. Signature MUST be in compact format: a 64-byte-length binary string that serializes r (32 bytes) and s (32 bytes) in that specific order.

`public_key_ptr` is a pointer to a Region that contains a list of references to the serialized public key against which the signature will be verified. The public key MUST be Serialized according to [SEC 2](https://www.oreilly.com/library/view/programming-bitcoin/9781492031482/ch04.html).

`secp256k1_verify` returns 0 on verification success, 1 on verification failure, and values greater than 1 in case of error.

## 3.10. Secp256k1 Recovery
A public key can be recovered from a message hash and a signature by using the `secp256k1_recover_pubkey` function.

```rust
fn secp256k1_recover_pubkey(message_hash_ptr: u32, signature_ptr: u32, recovery_param: u32, ) -> u64;
```

`message_hash_ptr` is a pointer to a Region (as defined in Section [2.5.3]) that contains the hash of the message to be verified. The hash function used SHOULD be SHA-256.

`signature_ptr` is a pointer to a Region that contains the digital signature. Signature MUST be in compact format: a 64-byte-length binary string that serializes r (32 bytes) and s (32 bytes) in that specific order.

`recovery_param` is an integer representing the recovery id `v` used to recover the EC point `R`. MUST be 0 or 1 to be consistent with [Ethereum restrictions](https://github.com/ethereum/go-ethereum/blob/v1.9.25/internal/ethapi/api.go#L466-L469).

`secp256k1_recover_pubkey` returns a `u64` value.
If the recovery succeeds, a pointer to the recovered public key, in serialized form and ready to be used as input in `secp256k1_verify`, is stored in the lower 32 bytes of the `u64` value.
Otherwise, an error code is stored in the upper 32 bytes of the `u64` value.

## 3.11. Ed25519 Verify
Integrity checks over a message hash and its signature can be done using the `ed25519_verify` function.
The signature MUST be in [Tendermint](https://docs.tendermint.com/v0.32/spec/blockchain/encoding.html#public-key-cryptography) format:

| Field      | Format                 | Size (bytes) |
|------------|------------------------|--------------|
| signature  | Raw ED25519 signature  | 64           |
| public key | Raw ED25519 public key | 32           |

It verifies the digital signature created with the `ed25519` algorithm and over the `curve25519`elliptic curve.

```rust
    fn ed25519_verify(message_ptr: u32, signature_ptr: u32, public_key_ptr: u32) -> u32;
```
`message_ptr` is a pointer to a Region (as defined in Section [2.5.3]) that contains the hash of the message to be verified. The hash function used SHOULD be SHA-512.

`signature_ptr` is a pointer to a Region that contains the digital signature. Signature MUST be in Raw ED25519 format.

`public_key_ptr` is a pointer to a Region that contains a list of references to the serialized public key against which the signature will be verified. The public key MUST be in Raw ED25519 format.

`ed25519_verify` returns 0 on verification success, 1 on verification failure, and values greater than 1 in case of error.

## 3.12. Ed25519 Batch Verify
Multiple signature verification over the `ed25519` algorithm can be optimized and batched together by using the `ed25519_batch_verify` function.

```rust
fn ed25519_batch_verify(messages_ptr: u32, signatures_ptr: u32, public_keys_ptr: u32) -> u32;
```

`messages_ptr` is a pointer to a Region (as defined in Section [2.5.3]) that contains a list of references to the hashes of the messages to be verified. The hash function used SHOULD be SHA-512.

`signatures_ptr` is a pointer to a Region that contains a list of references to the signatures to verify. Signature MUST be in Raw ED25519 format.

`public_keys_ptr` is a pointer to a Region that contains a list of references to the public keys against which the signatures will be verified. Public key MUST be in Raw ED25519 format.

`ed25519_batch_verify` returns 0 on verification success, 1 on verification failure, and values greater than 1 in case of error.

The function dynamically adapts to support three different scenarios:

- Equal number of messages, signatures, and public keys: Standard, generic functionality.
- One message, and an equal number of signatures and public keys: Multiple digital signature (multi-sig) verification of a single message.
- One public key, and an equal number of messages and signatures: Verification of multiple messages, all signed with the same private key.

Any other variants of input vectors result in an Error.
However, three special cases MUST be taken into account:
- The "one-message, with zero signatures and zero public keys" case, is considered the empty case.
- The "one-public key, with zero messages and zero signatures" case, is considered the empty case.
- The empty case (no messages, no signatures and no public keys) returns true.

## 3.13. Debug
Contracts MAY log information for debugging purposes.
Implementors MAY choose to emit these messages in the node logs.
Implementors MUST export the debug function. Implementors MUST interpret the debug information as UTF-8.

```rust
fn debug(source_ptr: u32);
```

`source_ptr` is a pointer to a region containing a UTF-8 encoded `String`.

In production environments, it is expected that the debug logs are discarded. Debug calls MUST not be metered or affect the blockchain's state.

## 3.14. Query Chain
The state of the underlying chain can be queried by contracts using the `query_chain` host function.

```rust
fn query_chain(request: u32) -> u32;
```

The `request` is a pointer to a region containing a JSON serialized `QueryMessage`.

```rust
enum QueryRequest {
    Custom(query),
    Bank(BankQuery),
    Wasm(WasmQuery),
}
```
The custom query can be used by implementors to extend their virtual machine's capabilities.

Implementors MUST ensure that the query is not allowed to alter the storage or any other part of the system.
Defaulting to do so would lead to possible reentrancy attacks.

Implementors MAY choose to support further variants.

### 3.14.1 Bank Query
Bank queries are used to query the native assets of a chain.

```rust
enum BankQuery {
    Balance {
        address: Address,
        denom: AssetId
    },
    AllBalances {
        address: Address,
    }
}
```

The `Balance` variant is used to query the balance of a specific asset, where `address` is the account holding the funds, and `denom` is the chain-specific asset identifier.

`AllBalances` provides the balance of all assets for the given `address`.

### 3.14.2 Wasm Query
Wasm queries can be used by contracts to query other contracts and their associated storage.

```rust
enum WasmQuery {
    Smart {
        contract_addr: Address,
        msg: Vec<u8>,
    },
    Raw {
        contract_addr: Address,
        key: Vec<u8>,
    }
    ContractInfo {
        contract_addr: Address,
    }
}
```

The `Smart` variant loads the contract specified by `contract_addr` into the virtual machine and executes the `query` export with the provided `msg`.

`Raw` queries avoid loading the contract itself, and instead, it directly queries its storage at `key`.
Note that due to updatability, this is more error-prone, but it implies lower gas costs.

Metadata about the contract MAY be queried using the `ContractInfo` variant, which returns the canonical address (See Section [2.5.2.3]).

# 4. Virtual Machine
In this section we cover the main implementation details of our minimalistic and `no_std` friendly abstract virtual machine for CosmWasm contract execution. 

## 4.1. Gas Metering
Gas metering is the process of instrumentalizing the code from the wasm module to obtain a value describing the cost of executing that code.
The instrumentalization includes both gas metering calls and stack limit checks.
These checks ensure the call stack size remains fixed across different execution engines, as some of them MAY support different complexities and this might lead to inconsistencies.

Implementation details are left open to the developers. Gas metering MUST be deterministic and reproducible.

### 4.1.1. Gas to Weight
Substrate based chains use weight as unit to charge computational execution.
One gas is equivalent to one weight, which is defined as 1<sup>-12</sup> seconds of execution time on a reference machine.

## 4.2. Messaging
CosmWasm is based in the actor model design pattern.
In this model, distributed entities are represented as independent actors.
Actors have their own and private internal state that cannot be altered by external actors.
All execution and logic is routed through messaging.
Actor is an abstract term, but in CosmWasm usually refers to a single instance of a smart contract.

Messages are the basic unit of communication in CosmWasm and message calls can spawn through several smart contracts (See Section [4.4.]) to see how the call graph is handled).
This messaging architecture reduces the coupling between actors to a minimum and enhances the scalability of the model.

CosmWasm defines the following standard for messages.

```rust
pub enum CosmosMsg<T = Empty>
where
    T: Clone + fmt::Debug + PartialEq + JsonSchema,
{
    Bank(BankMsg),
    /// This can be defined by each blockchain as a custom extension
    Custom(T),
    Staking(StakingMsg),
    Distribution(DistributionMsg),
    Stargate {
        type_url: String,
        value: Binary,
    },
    Ibc(IbcMsg),
    Wasm(WasmMsg),
} 
```

Actors define an entrypoint for dealing with messages and decide how to process them and alter its internal state.

```rust
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> { }
```
### 4.2.1 Submessages
Messages are used to interact with modules or other smart contracts.
Given the actor model and the loose coupling between actors, messages are sent but no state is kept.
This means there is no response on whether the call result was successful or not.

In some scenarios, the ability to get a result and decide whether the transaction MUST be aborted or not is extremely useful.
For this purpose, submessages are introduced: a second message sent with the sole purpose of getting a call result from a previous message.

```rust
pub struct SubMsg<T> {
    pub id: u64,                // reply_id that will be used to handle the reply
    pub msg: CosmosMsg<T>,      // message to be sent
    pub gas_limit: Option<u64>, // gas limit for the submessage
    pub reply_on: ReplyOn,      // a flag to determine when the reply should be sent
}
```
Submessages offer different patterns depending on the strategy to handle replies.
Please note that a sub-message failure always induces a revert of the sub-transaction that executed it.
Therefore, you get the result but the state on the destination actor has been already committed or reverted.
This does not revert the calling contract state.
It will simply return an error the calling contract will decide how to handle according to the following patterns:
```rust
pub enum ReplyOn {
    /// Always perform a callback after SubMsg is processed
    Always,
    /// Only callback if SubMsg returned an error, no callback on success case
    Error,
    /// Only callback if SubMsg was successful, no callback on error case
    Success,
    /// Never make a callback - this is like the original CosmosMsg semantics
    Never,
}
```
If the caller decides to not handle the error, its state will be reverted. 

### 4.2.2 Query
Queries are read-only messages.
They are used to query the state of another actor in the system.

```rust
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
  // ResolveAddress returns the current address that the name resolves to
  ResolveRecord { name: String },
  Config {},
} 
```

## 4.3. Calling a Contract
Contract to contract calls are structured through messaging.
Calling another contract requires messages to go through a `Dispatcher` that maintains the state related to the calls.
The purpose of the dispatcher is twofold: first, it prevents contracts from calling other contracts directly to prevent reentrancy attacks.
Second , it helps to maintain atomicity and revert if a call to another contract fails.

## 4.4. Contract Call Graph
A call graph is a flow graph that represents the calling relationships between actors and theirs subroutines. 
For CosmWasm this means the call graph represents the transactional flow of messages between smart contracts.
These dependencies can grow fast, specially in cross-chain scenarios. 
Precedence is determined using a width-first approach.
This means if an actor needs to process two messages `M1` and `M2` from actor `A1`, and two messages  `N1` and `N2` from actor `A2`, the order will be: `M1`, `N1`, `M2`, `N2`. 

To ensure atomicity of execution, the `Dispatcher` maintains the intermediate states across calls.
This, together with the use of escrow accounts that lock the funds along the call time, ensure that the contract is able to atomically revert, no matter how large the graph call is or when the error occurs.

## 4.5. Custom Message
Custom messages are a special kind of message that encapsulate a specific chain extension to the standard message types.
The implementations if left open to developers, but it MUST comply with the `CosmosMsg` traits.
They are used to communicate with custom native modules within a given chain.
They SHOULD be immutable, but there is no portability guarantees for custom messages.

## 4.6. Custom Query
Custom queries are a special kind of query that are used to deal with custom native modules within a given chain.
Contracts willing to enable custom queries MUST expose a `query` function  that access data storage in read-only mode.
Data format, for both the query and the response, can be customized, but SHOULD be documented.
They MUST comply with the `QueryRequest` traits.

# 5. Serialization Format
Serialization MUST be done using JSON format except when indicated otherwise.
As JSON is an industry standard, human-readable and simple.
Details for serialization are up to the implementors, but they MUST provide deterministic results.

# 6. IBC
Inter-Blockchain Communication ([IBC](https://ibcprotocol.org/documentation)) protocol allows for direct communication between independent blockchains.
We devote this section to formalize the requirements for our implementation to be IBC compatible.

## 6.1. Enabling IBC

Implementors willing to support IBC MUST enable the [**stargate** feature](https://github.com/ComposableFi/cosmwasm-vm/blob/b4896e068b8e58faae08d94728ad5684f668043b/vm/Cargo.toml#L11).

## 6.2. Host Functions

Implementors MUST enable the IBC extension (see section [6.1.]).

Implementors MUST declare the IBC host functions (see section [6.2.1.] up to [6.2.3.]).

**IBC capable** contracts (see section [2.1.2.](#212-ibc-exports)) MAY issue IBC operation via an [IbcMsg](https://github.com/CosmWasm/cosmwasm/blob/6033d91aab8dae99c0fe3b4ecf649242b21fde8a/packages/std/src/ibc.rs#L23) variant submessage of [CosmosMsg](https://github.com/CosmWasm/cosmwasm/blob/e161a7648170e7f740bc09c91c7f199dced3879c/packages/std/src/results/cosmos_msg.rs#L27).

Note that both `Stargate` and `Gov` variant of `CosmosMsg` are not supported by the VM, even if the IBC extension is enabled (see section [6.1.]).

```rust
pub enum CosmosMsg<T = Empty> {
    Bank(BankMsg),
    Custom(T),
    #[cfg(feature = "staking")]
    Staking(StakingMsg),
    #[cfg(feature = "staking")]
    Distribution(DistributionMsg),
    #[cfg(feature = "stargate")]
    Stargate {
        type_url: String,
        value: Binary,
    },
    #[cfg(feature = "stargate")]
    Ibc(IbcMsg),
    Wasm(WasmMsg),
    #[cfg(feature = "stargate")]
    Gov(GovMsg),
}

pub enum IbcMsg {
    Transfer {
        channel_id: String,
        to_address: String,
        amount: Coin,
        timeout: IbcTimeout,
    },
    SendPacket {
        channel_id: String,
        data: Binary,
        /// when packet times out, measured on remote chain
        timeout: IbcTimeout,
    },
    CloseChannel { channel_id: String },
}
```

### 6.2.1. IBC Transfer

Upon reception of the `IbcMsg::Transfer` variant submessage by the VM, the according `ibc_transfer` host function is called.

```rust
#[cfg(feature = "stargate")]
fn ibc_transfer(
    &mut self,
    channel_id: String,
    to_address: String,
    amount: Coin,
    timeout: IbcTimeout,
) -> Result<(), Self::Error>;
```

Implementors MUST verify that the `channel_id` exists and that it has previously been opened (see section [2.11.](#211-ibc-channel-open)).

Implementors MUST ensure that the `sender` balance can be reduced by the `amount`.

Implementors MAY forward the call to a specific module in charge of handling IBC operations.

### 6.2.2. IBC Packet Send

Upon reception of the `IbcMsg::SendPacket` variant submessage by the VM, the according `ibc_send_packet` host function is called.

```rust
#[cfg(feature = "stargate")]
fn ibc_send_packet(
    &mut self,
    channel_id: String,
    data: Binary,
    timeout: IbcTimeout,
) -> Result<(), Self::Error>;
```

Implementors MUST verify that the `channel_id` exists and that it has previously been opened (see section [2.11.](#211-ibc-channel-open)).

Implementors MAY forward the call to a specific module in charge of handling IBC operations.

### 6.2.3. IBC Channel Close

Upon reception of the `IbcMsg::CloseChannel` variant submessage by the VM, the according `ibc_channel_close` host function is called.

```rust
#[cfg(feature = "stargate")]
fn ibc_close_channel(&mut self, channel_id: String) -> Result<(), Self::Error>;
```

Implementors MUST verify that the `channel_id` exists and that it has previously been opened (see section [2.11.](#211-ibc-channel-open)).

Implementors MAY forward the call to a specific module in charge of handling IBC operations.

# 7. Pallet-CosmWasm
In this section we provide actual implementation details of the specification as a Substrate pallet.
We cover some of the theoretical topics that were presented before, and provide real code that exemplifies how they can be developed in a pallet.


## 7.1. Gas Metering
When a contract is uploaded through the `upload` extrinsic, its code is instrumented.
This instrumentation process adds both gas metering and stack height limit to the uploaded code.

Proper gas metering is achieved in two steps:
1. **Benchmarking**: Each instruction is benchmarked to determine the cost of its execution.
2. **Injecting Gas Metering**: Gas metering is injected into the code by using [wasm_instrument](https://github.com/paritytech/wasm-instrument). This process injects a call into the gas function for every execution or path/code block (function call, if, else, etc.) with the associated execution cost parameter. Therefore, it computes the overall total cost and ensures that every code block is paid before getting executed.

Then, whenever a contract entrypoint is called, the pallet checks if the instrumentation version is up-to-date.
In case not, the code gets re-instrumented to ensure proper gas metering.

## 7.2. Uploading Contracts
The `upload` extrinsic is used to upload smart contracts´ `code` to the pallet.

### 7.2.1. Definition
```rust
#[pallet::weight(T::WeightInfo::upload(code.len() as u32))]
pub fn upload(origin: OriginFor<T>, code: ContractCodeOf<T>) -> DispatchResultWithPostInfo;
```

### 7.2.2. Execution Flow
1. Check if the `code` is already uploaded.
2. Reserve `length(code) * deposit_per_byte` amount of native asset.
3. Check if the wasm code is valid.
4. Do the instrumentation by injecting gas metering and stack height limit.
5. Assign a code id to the `code`. This `code id ` is incremented on each upload.
6. Deposit the upload event.

```rust
pub enum Event<T: Config> {
	Uploaded {
		code_hash: CodeHashOf<T>,
		code_id: CosmwasmCodeId,
	}
```

### 7.2.3. Fee
Fees depend linearly on the size of the code.

## 7.3. Instantiating a Contract
The `instantiate` extrinsic is used to instantiate a smart contract.

### 7.3.1 Definition
```rust
#[pallet::weight(T::WeightInfo::instantiate(funds.len() as u32).saturating_add(*gas))]
pub fn instantiate(
	origin: OriginFor<T>,
	code_identifier: CodeIdentifier<T>,
	salt: ContractSaltOf<T>,
	admin: Option<AccountIdOf<T>>,
	label: ContractLabelOf<T>,
	funds: FundsOf<T>,
	gas: u64,
	message: ContractMessageOf<T>,
) -> DispatchResultWithPostInfo;
```

`origin` will be the `sender` from the contracts' perspective.

Our goal is to create a deterministic smart contract environment.
Hence, we are not only using `code id` to identify a code.
Since `code id` depends on the current state of the chain, users won't be able to deterministically identify their code.
So we created `CodeIdentifier` which makes users able to also identify their code by using `CodeHash`.
And the corresponding `code id` is fetched internally.
This feature comes in handy when users want to batch their `upload + instantiate` calls, or they do any kind of scripting to upload and run the contracts.
```rust
pub enum CodeIdentifier<T: Config> {
	CodeId(CosmwasmCodeId),
	CodeHash(CodeHashOf<T>),
}
```

`admin` is the optional owner of the contract. Note that if it is set to `None`, the contract cannot ever be migrated or do any `admin` operations.
Therefore, it will become an immutable contract.
The `label` field is used as a human-readable `String` for the instantiated contract.
`salt` is used when a user wants to instantiate the same contract with the same parameters twice.
This ensures that during the contract address generation, addresses remain unique.
`funds` are transferred to the contract prior to instantiation.
Then, new balances will be visible to the contract.
`gas` represents the gas limit, and, `message` field is passed to the contract as the `InstantiateMsg`.

### 7.3.2 Execution Flow
First, the contract address MUST be derived.
As stated previously, one of our goals is determinism.
Then, the contract addresses are also deterministic as opposed to other CosmWasm-running chains.
The algorithm is based on `instantiator`, `salt`, `code_hash` and `message` which is:
```
hash(instantiator + salt + code_hash + hash(message))
```
This gives users opportunity to know the contract address prior to creation, which becomes really handy when it comes to XCVM.
Because this will provide a way to know the `interpreter` address prior to the creation for example so that users can add `Transfer` instruction which transfers some funds to the `interpreter` without paying for late-binding.

Then the necessary setup is done like deriving a contract trie id for storage, increasing the `refcount` of the code.
Finally, instrumentation version is checked, and re-instrumentation happens if necessary.
Then, the `instantiate` entrypoint of the contract is called and `Instantiated` event is yielded.

```rust
pub enum Event<T: Config> {
	Instantiated {
		contract: AccountIdOf<T>,
		info: ContractInfoOf<T>,
	}
}
```

### 7.3.3 Fee
Total fees depend on three factors:
- Instructions to be run.
- Base cost of instantiate call
- Funds to be transferred.

The total fee can be computed as follows.
```
base_instantiate_fee + (fee_per_fund * length(funds)) + executed_instruction_costs
```
The remaining gas is refunded after execution.

## 7.4. Executing a Contract
The `execute` extrinsic is used for executing a smart contract.

### 7.4.1. Definition
```rust
#[pallet::weight(T::WeightInfo::execute(funds.len() as u32).saturating_add(*gas))]
pub fn execute(
	origin: OriginFor<T>,
	contract: AccountIdOf<T>,
	funds: FundsOf<T>,
	gas: Gas,
	message: ContractMessageOf<T>,
) -> DispatchResultWithPostInfo;
```

The `contract` field contains the contract address to execute.
The `funds` are transferred to the contract prior to instantiation.
So that the new balances will be visible to the contract.
`gas` represents the gas limit, and, `message` is passed to the contract as the `ExecuteMsg`.

### 7.4.2 Execution Flow
The execution flow is reduced to a minimum for contract execution.
Only a check for re-instrumentation is performed, and then the execution of the `execute` entrypoint of the contract is triggered.

### 7.4.3 Fee
Total fees depend on three factors:
- Instructions to be run.
- Base cost of instantiate call
- Funds to be transferred.

The total fee can be computed as follows.
```
base_instantiate_fee + (fee_per_fund * length(funds)) + executed_instruction_costs
```
The remaining gas is refunded after execution.


# 8. Security considerations
There have been a number of vulnerabilities in different smart-contract execution environments, often enabled because of small implementation mistakes. Each of these has unique circumstances.
Instead of going into each of these in-depth, we recommend:

1. Implementors to always use third-party auditors to verify their implementations.

2. Make use of fuzzing and verification frameworks to proof correctness. Some libraries we recommend are:

    - [kani](https://github.com/model-checking/kani)
    - [proptest](https://docs.rs/proptest/latest/proptest)
    - [fuzzing](https://github.com/rust-fuzz/cargo-fuzz)

## 8.1. Previous Vulnerabilities

- https://halborn.com/halborn-discovers-zero-day-vulnerability-in-cosmwasm

# 9. Contributors
Contributors that made our CosmWasm implementation possible are here listed in alphabetical order:

- Hussein Ait Lahcen - XCVM Principal Engineering - Composable Finance
- Abdullah Eryuzlu - Rust Developer - Composable Finance
- Cor Pruijs - Rust Developer - Composable Finance

[2.5.1.]: #251-environment-pointer
[2.5.2.]: #252-information-pointer
[2.5.2.3.]: #2523-contract-info
[2.5.3.]: #253-regions
[2.14.]: #214-ibc-packet-receive
[3.4.]: #34-db-scan
[3.6.]: #36-address-validate
[3.7.]: #37-address-canonicalize
[3.8.]: #38-address-humanize
[4.4.]: #44-contract-call-graph
[6.]: #6-ibc
[6.1.]: #61-enabling-ibc
[6.2.1.]: #621-ibc-transfer
[6.2.3]: #623-ibc-channel-close

