// SPDX-License-Identifier: MIT

pragma solidity ^0.8.14;
pragma experimental ABIEncoderV2;

interface IGateway {
    struct Bridge {
        uint256 networkId;
        BridgeSecurity security;
    }

    enum BridgeSecurity {
        Disabled,
        Deterministic,
        Probabilistic,
        Optimistic
    }

    struct Origin {
        uint32 networkId;
        bytes account;
    }

    function getAsset(uint256 assetId) external returns (address);

    function getBridge(uint256 networkId, BridgeSecurity security) external returns (address);

    function emitSpawn(
        bytes memory account,
        uint256 networkId,
        BridgeSecurity security,
        uint256 salt,
        bytes memory spawnedProgram,
        address[] memory assetAddresses,
        uint256[] memory amounts
    ) external;
}
