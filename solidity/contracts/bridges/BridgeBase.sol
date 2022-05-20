// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "@openzeppelin/contracts/access/AccessControlEnumerable.sol";
import "@openzeppelin/contracts/token/ERC20/utils/SafeERC20.sol";
import "@openzeppelin/contracts/security/ReentrancyGuard.sol";

import "../interfaces/IBridgeBase.sol";
import "../libraries/FeeOperations.sol";

abstract contract BridgeBase is IBridgeBase, AccessControlEnumerable, ReentrancyGuard {
    using SafeERC20 for IERC20;

    bytes32 public constant FEES_EXCLUDED = keccak256("FEES_EXCLUDED");
    bytes32 public constant FEES_COLLECTOR = keccak256("FEES_COLLECTOR");

    uint256 private fee;

    mapping(address => bool) public whitelistedTokens;
    mapping(address => mapping(address => uint256)) private balances;

    constructor() {
        fee = 0;
        _setInitialRoles();
    }

    modifier onlyWhitelistedToken(address tokenAddress) {
        require(whitelistedTokens[tokenAddress], "token not whitelisted");
        _;
    }

    function addWhitelistedToken(address tokenAddress) external override onlyAdmin {
        require(tokenAddress != address(0), "Invalid token address");
        whitelistedTokens[tokenAddress] = true;
        emit TokenAdded(tokenAddress);
    }

    function setFee(uint256 newFee) external override onlyAdmin {
        fee = newFee;
        emit FeeChanged(fee);
    }

    function removeWhitelistedToken(address tokenAddress) external override onlyAdmin {
        require(tokenAddress != address(0), "Invalid token address");
        delete whitelistedTokens[tokenAddress];
        emit TokenRemoved(tokenAddress);
    }

    /**
     * @notice Deposits ERC20 token into vault and initiate L2 implementation specific transfer
     * @param amount Token amount
     * @param tokenAddress Token address on L2
     */
    function depositERC20(
        uint256 amount,
        address tokenAddress,
        bytes calldata data
    ) external payable override onlyWhitelistedToken(tokenAddress) {
        _depositERC20(amount, tokenAddress, data, msg.sender);
    }

    /**
     * @notice Deposits ERC20 token into vault and initiate L2 implementation specific transfer for custom destination address
     * @param amount Token amount
     * @param tokenAddress Token address on L1
     * @param destination Destination of the token on L2
     */
    function depositERC20ForAddress(
        uint256 amount,
        address tokenAddress,
        bytes calldata data,
        address destination
    ) external payable override onlyWhitelistedToken(tokenAddress) {
        _depositERC20(amount, tokenAddress, data, destination);
    }

    function _depositERC20(
        uint256 amount,
        address tokenAddress,
        bytes memory data,
        address destination
    ) private {
        require(amount != 0, "Amount cannot be zero");
        uint256 feeAbsolute = 0;
        if (!hasRole(FEES_EXCLUDED, msg.sender)) {
            feeAbsolute = FeeOperations.getFeeAbsolute(amount, fee);
            address feesCollector = getRoleMember(FEES_COLLECTOR, 0);
            IERC20(tokenAddress).safeTransferFrom(msg.sender, feesCollector, feeAbsolute);

            amount = amount - feeAbsolute;
        }

        IERC20(tokenAddress).safeTransferFrom(msg.sender, address(this), amount);

        _transferL2Implementation(amount, tokenAddress, data, destination);
        balances[tokenAddress][msg.sender] = balances[tokenAddress][msg.sender] + amount;
        emit Deposit(msg.sender, tokenAddress, amount, feeAbsolute);
    }

    function _transferL2Implementation(
        uint256 amount,
        address tokenAddress,
        bytes memory data,
        address destination
    ) internal virtual;

    function withdrawTo(
        address accountTo,
        uint256 amount,
        address tokenAddress
    ) internal nonReentrant {
        IERC20 token = IERC20(tokenAddress);
        uint256 balance = balances[tokenAddress][accountTo];
        require(balance >= amount, "Not enough tokens on balance");
        require(token.balanceOf(address(this)) >= amount, "Not enough tokens on balance");
        balances[tokenAddress][accountTo] = balance - amount;
        token.safeTransfer(accountTo, amount);
        emit WithdrawalCompleted(accountTo, amount, tokenAddress);
    }

    /// @dev Initial function used to set the initial roles
    function _setInitialRoles() private {
        _setupRole(DEFAULT_ADMIN_ROLE, _msgSender());
        _setupRole(FEES_EXCLUDED, _msgSender());
        _setupRole(FEES_COLLECTOR, _msgSender());
        _setRoleAdmin(FEES_COLLECTOR, DEFAULT_ADMIN_ROLE);
        _setRoleAdmin(FEES_EXCLUDED, DEFAULT_ADMIN_ROLE);
    }

    modifier onlyAdmin() {
        require(hasRole(DEFAULT_ADMIN_ROLE, _msgSender()), "Permissions: Only admins allowed");
        _;
    }
}
