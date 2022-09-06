// SPDX-License-Identifier: MIT
pragma solidity ^0.8.13;
pragma abicoder v2;

import "@openzeppelin/contracts/token/ERC20/ERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/draft-ERC20Permit.sol";

// Date of creation: 2022-07-05T10:15:23.400Z

contract ERC20Mock is ERC20, ERC20Permit {

    constructor(
        string memory name,
        string memory symbol,
        address initialAccount,
        uint256 initialBalance
    ) ERC20(name, symbol) ERC20Permit(name) payable {
        _mint(initialAccount, initialBalance);
    }
}
