// SPDX-License-Identifier: MIT
pragma solidity 0.8.14;

interface IInterpreter {
    function interpret(bytes calldata program) external;
}
