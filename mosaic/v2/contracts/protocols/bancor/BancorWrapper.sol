// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../../interfaces/IMosaicExchange.sol";
import "./IBancorNetwork.sol";

contract BancorSwap is IMosaicExchange, OwnableUpgradeable {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    IBancorNetwork public swapRouter;

    function initialize(address swapRouterAddress) public initializer {
        swapRouter = IBancorNetwork(swapRouterAddress);
    }

    function swap(
        address tokenIn,
        address tokenOut,
        uint256 amount,
        uint256 amountOutMin,
        bytes calldata
    ) external override returns (uint256) {
        address[] memory path = swapRouter.conversionPath(IERC20(tokenIn), IERC20(tokenOut));

        IERC20Upgradeable(tokenIn).safeTransferFrom(msg.sender, address(this), amount);
        IERC20Upgradeable(tokenIn).safeIncreaseAllowance(address(swapRouter), amount);

        // If the beneficiary should be the sender of the transaction, use 0x0
        // If no affiliate fee should be paid out, use 0x0
        // the affiliate fee in parts per million. 30000 or 0.03% is the maximum allowed
        uint256 result = swapRouter.convertByPath(
            path,
            amount,
            amountOutMin,
            address(0),
            address(0),
            0
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
        address[] memory path = swapRouter.conversionPath(IERC20(tokenIn), IERC20(tokenOut));
        return swapRouter.rateByPath(path, amountIn);
    }
}
