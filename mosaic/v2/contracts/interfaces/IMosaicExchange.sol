// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IMosaicExchange {
    function swap(
        address _tokenA,
        address _tokenB,
        uint256 _amountIn,
        uint256 _amountOut,
        bytes calldata _data
    ) external returns (uint256);

    function getAmountsOut(
        address _tokenIn,
        address _tokenOut,
        uint256 _amountIn,
        bytes calldata _data
    ) external returns (uint256);
}
