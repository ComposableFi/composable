// SPDX-License-Identifier: UNLICENSED
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";

import "../../interfaces/IMosaicExchange.sol";
import "./V2/IVault.sol";

contract BalancerVaultV2Wrapper is IMosaicExchange, OwnableUpgradeable {
    IVault public balancerVaultV2;

    function initialize(address _vault) public initializer {
        __Ownable_init();
        balancerVaultV2 = IVault(_vault);
    }

    function swap(
        address _tokenA,
        address _tokenB,
        uint256 _amountIn,
        uint256 _amountOutMin,
        bytes calldata _data
    ) external override returns (uint256) {
        (bytes32 poolId, uint256 deadline) = abi.decode(_data, (bytes32, uint256));
        IVault.SingleSwap memory singleSwap;
        singleSwap.poolId = poolId;
        singleSwap.kind = IVault.SwapKind.GIVEN_IN;
        singleSwap.assetIn = IAsset(_tokenA);
        singleSwap.assetOut = IAsset(_tokenB);
        singleSwap.amount = _amountIn;

        IVault.FundManagement memory funds;
        funds.recipient = payable(msg.sender);
        funds.sender = address(this);

        IERC20(_tokenA).transferFrom(msg.sender, address(this), _amountIn);
        IERC20(_tokenA).approve(address(balancerVaultV2), _amountIn);

        uint256 amountOut = balancerVaultV2.swap(singleSwap, funds, _amountOutMin, deadline);
        return amountOut;
    }

    function getAmountsOut(
        address,
        address,
        uint256,
        bytes calldata
    ) external pure override returns (uint256) {
        revert("ERR: NOT IMPLEMENTED");
    }
}
