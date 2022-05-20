// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/proxy/Clones.sol";
import "@openzeppelin/contracts/access/AccessControlEnumerable.sol";

import "../interfaces/ITokenFactory.sol";
import "./IOU/IOUToken.sol";
import "./receipt/ReceiptToken.sol";

contract TokenFactory is ITokenFactory, AccessControlEnumerable {
    bytes32 public constant COMPOSABLE_VAULT = keccak256("COMPOSABLE_VAULT");

    event TokenCreated(address indexed underlyingAsset, address indexed iouToken, string tokenType);

    event VaultChanged(address indexed newAddress);

    constructor(address _vault, address _vaultConfig) {
        _setupRole(DEFAULT_ADMIN_ROLE, _msgSender());
        _setupRole(COMPOSABLE_VAULT, _vault);
        _setupRole(COMPOSABLE_VAULT, _vaultConfig);
        _setRoleAdmin(COMPOSABLE_VAULT, DEFAULT_ADMIN_ROLE);
    }

    /// @notice External function used by admin of the contract to set the vault address
    /// @param _vaultAddress new vault address
    function changeVaultAddress(address _vaultAddress)
        external
        validAddress(_vaultAddress)
        onlyAdmin
    {
        uint256 rolesCount = getRoleMemberCount(COMPOSABLE_VAULT);
        for (uint256 i = 0; i < rolesCount; i++) {
            address _vault = getRoleMember(COMPOSABLE_VAULT, 0);
            revokeRole(COMPOSABLE_VAULT, _vault);
        }
        grantRole(COMPOSABLE_VAULT, _vaultAddress);

        emit VaultChanged(_vaultAddress);
    }

    /// @notice External function called only by vault to create a new IOU token
    /// @param underlyingAddress Address of the ERC20 deposited token to get the info from
    /// @param tokenName Token prefix
    function createIOU(
        address underlyingAddress,
        string calldata tokenName,
        address _owner
    ) external override validAddress(underlyingAddress) onlyVault returns (address) {
        IOUToken newIou = new IOUToken(underlyingAddress, tokenName, block.chainid, _owner);

        emit TokenCreated(underlyingAddress, address(newIou), "IOU");

        return address(newIou);
    }

    /// @notice External function called only by vault to create a new Receipt token
    /// @param underlyingAddress Address of the ERC20 deposited token to get the info from
    /// @param tokenName Token prefix
    function createReceipt(
        address underlyingAddress,
        string calldata tokenName,
        address _owner
    ) external override validAddress(underlyingAddress) onlyVault returns (address) {
        //TODO: fix token name
        ReceiptToken newReceipt = new ReceiptToken(
            underlyingAddress,
            tokenName,
            block.chainid,
            _owner
        );

        emit TokenCreated(underlyingAddress, address(newReceipt), "RECEIPT");

        return address(newReceipt);
    }

    modifier onlyVault() {
        require(hasRole(COMPOSABLE_VAULT, _msgSender()), "Permissions: Only vault allowed");
        _;
    }

    modifier onlyAdmin() {
        require(hasRole(DEFAULT_ADMIN_ROLE, _msgSender()), "Permissions: Only admins allowed");
        _;
    }

    modifier validAddress(address _addr) {
        require(_addr != address(0), "Invalid address");
        _;
    }
}
