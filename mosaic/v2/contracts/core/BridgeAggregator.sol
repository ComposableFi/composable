// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/token/ERC20/utils/SafeERC20Upgradeable.sol";

import "../interfaces/IBridgeAggregator.sol";
import "../interfaces/IBridgeBase.sol";

/// @title BridgeAggregator
/// @notice Mosaic contract responsible with multiple bridge logic
contract BridgeAggregator is OwnableUpgradeable, IBridgeAggregator {
    using SafeERC20Upgradeable for IERC20Upgradeable;

    /// @notice Address of MosaicVault
    address public vaultAddress;
    /// @notice Address of MosaicHolding
    address public mosaicHolding;

    /// @notice Public mapping that keep track of the supported bridges
    /// @dev The bridges should extend IBridgeBase interface
    ///      destinationNetwork => bridge Id = bridge address
    mapping(uint256 => mapping(uint256 => address)) public supportedBridges;

    /// @notice Initialize function to set up the contract
    /// @dev it should be called immediately after deploy
    /// @param _mosaicHolding Address of the MosaicHolding contract
    function initialize(address _mosaicHolding) public initializer {
        __Ownable_init();
        mosaicHolding = _mosaicHolding;
    }

    /// @notice external function used by the owner to set the MosaicVault address
    /// @param _vaultAddress Address of the vault. Should not be equal to address(0)
    function setVault(address _vaultAddress) external onlyOwner validAddress(_vaultAddress) {
        vaultAddress = _vaultAddress;
    }

    /// @notice External function called by owner to add bridge
    /// @param destinationNetwork Chain ID of the destination network
    /// @param bridgeID ID of the bridge
    /// @param bridgeAddress Address of the bridge
    function addBridge(
        uint256 destinationNetwork,
        uint256 bridgeID,
        address bridgeAddress
    ) external override onlyOwner validAddress(bridgeAddress) {
        require(
            supportedBridges[destinationNetwork][bridgeID] == address(0),
            "Bridge already exist"
        );
        supportedBridges[destinationNetwork][bridgeID] = bridgeAddress;
    }

    /// @notice External function called by admin to remove supported bridge
    /// @param destinationNetwork Chain ID of the destination network
    /// @param bridgeID Id of the bridge to remove
    /// @dev destinationNetwork is used to identify the bridge
    function removeBridge(uint256 destinationNetwork, uint256 bridgeID) external onlyOwner {
        delete supportedBridges[destinationNetwork][bridgeID];
    }

    /// @notice External function called only by the address of the vault to bridge token to L2
    /// @param destinationNetwork chain id of the destination network
    /// @param receiver Address of the receiver on the L2 network
    /// @param token Address of the ERC20 token
    /// @param amount Amount need to be send
    /// @param _data Additional data that different bridge required in order to mint token
    function bridgeTokens(
        uint256 destinationNetwork,
        uint256 bridgeId,
        address receiver,
        address token,
        uint256 amount,
        bytes calldata _data
    ) external override onlyVault validAmount(amount) {
        address _bridgeAddress = supportedBridges[destinationNetwork][bridgeId];
        require(_bridgeAddress != address(0), "Invalid bridge id");
        IERC20Upgradeable(token).safeTransferFrom(mosaicHolding, address(this), amount);
        IERC20Upgradeable(token).safeApprove(_bridgeAddress, amount);
        IBridgeBase(_bridgeAddress).depositERC20ForAddress(amount, token, _data, receiver);
        emit AssetSend(receiver, token, amount, destinationNetwork);
    }

    modifier validAmount(uint256 _value) {
        require(_value > 0, "Invalid amount");
        _;
    }

    modifier validAddress(address _address) {
        require(_address != address(0), "Invalid address");
        _;
    }

    modifier onlyVault() {
        require(vaultAddress != address(0), "Vault address not set");
        require(msg.sender == vaultAddress, "Permissions: Only vault allowed");
        _;
    }
}
