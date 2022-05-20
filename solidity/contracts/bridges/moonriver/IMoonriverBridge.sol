// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IMoonriverBridge {
    function sendERC20SToken(
        uint256 destinationChainID,
        address recipient,
        uint256 amount
    ) external;

    function sendERC721MoonToken(
        uint256 destinationChainID,
        address recipient,
        uint256 tokenId
    ) external;
}
