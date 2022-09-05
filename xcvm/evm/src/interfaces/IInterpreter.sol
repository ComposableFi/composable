// SPDX-License-Identifier: MIT
pragma solidity 0.8.14;

interface IInterpreter {
    /**
     * @notice assuming that all operation parameters are uint8, each instruction is 2bytes which contains [operation|param]
     * @param program uint16[]
     */
    function interpret(uint16[] calldata program) external;
}
