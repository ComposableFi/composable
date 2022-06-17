// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IWETH {
    function deposit() external payable;

    function withdraw(uint256 _wad) external;

    function balanceOf(address _account) external view returns (uint256);

    function name() external view returns (string memory);

    function symbol() external view returns (string memory);
}
