// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../../interfaces/IMosaicExchange.sol";
import "./IExchangeProxy.sol";

contract BalancerV1Wrapper is IMosaicExchange, OwnableUpgradeable {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    IExchangeProxy public exchange;

    function initialize(address _exchangeAddress) public initializer {
        __Ownable_init();
        exchange = IExchangeProxy(_exchangeAddress);
    }

    function swap(
        address tokenIn,
        address tokenOut,
        uint256 amount,
        uint256 amountOutMin,
        bytes calldata data
    ) external override returns (uint256) {
        IERC20Upgradeable(tokenIn).safeTransferFrom(msg.sender, address(this), amount);
        IERC20Upgradeable(tokenIn).safeIncreaseAllowance(address(exchange), amount);

        uint256 nPools = abi.decode(data, (uint256));

        uint256 result = exchange.smartSwapExactIn(
            TokenInterface(tokenIn),
            TokenInterface(tokenOut),
            amount,
            amountOutMin,
            nPools
        );
        IERC20Upgradeable(tokenOut).safeTransfer(msg.sender, result);

        return result;
    }

    function getAmountsOut(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        bytes calldata data
    ) external view override returns (uint256) {
        uint256 nPools = abi.decode(data, (uint256));
        (, uint256 amountOut) = exchange.viewSplitExactIn(tokenIn, tokenOut, amountIn, nPools);
        return amountOut;
    }
}
