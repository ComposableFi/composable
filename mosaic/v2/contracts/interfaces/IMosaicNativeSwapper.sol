// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IMosaicNativeSwapper {
    event SwappedToNative(address _tokenIn, uint256 amountIn, uint256 amountOut, address addressTo);

    function swapToNative(
        address _tokenIn,
        uint256 _amount,
        uint256 _minAmountOut,
        address _to,
        bytes calldata _data
    ) external returns (uint256);
}
