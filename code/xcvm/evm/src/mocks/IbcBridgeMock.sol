// SPDX-License-Identifier: Apache-2.0
pragma solidity ^0.8.9;
import "../interfaces/IRouter.sol";

contract IBCBridgeMock {
    function runProgram(
        address router,         
        IRouter.Origin memory origin,
        bytes memory program,
        address[] memory _assets,
        uint256[] memory _amounts)
    public {
        IRouter(router).runProgram(origin, program, _assets, _amounts);
    } 

    function sendProgram(
        bytes memory account,
        uint32 networkId,
        bytes memory salt,
        bytes memory spawnedProgram,
        uint128[] memory assetIds,
        uint256[] memory amounts
    ) external {
    }
}