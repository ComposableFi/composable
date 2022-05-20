// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./IBridgeRouter.sol";
import "../BridgeBase.sol";

contract VaultCelo is BridgeBase {
    address router;

    constructor(address _router) {
        router = _router;
    }

    function setBridgeRouterAddress(address _router) external onlyAdmin {
        require(_router != address(0), "Invalid address");
        router = _router;
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory,
        address destination
    ) internal override {
        // approve the tokens for transfer
        SafeERC20.safeIncreaseAllowance(IERC20(token), router, amount);

        // convert to bytes32
        bytes32 _destination = bytes32(uint256(uint160(destination)));

        // 1667591279 value hardcoded for celo
        IBridgeRouter(router).send(token, amount, 1667591279, _destination);
    }
}
