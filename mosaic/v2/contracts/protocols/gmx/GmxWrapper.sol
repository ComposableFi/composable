// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../../interfaces/IMosaicExchange.sol";
import "./IGmxRouter.sol";
import "./IGmxReader.sol";

contract GmxSwap is IMosaicExchange, OwnableUpgradeable {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    IGmxRouter public swapRouter;

    function initialize(address swapRouterAddress) public initializer {
        swapRouter = IGmxRouter(swapRouterAddress);
    }

    function swap(
        address tokenIn,
        address tokenOut,
        uint256 amount,
        uint256 amountOutMin,
        bytes calldata
    ) external override returns (uint256) {
        uint256 balanceBefore = IERC20Upgradeable(tokenOut).balanceOf(address(this));

        address[] memory path = new address[](2);
        path[0] = tokenIn;
        path[1] = tokenOut;

        IERC20Upgradeable(tokenIn).safeTransferFrom(msg.sender, address(this), amount);
        IERC20Upgradeable(tokenIn).safeIncreaseAllowance(address(swapRouter), amount);

        swapRouter.swap(path, amount, amountOutMin, address(this));

        uint256 balanceAfter = IERC20Upgradeable(tokenOut).balanceOf(address(this));
        require(balanceAfter > balanceBefore, "ERR: SWAP FAILED");

        IERC20Upgradeable(tokenOut).safeTransfer(msg.sender, balanceAfter - balanceBefore);
        return balanceAfter - balanceBefore;
    }

    function getAmountsOut(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        bytes calldata data
    ) external view override returns (uint256) {
        (address vaultAddress, address readerAddress) = abi.decode(data, (address, address));
        require(vaultAddress != address(0), "ERR: VAULT IS EMPTY");
        require(readerAddress != address(0), "ERR: READER IS EMPTY");

        //amount after fees, fee
        (uint256 amountOutAfterFees, ) = IGmxReader(readerAddress).getAmountOut(
            IGmxVault(vaultAddress),
            tokenIn,
            tokenOut,
            amountIn
        );

        return amountOutAfterFees;
    }
}
