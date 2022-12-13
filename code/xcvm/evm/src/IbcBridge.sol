// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;

import "./ibc/core/types/Channel.sol";
import "./ibc/core/IBCModule.sol";
import "./ibc/core/IBCHandler.sol";
import "./ibc/core/IBCHost.sol";
import "./ibc/core/types/App.sol";
import "./ibc/lib/strings.sol";
import "./ibc/lib/Bytes.sol";
import "@openzeppelin/contracts/utils/Context.sol";
import "./interfaces/IRouter.sol";

import "@lazyledger/protobuf3-solidity-lib/contracts/ProtobufLib.sol";
import "./interfaces/IIbcBridge.sol";
import "./libraries/SDK.sol";

contract IBCBridge is Context, IIbcBridge, IModuleCallbacks {
    IBCHandler ibcHandler;
    IBCHost ibcHost;
    string public sourcePort;
    string public sourceChannel;
    address public routerAddress;

    constructor(address _routerAddress, IBCHost _ibcHost, IBCHandler _ibcHandler) {
        routerAddress = _routerAddress;
        ibcHost = _ibcHost;
        ibcHandler = _ibcHandler;
    }

    function sendProgram(
        bytes memory account,
        uint128 networkId,
        bytes memory salt,
        bytes memory spawnedProgram,
        uint128[] memory assetIds,
        uint256[] memory amounts
    ) external {
        require(msg.sender == routerAddress, "only router can send packet");
        bytes memory data = SDK.generateIBCSpawn(
            SDK.generateUserOrigin(account, networkId),
            SDK.generateInterpreterOrigin(account),
            salt,
            spawnedProgram,
            assetIds,
            amounts
        );
        _sendPacket(data, uint64(block.timestamp + 60 * 10)); // 10 minutes
    }


    function _sendPacket(bytes memory data, uint64 timeout) virtual internal {
        (Channel.Data memory channel, bool found) = ibcHost.getChannel(sourcePort, sourceChannel);
        require(found, "channel not found");
        ibcHandler.sendPacket(Packet.Data({
            sequence: ibcHost.getNextSequenceSend(sourcePort, sourceChannel),
            source_port: sourcePort,
            source_channel: sourceChannel,
            destination_port: channel.counterparty.port_id,
            destination_channel: channel.counterparty.channel_id,
            data: data,
            timeout_height: Height.Data({revision_number: 0, revision_height: 0}), // to 0 because remote block height is not known
            timeout_timestamp: timeout
        }));
    }


    /// Module callbacks ///
    function onRecvPacket(Packet.Data calldata packet, address relayer) external virtual override returns (bytes memory acknowledgement) {
        (bytes memory intepreterOrigin,
        IRouter.Origin memory origin,
        bytes memory program,
        bytes memory salt,
        address[] memory _assets,
        uint256[] memory _amounts) = SDK.decodeIBCSpawn(packet.data, routerAddress);
        return _newAcknowledgement(IRouter(routerAddress).runProgram(origin, salt, program, _assets, _amounts));
    }

    function onAcknowledgementPacket(Packet.Data calldata packet, bytes calldata acknowledgement, address relayer) external virtual override {
        //if (!_isSuccessAcknowledgement(acknowledgement)) {
            // TODO if failed, transferred funds should be returned to user
        //}
    }

    function _newAcknowledgement(bool success) virtual internal pure returns (bytes memory) {
        bytes memory acknowledgement = new bytes(1);
        if (success) {
            acknowledgement[0] = 0x01;
        } else {
            acknowledgement[0] = 0x00;
        }
        return acknowledgement;
    }

    function onChanOpenInit(Channel.Order, string[] calldata, string calldata, string calldata channelId, ChannelCounterparty.Data calldata, string calldata) external virtual override {
        // TODO authenticate a capability
        //channelEscrowAddresses[channelId] = address(this);
    }

    function onChanOpenTry(Channel.Order, string[] calldata, string calldata, string calldata channelId, ChannelCounterparty.Data calldata, string calldata, string calldata) external virtual override {
        // TODO authenticate a capability
        //channelEscrowAddresses[channelId] = address(this);
    }

    function onChanOpenAck(string calldata portId, string calldata channelId, string calldata counterpartyVersion) external virtual override {}

    function onChanOpenConfirm(string calldata portId, string calldata channelId) external virtual override {}

    function onChanCloseInit(string calldata portId, string calldata channelId) external virtual override {}

    function onChanCloseConfirm(string calldata portId, string calldata channelId) external virtual override {}
}