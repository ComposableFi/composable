// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../../interfaces/IMosaicExchange.sol";
import "./ISynapseSwap.sol";

contract SynapseSwap is IMosaicExchange, OwnableUpgradeable {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    ISynapseSwap public swapRouter;

    function initialize(address swapRouterAddress) public initializer {
        swapRouter = ISynapseSwap(swapRouterAddress);
    }

    function swap(
        address tokenIn,
        address tokenOut,
        uint256 amount,
        uint256 amountOutMin,
        bytes calldata data
    ) external override returns (uint256) {
        uint8 tokenIndexFrom = swapRouter.getTokenIndex(tokenIn);
        uint8 tokenIndexTo = swapRouter.getTokenIndex(tokenOut);
        uint256 deadline = abi.decode(data, (uint256));

        IERC20Upgradeable(tokenIn).safeTransferFrom(msg.sender, address(this), amount);
        IERC20Upgradeable(tokenIn).safeIncreaseAllowance(address(swapRouter), amount);

        uint256 result = swapRouter.swap(
            tokenIndexFrom,
            tokenIndexTo,
            amount,
            amountOutMin,
            deadline
        );
        IERC20Upgradeable(tokenOut).safeTransfer(msg.sender, result);
        return result;
    }

    function getAmountsOut(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        bytes calldata
    ) external view override returns (uint256) {
        uint8 tokenIndexFrom = swapRouter.getTokenIndex(tokenIn);
        uint8 tokenIndexTo = swapRouter.getTokenIndex(tokenOut);
        return swapRouter.calculateSwap(tokenIndexFrom, tokenIndexTo, amountIn);
    }
}
