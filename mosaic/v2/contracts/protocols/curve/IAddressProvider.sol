// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IAddressProvider {
    // Fetch the address associated with `_id`
    // solhint-disable-next-line func-name-mixedcase
    function get_address(uint256 _id) external view returns (address contractAddress);
}
