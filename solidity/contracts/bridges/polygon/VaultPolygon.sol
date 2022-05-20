// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./IRootChainManager.sol";
import "../BridgeBase.sol";

contract VaultPolygon is BridgeBase {
    IRootChainManager public rootChainManager;

    constructor(address _rootChainManager) {
        rootChainManager = IRootChainManager(_rootChainManager);
    }

    function setRootChainManager(address _router) external onlyAdmin {
        require(_router != address(0), "Invalid address");
        rootChainManager = IRootChainManager(_router);
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory,
        address destination
    ) internal override {
        address predicateAddress = rootChainManager.typeToPredicate(
            rootChainManager.tokenToType(token)
        );
        SafeERC20.safeIncreaseAllowance(IERC20(token), predicateAddress, amount);
        bytes memory encodedAmount = abi.encode(amount);
        rootChainManager.depositFor(destination, token, encodedAmount);
    }
}
