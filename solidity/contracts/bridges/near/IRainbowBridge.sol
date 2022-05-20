// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IRainbowBridge {
    function lockToken(
        address ethToken,
        uint256 amount,
        string calldata accountId
    ) external payable;
}
