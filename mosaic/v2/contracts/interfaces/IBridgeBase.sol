// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IBridgeBase {
    event Deposit(address indexed account, address indexed erc20, uint256 value, uint256 feeValue);
    event TokenAdded(address indexed erc20);
    event TokenRemoved(address indexed erc20);
    event FeeChanged(uint256 fee);
    event WithdrawalCompleted(address accountTo, uint256 amount, address tokenAddress);

    function addWhitelistedToken(address _tokenAddress) external;

    function setFee(uint256 _newFee) external;

    function removeWhitelistedToken(address _tokenAddress) external;

    function depositERC20(
        uint256 _amount,
        address _tokenAddress,
        bytes calldata _data
    ) external payable;

    function depositERC20ForAddress(
        uint256 _amount,
        address _tokenAddress,
        bytes calldata _data,
        address _destination
    ) external payable;
}
