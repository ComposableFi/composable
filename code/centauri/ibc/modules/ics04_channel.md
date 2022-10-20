## ICS04_CHANNEL

IBC channels are direct links between identical deployments of an application on connected chains. Channels are built on top of open connections.

### Channel Context

The channel context encapsulates all the storage requirements for channels in the context object.    
implement the [`ChannelReader`](/code/centauri/ibc/modules/src/core/ics04_channel/context.rs#L26) and [`ChannelKeeper`](/code/centauri/ibc/modules/src/core/ics04_channel/context.rs#L107) for the context object

```text
    impl ChannelReader for Context { ... }
    
    impl ChannelKeeper for Context { ... }  
```

### Channel Messages and Events

There are four messages that describe the channel open handshake process and two for the channel close process.

**MsgChannelOpenInit**

This message is submitted to start the channel handshake process, there's nothing to prove at this point, so it requires no proof.  
This message contains the channel_end port_id and signer.

```rust
    pub struct MsgChannelOpenInit {
        pub port_id: PortId,
        pub channel: ChannelEnd,
        pub signer: Signer,
    }
```
This message is processed by the [`chan_open_init`](/code/centauri/ibc/modules/src/core/ics04_channel/handler/chan_open_init.rs) handler.  
The `OpenInitChannel` event is emitted on successful processing of the event.


**MsgChannelOpenTry**

This message is submitted to a chain after `MsgChannelOpenInit` has been executed on its counterparty, this message requires a proof that indicates that the channel was  
initialized on the counterparty and committed to it's state tree.

```rust
    pub struct MsgChannelOpenTry {
        pub port_id: PortId,
        pub channel: ChannelEnd,
        pub counterparty_version: Version,
        pub proofs: Proofs,
        pub signer: Signer,
    }
```
This message is processed by the [`chan_open_try`](/code/centauri/ibc/modules/src/core/ics04_channel/handler/chan_open_try.rs) handler.  
The `OpenTryChannel` event is emitted on successful processing of the event.


**MsgChannelOpenAck**

This message is submitted to a chain after `MsgChannelOpenTry` has been executed on its counterparty, this message requires a proof that indicates that the channel was  
initialized with a state of `TryOpen` on the counterparty and committed to it's state tree

```rust
    pub struct MsgChannelOpenAck {
        pub port_id: PortId,
        pub channel_id: ChannelId,
        pub counterparty_channel_id: ChannelId,
        pub counterparty_version: Version,
        pub proofs: Proofs,
        pub signer: Signer,
    }
```
This message is processed by the [`chan_open_ack`](/code/centauri/ibc/modules/src/core/ics04_channel/handler/chan_open_ack.rs) handler.  
The `OpenAckChannel` event is emitted on successful processing of the event.  
The channel becomes open if this message is processed successfully.

**MsgChannelOpenConfirm**

This message is submitted to a chain after `MsgChannelOpenAck` has been executed on its counterparty, this message requires a proof that indicates that the channel is `Open` on the counterparty.

```rust
    pub struct MsgChannelOpenConfirm {
        pub port_id: PortId,
        pub channel_id: ChannelId,
        pub proofs: Proofs,
        pub signer: Signer,
    }
```
This message is processed by the [`chan_open_confirm`](/code/centauri/ibc/modules/src/core/ics04_channel/handler/chan_open_confirm.rs) handler.  
The `OpenConfirmChannel` event is emitted on successful processing of the event.  
The channel becomes open if this message is processed successfully.

**MsgChannelCloseInit**

This message is submitted to start the channel close process, there's nothing to prove at this point, so it requires no proof.

```rust
    pub struct MsgChannelCloseInit {
        pub port_id: PortId,
        pub channel_id: ChannelId,
        pub signer: Signer,
    }
```
This message is processed by the [`chan_close_init`](/code/centauri/ibc/modules/src/core/ics04_channel/handler/chan_close_init.rs) handler.  
The `CloseInitChannel` event is emitted on successful processing of the event.

**MsgChannelOpenConfirm**

This message is submitted to a chain after `MsgChannelCloseInit` has been executed on its counterparty, this message requires a proof that indicates that the channel state is `Close` on the counterparty.

```rust
    pub struct MsgChannelOpenConfirm {
        pub port_id: PortId,
        pub channel_id: ChannelId,
        pub proofs: Proofs,
        pub signer: Signer,
    }
```
This message is processed by the [`chan_close_confirm`](/code/centauri/ibc/modules/src/core/ics04_channel/handler/chan_close_confirm.rs) handler.  
The `CloseConfirmChannel` event is emitted on successful processing of the event.  
The channel becomes closed if this message is processed successfully.

### Packets

Packets are means through which protocols built on top of core ibc exchange data, ibc application protocols determine how this data is serialized and deserialized. 
This packet data remains opaque to the core ibc protocol which facilitates its transmission across connections through channels.

**SendPacket**

This is not like other messages, it just involves creating a packet and depositing the packet commitment in the state and emitting the `SendPacketEvent`.
To handle packet creation call the [`send_packet`](/code/centauri/ibc/modules/src/core/ics04_channel/handler/send_packet.rs) method.
