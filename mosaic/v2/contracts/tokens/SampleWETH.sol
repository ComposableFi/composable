// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import "./SampleTokenERC20.sol";

contract SampleWETH is SampleTokenERC20 {
    uint256 public initialSupply =
        0x0000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff;

    event Deposit(address indexed dst, uint256 wad);

    // solhint-disable-next-line no-empty-blocks
    constructor() SampleTokenERC20("Wrapped Ether", "WETH", initialSupply) {}

    receive() external payable {
        deposit();
    }

    function deposit() public payable {
        _mint(msg.sender, msg.value);
        emit Deposit(msg.sender, msg.value);
    }
}
