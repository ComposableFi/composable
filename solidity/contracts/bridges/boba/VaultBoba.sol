// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./IBobaERC20Bridge.sol";
import "../BridgeBase.sol";

contract VaultBoba is BridgeBase {
    IBobaERC20Bridge bobaBridge;

    constructor(address _bobaBridge) {
        require(_bobaBridge != address(0), "Invalid address");
        bobaBridge = IBobaERC20Bridge(_bobaBridge);
    }

    function setBobaBridge(address _bobaBridge) external onlyAdmin {
        require(_bobaBridge != address(0), "Invalid address");
        bobaBridge = IBobaERC20Bridge(_bobaBridge);
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory data,
        address destination
    ) internal override {
        // approve the tokens for transfer
        SafeERC20.safeIncreaseAllowance(IERC20(token), address(bobaBridge), amount);

        // get the address of the token on l2
        (address l2TokenAddress, uint32 gasLimit) = abi.decode(data, (address, uint32));

        bobaBridge.depositERC20To(token, l2TokenAddress, destination, amount, gasLimit, "");
    }
}
