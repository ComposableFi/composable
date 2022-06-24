// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IBridgeAggregator {
    /// @notice event emitted when a token is send to L2 network
    /// @param destination address of the receiver
    /// @param token address of the token
    /// @param amount token amount send
    event AssetSend(
        address indexed destination,
        address indexed token,
        uint256 amount,
        uint256 chainId
    );

    function addBridge(
        uint256 _destinationNetwork,
        uint256 _bridgeID,
        address _bridgeAddress
    ) external;

    function bridgeTokens(
        uint256 _destinationNetwork,
        uint256 _bridgeId,
        address _destination,
        address _token,
        uint256 _amount,
        bytes calldata _data
    ) external;
}
