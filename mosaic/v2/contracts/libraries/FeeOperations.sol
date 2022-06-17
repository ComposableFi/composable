// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

library FeeOperations {
    uint256 internal constant FEE_FACTOR = 10000;

    function getFeeAbsolute(uint256 amount, uint256 fee) internal pure returns (uint256) {
        return (amount * fee) / FEE_FACTOR;
    }
}
