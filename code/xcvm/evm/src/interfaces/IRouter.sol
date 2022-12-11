// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
pragma experimental ABIEncoderV2;

interface IRouter {
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

    function getAsset(uint256 assetId) external view returns (address);

    function getAssetIdByLocalId(address asset) external view returns (uint256);

    function getBridge(uint256 networkId, BridgeSecurity security) external view returns (address);

    function runProgram(
        Origin memory origin,
        bytes memory program,
        address[] memory _assets,
        uint256[] memory _amounts
    ) external payable returns (bool);

    function emitSpawn(
        bytes memory account,
        uint256 networkId,
        BridgeSecurity security,
        bytes memory salt,
        bytes memory spawnedProgram,
        address[] memory assetAddresses,
        uint128[] memory assetIds,
        uint256[] memory amounts
    ) external;
}
