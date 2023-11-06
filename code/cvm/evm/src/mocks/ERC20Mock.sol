// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
pragma abicoder v2;

import "openzeppelin-contracts/token/ERC20/ERC20.sol";
import "openzeppelin-contracts/token/ERC20/extensions/draft-ERC20Permit.sol";

contract ERC20Mock is ERC20, ERC20Permit {
    constructor(
        string memory name,
        string memory symbol,
        address initialAccount,
        uint256 initialBalance
    ) payable ERC20(name, symbol) ERC20Permit(name) {
        _mint(initialAccount, initialBalance);
    }
}
