// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "../BridgeBase.sol";

import "./IRainbowBridge.sol";

contract VaultNear is BridgeBase {
    IRainbowBridge public rainbowBridge;

    constructor(address _rainbow) {
        require(_rainbow != address(0), "Invalid address");
        rainbowBridge = IRainbowBridge(_rainbow);
    }

    function setRainbowBridge(address _rainbow) external onlyAdmin {
        require(_rainbow != address(0), "Invalid address");
        rainbowBridge = IRainbowBridge(_rainbow);
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory data,
        address
    ) internal override {
        SafeERC20.safeIncreaseAllowance(IERC20(token), address(rainbowBridge), amount);
        string memory accountId = abi.decode(data, (string));
        rainbowBridge.lockToken(token, amount, accountId);
    }
}
