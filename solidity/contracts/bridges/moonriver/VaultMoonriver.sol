// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "../BridgeBase.sol";
import "./IMoonriverBridge.sol";

contract VaultMoonriver is BridgeBase {
    IMoonriverBridge public moonriverBridge;

    constructor(address _bridge) {
        require(_bridge != address(0), "Invalid address");
        moonriverBridge = IMoonriverBridge(_bridge);
    }

    function setBridge(address _bridge) external onlyAdmin {
        require(_bridge != address(0), "Invalid address");
        moonriverBridge = IMoonriverBridge(_bridge);
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory data,
        address destination
    ) internal override {
        SafeERC20.safeIncreaseAllowance(IERC20(token), address(moonriverBridge), amount);
        uint256 chainId = abi.decode(data, (uint256));
        moonriverBridge.sendERC20SToken(chainId, destination, amount);
    }
}
