// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;
import "./ISwap.sol";

interface IStableSwap is ISwap {
    // solhint-disable-next-line func-name-mixedcase
    function get_dy(
        int128 i,
        int128 j,
        uint256 dx
    ) external view returns (uint256);

    function exchange(
        int128 i,
        int128 j,
        uint256 dx,
        // solhint-disable-next-line var-name-mixedcase
        uint256 min_dy
    ) external;
}
