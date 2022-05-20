// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IBridgeRouter {
    event Send(
        address indexed token,
        address indexed from,
        uint32 indexed toDomain,
        bytes32 toId,
        uint256 amount
    );

    function send(
        address _token,
        uint256 _amount,
        uint32 _destination,
        bytes32 _recipient
    ) external;
}
