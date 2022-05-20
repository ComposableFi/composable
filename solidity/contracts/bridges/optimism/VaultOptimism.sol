// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./iOVM_L1ERC20Bridge.sol";
import "../BridgeBase.sol";

contract VaultOptimism is BridgeBase {
    address l1ERC20Bridge;

    constructor(address _l1ERC20Bridge) {
        l1ERC20Bridge = _l1ERC20Bridge;
    }

    function setL1ERC20BridgeAddress(address _l1ERC20Bridge) external onlyAdmin {
        l1ERC20Bridge = _l1ERC20Bridge;
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory data,
        address destination
    ) internal override {
        // approve the tokens for transfer
        SafeERC20.safeIncreaseAllowance(IERC20(token), l1ERC20Bridge, amount);

        // get the address of the token on l2
        (address l2TokenAddress, uint32 gasLimit) = abi.decode(data, (address, uint32));

        iOVM_L1ERC20Bridge(l1ERC20Bridge).depositERC20To(
            token,
            l2TokenAddress,
            destination,
            amount,
            gasLimit,
            ""
        );
    }
}
