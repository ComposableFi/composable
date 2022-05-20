// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./IForeignOmnibridge.sol";
import "../BridgeBase.sol";

contract VaultXdai is BridgeBase {
    IForeignOmnibridge public xDaiOmniBridgeAddress;

    constructor(address _xDaiOmniBridgeAddress) {
        xDaiOmniBridgeAddress = IForeignOmnibridge(_xDaiOmniBridgeAddress);
    }

    function setDaiOmniBridgeAddress(address _router) external onlyAdmin {
        require(_router != address(0), "Invalid address");
        xDaiOmniBridgeAddress = IForeignOmnibridge(_router);
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory,
        address destination
    ) internal override {
        require(
            xDaiOmniBridgeAddress.withinLimit(token, amount),
            "Amount exceeds limit of the bridge"
        );

        SafeERC20.safeIncreaseAllowance(IERC20(token), address(xDaiOmniBridgeAddress), amount);
        xDaiOmniBridgeAddress.relayTokens(IERC677(token), destination, amount);
    }
}
