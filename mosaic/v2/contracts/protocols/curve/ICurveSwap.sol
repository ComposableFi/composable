// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;
import "./ISwap.sol";

interface ICurveSwap is ISwap {
    // solhint-disable-next-line func-name-mixedcase
    function get_dy(
        uint256 i,
        uint256 j,
        uint256 dx
    ) external view returns (uint256);

    function exchange(
        uint256 i,
        uint256 j,
        uint256 dx,
        // solhint-disable-next-line var-name-mixedcase
        uint256 min_dy,
        // solhint-disable-next-line var-name-mixedcase
        bool use_eth
    ) external payable;
}
