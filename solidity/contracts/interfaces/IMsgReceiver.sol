// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

interface IMsgReceiver {
    function forwardCall() external;
}
