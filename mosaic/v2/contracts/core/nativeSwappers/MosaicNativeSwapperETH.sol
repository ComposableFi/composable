// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "../../interfaces/IMosaicNativeSwapper.sol";
import "../../protocols/uniswap/IUniswapV2Router02.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

contract MosaicNativeSwapperETH is IMosaicNativeSwapper {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    IUniswapV2Router02 public swapRouter;

    constructor(address _swapRouterAddress) {
        swapRouter = IUniswapV2Router02(_swapRouterAddress);
    }

    /// @notice swaps tokenIn into ETH
    /// @param _tokenIn address of the token to be swapped
    /// @param _amount amount of tokenIn to be swapped
    /// @param _to destination address of the swap
    /// @param _data additional data required for each AMM implementation
    function swapToNative(
        address _tokenIn,
        uint256 _amount,
        uint256 _minAmountOut,
        address _to,
        bytes calldata _data
    ) external override returns (uint256) {
        address[] memory path = new address[](2);
        path[0] = _tokenIn;
        path[1] = swapRouter.WETH();

        uint256 deadline;

        if (_data.length != 0) {
            (deadline) = abi.decode(_data, (uint256));
        } else {
            deadline = block.timestamp;
        }

        IERC20Upgradeable(_tokenIn).safeTransferFrom(msg.sender, address(this), _amount);
        IERC20Upgradeable(_tokenIn).safeIncreaseAllowance(address(swapRouter), _amount);

        uint256[] memory amounts = swapRouter.swapExactTokensForETH(
            _amount,
            _minAmountOut,
            path,
            _to,
            deadline
        );

        emit SwappedToNative(_tokenIn, _amount, amounts[1], _to);
        return amounts[1];
    }
}
