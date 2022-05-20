// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./ILiquidityPoolManager.sol";
import "../BridgeBase.sol";

contract VaultBiconomy is BridgeBase {
    ILiquidityPoolManager public liquidityPoolManager;

    constructor(address _liquidityPoolManagerAddress) {
        liquidityPoolManager = ILiquidityPoolManager(_liquidityPoolManagerAddress);
    }

    function setLiquidityPoolManagerAddress(address _router) external onlyAdmin {
        require(_router != address(0), "Invalid address");
        liquidityPoolManager = ILiquidityPoolManager(_router);
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory data,
        address destination
    ) internal override {
        SafeERC20.safeIncreaseAllowance(IERC20(token), address(liquidityPoolManager), amount);

        uint256 chainId = abi.decode(data, (uint256));

        liquidityPoolManager.depositErc20(token, destination, amount, chainId);
    }
}
