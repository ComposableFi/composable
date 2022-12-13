// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;

import "./types/Client.sol";
import "./types/Channel.sol";
import "./IBCConnection.sol";
import "./IBCMsgs.sol";
import "./IBCHost.sol";
import "./IBCHeight.sol";

library IBCChannel {
    using IBCHeight for Height.Data;

    function channelOpenInit(
        IBCHost host,
        IBCMsgs.MsgChannelOpenInit memory msg_
    ) public returns (string memory) {
        host.onlyIBCModule();
        ConnectionEnd.Data memory connection;
        require(msg_.channel.connection_hops.length == 1, "connection_hops length must be 1");
        connection = mustGetConnection(host, msg_.channel);
        require(connection.versions.length == 1, "single version must be negotiated on connection before opening channel");
        require(msg_.channel.state == Channel.State.STATE_INIT, "channel state must STATE_INIT");

        // TODO verifySupportedFeature

        // TODO authenticates a port binding

        string memory channelId = host.generateChannelIdentifier();
        host.setChannel(msg_.portId, channelId, msg_.channel);
        host.setNextSequenceSend(msg_.portId, channelId, 1);
        host.setNextSequenceRecv(msg_.portId, channelId, 1);
        host.setNextSequenceAck(msg_.portId, channelId, 1);

        return channelId;
    }

    function channelOpenTry(
        IBCHost host,
        IBCMsgs.MsgChannelOpenTry memory msg_
    ) public returns (string memory) {
        host.onlyIBCModule();
        ConnectionEnd.Data memory connection;
        require(msg_.channel.connection_hops.length == 1, "connection_hops length must be 1");
        connection = mustGetConnection(host, msg_.channel);
        require(connection.versions.length == 1, "single version must be negotiated on connection before opening channel");
        require(msg_.channel.state == Channel.State.STATE_TRYOPEN, "channel state must be STATE_TRYOPEN");

        // TODO verifySupportedFeature

        // TODO authenticates a port binding

        ChannelCounterparty.Data memory expectedCounterparty = ChannelCounterparty.Data({
            port_id: msg_.portId,
            channel_id: ""
        });
        Channel.Data memory expectedChannel = Channel.Data({
            state: Channel.State.STATE_INIT,
            ordering: msg_.channel.ordering,
            counterparty: expectedCounterparty,
            connection_hops: getCounterpartyHops(host, msg_.channel),
            version: msg_.counterpartyVersion
        });
        require(IBCConnection.verifyChannelState(host, connection, msg_.proofHeight, msg_.proofInit, msg_.channel.counterparty.port_id, msg_.channel.counterparty.channel_id, Channel.encode(expectedChannel)), "failed to verify channel state");

        string memory channelId = host.generateChannelIdentifier();
        host.setChannel(msg_.portId, channelId, msg_.channel);
        host.setNextSequenceSend(msg_.portId, channelId, 1);
        host.setNextSequenceRecv(msg_.portId, channelId, 1);
        host.setNextSequenceAck(msg_.portId, channelId, 1);

        return channelId;
    }

    function channelOpenAck(
        IBCHost host,
        IBCMsgs.MsgChannelOpenAck memory msg_
    ) public {
        host.onlyIBCModule();
        Channel.Data memory channel;
        ConnectionEnd.Data memory connection;

        channel = mustGetChannel(host, msg_.portId, msg_.channelId);
        require(channel.state == Channel.State.STATE_INIT || channel.state == Channel.State.STATE_TRYOPEN, "invalid channel state");

        // TODO authenticates a port binding

        connection = mustGetConnection(host, channel);
        require(connection.state == ConnectionEnd.State.STATE_OPEN, "connection state is not OPEN");

        ChannelCounterparty.Data memory expectedCounterparty = ChannelCounterparty.Data({
            port_id: msg_.portId,
            channel_id: msg_.channelId
        });
        Channel.Data memory expectedChannel = Channel.Data({
            state: Channel.State.STATE_TRYOPEN,
            ordering: channel.ordering,
            counterparty: expectedCounterparty,
            connection_hops: getCounterpartyHops(host, channel),
            version: msg_.counterpartyVersion
        });
        require(IBCConnection.verifyChannelState(host, connection, msg_.proofHeight, msg_.proofTry, channel.counterparty.port_id, msg_.counterpartyChannelId, Channel.encode(expectedChannel)), "failed to verify channel state");
        channel.state = Channel.State.STATE_OPEN;
        channel.version = msg_.counterpartyVersion;
        channel.counterparty.channel_id = msg_.counterpartyChannelId;
        host.setChannel(msg_.portId, msg_.channelId, channel);
    }

    function channelOpenConfirm(
        IBCHost host,
        IBCMsgs.MsgChannelOpenConfirm memory msg_
    ) public {
        host.onlyIBCModule();
        Channel.Data memory channel;
        ConnectionEnd.Data memory connection;

        channel = mustGetChannel(host, msg_.portId, msg_.channelId);
        require(channel.state == Channel.State.STATE_TRYOPEN, "channel state is not TRYOPEN");

        // TODO authenticates a port binding

        connection = mustGetConnection(host, channel);
        require(connection.state == ConnectionEnd.State.STATE_OPEN, "connection state is not OPEN");

        ChannelCounterparty.Data memory expectedCounterparty = ChannelCounterparty.Data({
            port_id: msg_.portId,
            channel_id: msg_.channelId
        });
        Channel.Data memory expectedChannel = Channel.Data({
            state: Channel.State.STATE_OPEN,
            ordering: channel.ordering,
            counterparty: expectedCounterparty,
            connection_hops: getCounterpartyHops(host, channel),
            version: channel.version
        });
        require(IBCConnection.verifyChannelState(host, connection, msg_.proofHeight, msg_.proofAck, channel.counterparty.port_id, channel.counterparty.channel_id, Channel.encode(expectedChannel)), "failed to verify channel state");
        channel.state = Channel.State.STATE_OPEN;
        host.setChannel(msg_.portId, msg_.channelId, channel);
    }

    function channelCloseInit(
        IBCHost host,
        IBCMsgs.MsgChannelCloseInit memory msg_
    ) public {
        host.onlyIBCModule();
        Channel.Data memory channel;
        ConnectionEnd.Data memory connection;

        channel = mustGetChannel(host, msg_.portId, msg_.channelId);
        require(channel.state != Channel.State.STATE_CLOSED, "channel state is already CLOSED");

        // TODO authenticates a port binding

        connection = mustGetConnection(host, channel);
        require(connection.state == ConnectionEnd.State.STATE_OPEN, "connection state is not OPEN");

        channel.state = Channel.State.STATE_CLOSED;
        host.setChannel(msg_.portId, msg_.channelId, channel);
    }

    function channelCloseConfirm(
        IBCHost host,
        IBCMsgs.MsgChannelCloseConfirm memory msg_
    ) public {
        host.onlyIBCModule();
        Channel.Data memory channel;
        ConnectionEnd.Data memory connection;

        channel = mustGetChannel(host, msg_.portId, msg_.channelId);
        require(channel.state != Channel.State.STATE_CLOSED, "channel state is already CLOSED");

        // TODO authenticates a port binding

        connection = mustGetConnection(host, channel);
        require(connection.state == ConnectionEnd.State.STATE_OPEN, "connection state is not OPEN");

        ChannelCounterparty.Data memory expectedCounterparty = ChannelCounterparty.Data({
            port_id: msg_.portId,
            channel_id: msg_.channelId
        });
        Channel.Data memory expectedChannel = Channel.Data({
            state: Channel.State.STATE_CLOSED,
            ordering: channel.ordering,
            counterparty: expectedCounterparty,
            connection_hops: getCounterpartyHops(host, channel),
            version: channel.version
        });
        require(IBCConnection.verifyChannelState(host, connection, msg_.proofHeight, msg_.proofInit, channel.counterparty.port_id, channel.counterparty.channel_id, Channel.encode(expectedChannel)), "failed to verify channel state");
        channel.state = Channel.State.STATE_CLOSED;
        host.setChannel(msg_.portId, msg_.channelId, channel);
    }

    function sendPacket(IBCHost host, Packet.Data calldata packet) external {
        host.onlyIBCModule();
        Channel.Data memory channel;
        ConnectionEnd.Data memory connection;
        IClient client;
        Height.Data memory latestHeight;
        uint64 latestTimestamp;
        uint64 nextSequenceSend;
        bool found;

        channel = mustGetChannel(host, packet.source_port, packet.source_channel);
        require(channel.state == Channel.State.STATE_OPEN, "channel state must be OPEN");
        require(hashString(packet.destination_port) == hashString(channel.counterparty.port_id), "packet destination port doesn't match the counterparty's port");
        require(hashString(packet.destination_channel) == hashString(channel.counterparty.channel_id), "packet destination channel doesn't match the counterparty's channel");
        connection = mustGetConnection(host, channel);
        client = IBCClient.getClient(host, connection.client_id);
        (latestHeight, found) = client.getLatestHeight(host, connection.client_id);
        require(packet.timeout_height.isZero() || latestHeight.lt(packet.timeout_height), "receiving chain block height >= packet timeout height");
        (latestTimestamp, found) = client.getTimestampAtHeight(host, connection.client_id, latestHeight);
        require(found, "consensusState not found");
        require(packet.timeout_timestamp == 0 || latestTimestamp < packet.timeout_timestamp, "receiving chain block timestamp >= packet timeout timestamp");

        nextSequenceSend = host.getNextSequenceSend(packet.source_port, packet.source_channel);
        require(nextSequenceSend > 0, "sequenceSend not found");
        require(packet.sequence == nextSequenceSend, "packet sequence != next send sequence");

        nextSequenceSend++;
        host.setNextSequenceSend(packet.source_port, packet.source_channel, nextSequenceSend);
        host.setPacketCommitment(packet.source_port, packet.source_channel, packet.sequence, packet);

        // TODO emit an event that includes a packet
    }

    function recvPacket(IBCHost host, IBCMsgs.MsgPacketRecv calldata msg_) external {
        host.onlyIBCModule();
        Channel.Data memory channel;
        ConnectionEnd.Data memory connection;
        channel = mustGetChannel(host, msg_.packet.destination_port, msg_.packet.destination_channel);
        require(channel.state == Channel.State.STATE_OPEN, "channel state must be OPEN");

        // TODO
        // Authenticate capability to ensure caller has authority to receive packet on this channel

        require(hashString(msg_.packet.source_port) == hashString(channel.counterparty.port_id), "packet source port doesn't match the counterparty's port");
        require(hashString(msg_.packet.source_channel) == hashString(channel.counterparty.channel_id), "packet source channel doesn't match the counterparty's channel");

        connection = mustGetConnection(host, channel);
        require(connection.state == ConnectionEnd.State.STATE_OPEN, "connection state is not OPEN");

        require(msg_.packet.timeout_height.revision_height == 0 || block.number < msg_.packet.timeout_height.revision_height, "block height >= packet timeout height");
        require(msg_.packet.timeout_timestamp == 0 || block.timestamp < msg_.packet.timeout_timestamp, "block timestamp >= packet timeout timestamp");

        bytes32 commitment = host.makePacketCommitment(msg_.packet);
        require(IBCConnection.verifyPacketCommitment(host, connection, msg_.proofHeight, msg_.proof, msg_.packet.source_port, msg_.packet.source_channel, msg_.packet.sequence, commitment), "failed to verify packet commitment");

        if (channel.ordering == Channel.Order.ORDER_UNORDERED) {
            require(!host.hasPacketReceipt(msg_.packet.destination_port, msg_.packet.destination_channel, msg_.packet.sequence), "packet sequence already has been received");
            host.setPacketReceipt(msg_.packet.destination_port, msg_.packet.destination_channel, msg_.packet.sequence);
        } else if (channel.ordering == Channel.Order.ORDER_ORDERED) {
            uint64 nextSequenceRecv = host.getNextSequenceRecv(msg_.packet.destination_port, msg_.packet.destination_channel);
            require(nextSequenceRecv > 0 && nextSequenceRecv == msg_.packet.sequence, "packet sequence != next receive sequence");
            host.setNextSequenceRecv(msg_.packet.destination_port, msg_.packet.destination_channel, nextSequenceRecv+1);
        } else {
            revert("unknown ordering type");
        }
    }

    // WriteAcknowledgement writes the packet execution acknowledgement to the state,
    // which will be verified by the counterparty chain using AcknowledgePacket.
    function writeAcknowledgement(IBCHost host, string calldata destinationPortId, string calldata destinationChannel, uint64 sequence, bytes calldata acknowledgement) external {
        host.onlyIBCModule();
        Channel.Data memory channel;
        bytes32 ackHash;
        bool found;
        channel = mustGetChannel(host, destinationPortId, destinationChannel);
        require(channel.state == Channel.State.STATE_OPEN, "channel state must be OPEN");

        (ackHash, found) = host.getPacketAcknowledgementCommitment(destinationPortId, destinationChannel, sequence);
        require(!found, "acknowledgement for packet already exists");

        require(acknowledgement.length > 0, "acknowledgement cannot be empty");
        host.setPacketAcknowledgementCommitment(destinationPortId, destinationChannel, sequence, acknowledgement);
    }

    // TODO use calldata
    function acknowledgePacket(IBCHost host, IBCMsgs.MsgPacketAcknowledgement memory msg_) public {
        host.onlyIBCModule();
        Channel.Data memory channel;
        ConnectionEnd.Data memory connection;
        bytes32 commitment;
        bool found;
        channel = mustGetChannel(host, msg_.packet.source_port, msg_.packet.source_channel);
        require(channel.state == Channel.State.STATE_OPEN, "channel state must be OPEN");

        require(hashString(msg_.packet.destination_port) == hashString(channel.counterparty.port_id), "packet destination port doesn't match the counterparty's port");
        require(hashString(msg_.packet.destination_channel) == hashString(channel.counterparty.channel_id), "packet destination channel doesn't match the counterparty's channel");

        connection = mustGetConnection(host, channel);
        require(connection.state == ConnectionEnd.State.STATE_OPEN, "connection state is not OPEN");

        (commitment, found) = host.getPacketCommitment(msg_.packet.source_port, msg_.packet.source_channel, msg_.packet.sequence);
        require(found, "packet commitment not found");

        require(commitment == host.makePacketCommitment(msg_.packet), "commitment bytes are not equal");

        require(IBCConnection.verifyPacketAcknowledgement(host, connection, msg_.proofHeight, msg_.proof, msg_.packet.destination_port, msg_.packet.destination_channel, msg_.packet.sequence, msg_.acknowledgement), "failed to verify packet acknowledgement commitment");

        if (channel.ordering == Channel.Order.ORDER_ORDERED) {
            uint64 nextSequenceAck = host.getNextSequenceAck(msg_.packet.source_port, msg_.packet.source_channel);
            require(nextSequenceAck == 0, "sequence ack not found");
            require(msg_.packet.sequence == nextSequenceAck, "packet sequence != next ack sequence");
            nextSequenceAck++;
            host.setNextSequenceAck(msg_.packet.source_port, msg_.packet.source_channel, nextSequenceAck);
        }

        host.deletePacketCommitment(msg_.packet.source_port, msg_.packet.source_channel, msg_.packet.sequence);
    }

    function getCounterpartyHops(IBCHost host, Channel.Data memory channel) internal view returns (string[] memory hops) {
        require(channel.connection_hops.length == 1, "connection_hops length must be 1");
        ConnectionEnd.Data memory connection = mustGetConnection(host, channel);
        hops = new string[](1);
        hops[0] = connection.counterparty.connection_id;
        return hops;
    }

    function mustGetConnection(IBCHost host_, Channel.Data memory channel_) private view returns (ConnectionEnd.Data memory) {
        (ConnectionEnd.Data memory connection, bool found) = host_.getConnection(channel_.connection_hops[0]);
        require(found, "connection not found");
        return connection;
    }

    function mustGetChannel(IBCHost host_, string memory portId_, string memory channelId_) private view returns (Channel.Data memory) {
        (Channel.Data memory channel, bool found) = host_.getChannel(portId_, channelId_);
        require(found, "channel not found");
        return channel;
    }

    function hashString(string memory s) private pure returns (bytes32) {
        return keccak256(abi.encodePacked(s));
    }

}
