// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts-upgradeable/access/AccessControlUpgradeable.sol";
import "@openzeppelin/contracts-upgradeable/security/ReentrancyGuardUpgradeable.sol";

import "../interfaces/IInvestmentStrategy.sol";

/// @title InvestmentStrategyBase
/// @notice Abstract contract that have common functionalities for the investment strategies
abstract contract InvestmentStrategyBase is
    IInvestmentStrategy,
    AccessControlUpgradeable,
    ReentrancyGuardUpgradeable
{
    /// @notice Role name for the MosaicHolding
    bytes32 public constant MOSAIC_HOLDING = keccak256("MOSAIC_HOLDING");
    /// @notice Address of the MosaicHolding
    address public mosaicHolding;

    /// @dev internal initialize function that every contract that inherit should invoke
    /// @param _admin address of the contract admin
    /// @param _investor address of the investor
    function initializeBase(address _admin, address _investor)
        internal
        initializer
        validAddress(_admin)
        validAddress(_investor)
    {
        __AccessControl_init();
        __ReentrancyGuard_init();
        _setupRole(DEFAULT_ADMIN_ROLE, _admin);
        _setRoleAdmin(MOSAIC_HOLDING, DEFAULT_ADMIN_ROLE);
        _setupRole(MOSAIC_HOLDING, _investor);
        mosaicHolding = _investor;
    }

    modifier validAddress(address _address) {
        require(_address != address(0), "Invalid address");
        _;
    }

    modifier validAmount(uint256 _value) {
        require(_value > 0, "Invalid amount");
        _;
    }

    modifier onlyAdmin() {
        require(hasRole(DEFAULT_ADMIN_ROLE, _msgSender()), "Permissions: Only admins allowed");
        _;
    }

    modifier onlyInvestor() {
        require(hasRole(MOSAIC_HOLDING, _msgSender()), "Permissions: Only investor allowed");
        _;
    }

    modifier oneTokenAllowed(Investment[] calldata investments) {
        require(investments.length == 1, "Only one token allowed");
        _;
    }
}
