// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;

import "./types/Channel.sol";

interface IModuleCallbacks {
    function onChanOpenInit(Channel.Order, string[] calldata connectionHops, string calldata portId, string calldata channelId, ChannelCounterparty.Data calldata counterparty, string calldata version) external;
    function onChanOpenTry(Channel.Order, string[] calldata connectionHops, string calldata portId, string calldata channelId, ChannelCounterparty.Data calldata counterparty, string calldata version, string calldata counterpartyVersion) external;
    function onChanOpenAck(string calldata portId, string calldata channelId, string calldata counterpartyVersion) external;
    function onChanOpenConfirm(string calldata portId, string calldata channelId) external;
    function onChanCloseInit(string calldata portId, string calldata channelId) external;
    function onChanCloseConfirm(string calldata portId, string calldata channelId) external;

    function onRecvPacket(Packet.Data calldata, address relayer) external returns(bytes memory);
    function onAcknowledgementPacket(Packet.Data calldata, bytes calldata acknowledgement, address relayer) external;
}
