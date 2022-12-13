// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;

interface IInterpreter {
    function interpret(bytes calldata program, address relayer) external;

    function addOwners(address[] calldata newOwners) external;

    function removeOwners(address[] calldata newOwners) external;

    function salt() external view returns (bytes memory);

}
