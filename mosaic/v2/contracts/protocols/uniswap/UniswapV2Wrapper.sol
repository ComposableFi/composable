// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../../interfaces/IMosaicExchange.sol";
import "../uniswap/IUniswapV2Router02.sol";

contract UniswapV2Wrapper is IMosaicExchange, OwnableUpgradeable {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    IUniswapV2Router02 public swapRouter;

    function initialize(address _swapRouterAddress) public initializer {
        __Ownable_init();
        swapRouter = IUniswapV2Router02(_swapRouterAddress);
    }

    function swap(
        address _tokenIn,
        address _tokenOut,
        uint256 _amount,
        uint256 _amountOutMin,
        bytes calldata _data
    ) external override returns (uint256) {
        address[] memory path = new address[](2);
        path[0] = _tokenIn;
        path[1] = _tokenOut;

        uint256 deadline;

        if (_data.length != 0) {
            (deadline) = abi.decode(_data, (uint256));
        } else {
            deadline = block.timestamp;
        }

        IERC20Upgradeable(_tokenIn).safeTransferFrom(msg.sender, address(this), _amount);
        IERC20Upgradeable(_tokenIn).safeIncreaseAllowance(address(swapRouter), _amount);

        uint256[] memory amounts = swapRouter.swapExactTokensForTokens(
            _amount,
            _amountOutMin,
            path,
            msg.sender,
            deadline
        );
        return amounts[1];
    }

    function getAmountsOut(
        address tokenIn,
        address tokenOut,
        uint256 amountIn,
        bytes calldata
    ) external view override returns (uint256) {
        address[] memory path = new address[](2);
        path[0] = tokenIn;
        path[1] = tokenOut;
        uint256[] memory amounts = swapRouter.getAmountsOut(amountIn, path);
        return amounts[1];
    }
}
