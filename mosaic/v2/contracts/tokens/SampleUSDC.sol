// SPDX-License-Identifier: UNLICENSED

pragma solidity ^0.8.0;

import "./SampleTokenERC20.sol";

contract SampleUSDC is SampleTokenERC20 {
    // solhint-disable-next-line no-empty-blocks
    constructor(uint256 initialSupply) SampleTokenERC20("Sample USDC", "USDC", initialSupply) {}

    function decimals() public pure override returns (uint8) {
        return 6;
    }
}
