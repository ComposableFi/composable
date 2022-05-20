// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface ILiquidityPoolManager {
    function depositErc20(
        address tokenAddress,
        address receiver,
        uint256 amount,
        uint256 toChainId
    ) external;
}
