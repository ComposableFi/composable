// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./IL1_ERC20_Bridge.sol";
import "../BridgeBase.sol";

contract VaultHop is BridgeBase {
    address public relayer;
    uint256 public relayerFee;
    uint256 public deadline = 1 weeks;

    mapping(address => address) public ERC20_BRIDGE_ADDRESS;

    function setRelayerAddress(address _relayer) external onlyAdmin {
        relayer = _relayer;
    }

    function setRelayerFee(uint256 _relayerFee) external onlyAdmin {
        relayerFee = _relayerFee;
    }

    function addBridgeAddress(address tokenAddress, address bridgeAddress) external onlyAdmin {
        require(
            ERC20_BRIDGE_ADDRESS[tokenAddress] == address(0),
            "bridge address already assigned"
        );
        ERC20_BRIDGE_ADDRESS[tokenAddress] = bridgeAddress;
    }

    function changeBridgeAddress(address tokenAddress, address newBridgeAddress)
        external
        onlyAdmin
    {
        require(
            ERC20_BRIDGE_ADDRESS[tokenAddress] != address(0),
            "bridge address not assigned yet"
        );
        ERC20_BRIDGE_ADDRESS[tokenAddress] = newBridgeAddress;
    }

    function deleteBridgeAddress(address tokenAddress) external onlyAdmin {
        require(
            ERC20_BRIDGE_ADDRESS[tokenAddress] != address(0),
            "bridge address not assigned yet"
        );
        delete ERC20_BRIDGE_ADDRESS[tokenAddress];
    }

    function _transferL2Implementation(
        uint256 amount,
        address token,
        bytes memory data,
        address destination
    ) internal override {
        require(ERC20_BRIDGE_ADDRESS[token] != address(0), "token not supported on Hop");
        address bridgeAddress = ERC20_BRIDGE_ADDRESS[token];

        SafeERC20.safeIncreaseAllowance(IERC20(token), bridgeAddress, amount);

        (uint256 chainId, uint256 amountOutMin) = abi.decode(data, (uint256, uint256));

        IL1_ERC20_Bridge(bridgeAddress).sendToL2(
            chainId,
            destination,
            amount,
            amountOutMin,
            deadline,
            relayer,
            relayerFee
        );
    }
}
