// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface ISummonerConfig {
    function getTransferLockupTime() external view returns (uint256);

    function getFeeTokenAmount(uint256 remoteNetworkId, address feeToken)
        external
        view
        returns (uint256);

    function getPausedNetwork(uint256 networkId) external view returns (bool);
}
