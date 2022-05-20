// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface TokenInterface {}

// https://docs.balancer.fi/v/v1/smart-contracts/exchange-proxy
// https://github.com/balancer-labs/balancer-registry/blob/master/contracts/ExchangeProxy.sol
interface IExchangeProxy {
    struct Swap {
        address pool;
        address tokenIn;
        address tokenOut;
        uint256 swapAmount; // tokenInAmount / tokenOutAmount
        uint256 limitReturnAmount; // minAmountOut / maxAmountIn
        uint256 maxPrice;
    }

    function smartSwapExactIn(
        TokenInterface tokenIn,
        TokenInterface tokenOut,
        uint256 totalAmountIn,
        uint256 minTotalAmountOut,
        uint256 nPools
    ) external payable returns (uint256 totalAmountOut);

    function viewSplitExactIn(
        address tokenIn,
        address tokenOut,
        uint256 swapAmount,
        uint256 nPools
    ) external view returns (Swap[] memory swaps, uint256 totalOutput);
}
