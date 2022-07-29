// SPDX-License-Identifier: MIT

pragma solidity >=0.6.2;

import "./IGmxVault.sol";

interface IGmxReader {
    function getAmountOut(
        IGmxVault _vault,
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn
    ) external view returns (uint256, uint256);
}
