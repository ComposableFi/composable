// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

import "./EIP20NonStandardInterface.sol";

interface CErc20Interface is EIP20NonStandardInterface {
    function underlying() external view returns (address);

    function mint(uint256) external returns (uint256);

    function exchangeRateCurrent() external returns (uint256);

    function exchangeRateStored() external view returns (uint256);

    function supplyRatePerBlock() external returns (uint256);

    function borrowRatePerBlock() external returns (uint256);

    function redeem(uint256) external returns (uint256);

    function redeemUnderlying(uint256) external returns (uint256);

    function balanceOfUnderlying(address) external returns (uint256);
}
