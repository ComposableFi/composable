// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;

interface IIbcBridge {
    function sendProgram(
        bytes memory account,
        uint128 networkId,
        bytes memory salt,
        bytes memory spawnedProgram,
        uint128[] memory assetIds,
        uint256[] memory amounts
    ) external;
}