// SPDX-License-Identifier: MIT

pragma solidity ^0.8.14;
pragma experimental ABIEncoderV2;

interface IGateway {

    enum BridgeSecurity {
        Disabiled,
        Deterministic,
        Probabilistic,
        Optimistic
    }

    struct Origin {
        uint32 networkId;
        bytes account;
    }
    function assets(uint256 assetId) external returns (address);
}
