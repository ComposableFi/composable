// SPDX-License-Identifier: MIT

pragma solidity ^0.8.0;

interface IComptroller {
    // Claim all the COMP accrued by holder in all markets
    function claimComp(address holder) external;
}
