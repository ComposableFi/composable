// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../../interfaces/IMosaicExchange.sol";
import "./ICurveSwap.sol";
import "./IStableSwap.sol";

contract CurveWrapper is IMosaicExchange, OwnableUpgradeable {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    enum SwapType {
        Stable,
        Curve
    }
    struct SwapData {
        address swapAddr;
        uint256 swapType;
        int128 i;
        int128 j;
        // solhint-disable-next-line var-name-mixedcase
        uint256 i_unsigned;
        // solhint-disable-next-line var-name-mixedcase
        uint256 j_unsigned;
    }

    function initialize() public initializer {
        __Ownable_init();
    }

    function swap(
        address tokenIn,
        address tokenOut,
        uint256 amount,
        uint256 amountOutMin,
        bytes calldata data
    ) external override returns (uint256) {
        SwapData memory swapData;
        (
            swapData.swapAddr,
            swapData.swapType,
            swapData.i,
            swapData.j,
            swapData.i_unsigned,
            swapData.j_unsigned
        ) = abi.decode(data, (address, uint256, int128, int128, uint256, uint256));

        IERC20Upgradeable(tokenIn).safeTransferFrom(msg.sender, address(this), amount);
        IERC20Upgradeable(tokenIn).safeIncreaseAllowance(swapData.swapAddr, amount);

        uint256 balanceBefore = IERC20Upgradeable(tokenOut).balanceOf(address(this));
        if (swapData.swapType == 0) {
            IStableSwap stableSwapRouter = IStableSwap(swapData.swapAddr);
            require(
                stableSwapRouter.coins(swapData.i_unsigned) == tokenIn,
                "ERR: INVALID TOKENIN ADDRESS"
            );
            require(
                stableSwapRouter.coins(swapData.j_unsigned) == tokenOut,
                "ERR: INVALID TOKENOUT ADDRESS"
            );
            stableSwapRouter.exchange(swapData.i, swapData.j, amount, amountOutMin);
        } else if (swapData.swapType == 1) {
            ICurveSwap curveSwapRouter = ICurveSwap(swapData.swapAddr);
            require(
                curveSwapRouter.coins(swapData.i_unsigned) == tokenIn,
                "ERR: INVALID TOKENIN ADDRESS"
            );
            require(
                curveSwapRouter.coins(swapData.j_unsigned) == tokenOut,
                "ERR: INVALID TOKENOUT ADDRESS"
            );

            curveSwapRouter.exchange(
                swapData.i_unsigned,
                swapData.j_unsigned,
                amount,
                amountOutMin,
                false
            );
        }
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
        (
            address swapAddr,
            uint256 swapType,
            int128 i,
            int128 j,
            // solhint-disable-next-line var-name-mixedcase
            uint256 i_unsigned,
            // solhint-disable-next-line var-name-mixedcase
            uint256 j_unsigned
        ) = abi.decode(data, (address, uint256, int128, int128, uint256, uint256));

        uint256 result = 0;
        if (swapType == 0) {
            IStableSwap stableSwapRouter = IStableSwap(swapAddr);
            require(stableSwapRouter.coins(i_unsigned) == tokenIn, "ERR: INVALID TOKENIN ADDRESS");
            require(
                stableSwapRouter.coins(j_unsigned) == tokenOut,
                "ERR: INVALID TOKENOUT ADDRESS"
            );
            result = stableSwapRouter.get_dy(i, j, amountIn);
        } else if (swapType == 1) {
            ICurveSwap curveSwapRouter = ICurveSwap(swapAddr);
            require(curveSwapRouter.coins(i_unsigned) == tokenIn, "ERR: INVALID TOKENIN ADDRESS");
            require(curveSwapRouter.coins(j_unsigned) == tokenOut, "ERR: INVALID TOKENOUT ADDRESS");

            result = curveSwapRouter.get_dy(i_unsigned, j_unsigned, amountIn);
        }

        return result;
    }
}
