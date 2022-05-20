// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;
import "./IERC677.sol";

interface IForeignOmnibridge {
    function withinLimit(address token, uint256 amount) external view returns (bool);

    function relayTokens(
        IERC677 token,
        address receiver,
        uint256 value
    ) external;
}
