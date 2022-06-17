// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/OwnableUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";

import "../interfaces/IMosaicHolding.sol";
import "../interfaces/ITokenFactory.sol";
import "../interfaces/IReceiptBase.sol";
import "../interfaces/IVaultConfigBase.sol";

abstract contract VaultConfigBase is
    IVaultConfigBase,
    OwnableUpgradeable,
    ReentrancyGuardUpgradeable
{
    ITokenFactory internal tokenFactory;
    IMosaicHolding public mosaicHolding;

    /// @notice Address of the MosaicVault
    address public vault;

    /// @notice Get mosaicHolding address
    function getMosaicHolding() external view override returns (address) {
        return address(mosaicHolding);
    }

    /// @notice Used to set address of the MosaicVault
    /// @param _vault address of the MosaicVault
    function setVault(address _vault) external override validAddress(_vault) onlyOwner {
        vault = _vault;
    }

    /// @notice External function used to set the Token Factory Address
    /// @dev Address of the factory need to be set after the initialization in order to use the vault
    /// @param _tokenFactoryAddress Address of the already deployed Token Factory
    function setTokenFactoryAddress(address _tokenFactoryAddress)
        external
        override
        onlyOwner
        validAddress(_tokenFactoryAddress)
    {
        tokenFactory = ITokenFactory(_tokenFactoryAddress);
    }

    modifier validAddress(address _address) {
        require(_address != address(0), "Invalid address");
        _;
    }

    modifier validAmount(uint256 _value) {
        require(_value > 0, "Invalid amount");
        _;
    }
}
