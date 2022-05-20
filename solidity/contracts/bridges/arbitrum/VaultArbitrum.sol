// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "../BridgeBase.sol";

import "./IGatewayRouter.sol";

contract VaultArbitrum is BridgeBase {
    IGatewayRouter public gatewayRouter;

    constructor(address _router) {
        require(_router != address(0), "Invalid address");
        gatewayRouter = IGatewayRouter(_router);
    }

    function setGateWayRouter(address _router) external onlyAdmin {
        require(_router != address(0), "Invalid address");
        gatewayRouter = IGatewayRouter(_router);
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory data,
        address destination
    ) internal override {
        SafeERC20.safeIncreaseAllowance(IERC20(token), address(gatewayRouter), amount);
        (uint256 maxGas, uint256 gasPriceBid, bytes memory _data) = abi.decode(
            data,
            (uint256, uint256, bytes)
        );
        gatewayRouter.outboundTransfer{value: msg.value}(
            token,
            destination,
            amount,
            maxGas,
            gasPriceBid,
            _data
        );
    }
}
