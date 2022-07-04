// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../../interfaces/IMosaicExchange.sol";
import "./ISwapRouter.sol";
import "./IQuoter.sol";

// @title UniswapWrapper
// @notice Uniswap V3
contract UniswapWrapper is IMosaicExchange, OwnableUpgradeable {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    ISwapRouter public swapRouter;
    IQuoter public quoter;

    function initialize(address swapRouterAddress, address quoterAddress) public initializer {
        swapRouter = ISwapRouter(swapRouterAddress);
        quoter = IQuoter(quoterAddress);
    }

    function swap(
        address tokenIn,
        address tokenOut,
        uint256 amount,
        uint256 amountOutMin,
        bytes calldata data
    ) external override returns (uint256) {
        IERC20Upgradeable(tokenIn).safeTransferFrom(msg.sender, address(this), amount);
        IERC20Upgradeable(tokenIn).safeApprove(address(swapRouter), amount);
        (uint256 deadline, uint160 sqrtPriceLimitX96, uint24 fee) = abi.decode(
            data,
            (uint256, uint160, uint24)
        );

        ISwapRouter.ExactInputSingleParams memory params = ISwapRouter.ExactInputSingleParams(
            tokenIn,
            tokenOut,
            fee,
            msg.sender,
            deadline,
            amount,
            amountOutMin,
            sqrtPriceLimitX96
        );
        return swapRouter.exactInputSingle(params);
    }

    function getAmountsOut(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        bytes calldata data
    ) external override returns (uint256) {
        (uint160 sqrtPriceLimitX96, uint24 fee) = abi.decode(data, (uint160, uint24));
        return quoter.quoteExactInputSingle(tokenIn, tokenOut, fee, amountIn, sqrtPriceLimitX96);
    }
}
