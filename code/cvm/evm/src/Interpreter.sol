// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;

import "protobuf3-solidity-lib/ProtobufLib.sol";
import "openzeppelin-contracts/token/ERC20/IERC20.sol";
import "openzeppelin-contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "./interfaces/IInterpreter.sol";
import "./interfaces/IRouter.sol";
import "./libraries/SDK.sol";

/**
 * @title Interpreter
 * @notice Custom interpreter
 */
contract Interpreter is IInterpreter {

    address public creator;
    address public routerAddress;
    bytes public override salt;
    mapping(address => bool) public owners;
    IRouter.Origin origin;

    modifier onlyOwnerOrCreator() {
        require(keccak256(abi.encodePacked(msg.sender)) == keccak256(origin.account) || owners[msg.sender] || address(this) == msg.sender);
        _;
    }

    constructor(IRouter.Origin memory _origin, address _routerAddress, bytes memory _salt) {
        owners[msg.sender] = true;
        creator = msg.sender;
        routerAddress = _routerAddress;
        salt = _salt;
        origin = _origin;
    }

    function addOwners(address[] memory newOwners) public onlyOwnerOrCreator {
        for(uint256 i=0; i<newOwners.length; i++) {
            require(newOwners[i] != address(0), "Interpreter: invalid address");
            owners[newOwners[i]] = true;
        }
    }

    function removeOwners(address[] memory newOwners) public onlyOwnerOrCreator {
        for(uint256 i=0; i<newOwners.length; i++) {
            require(newOwners[i] != address(0), "Interpreter: invalid address");
            owners[newOwners[i]] = false;
        }
    }

    receive() external payable {}

    /**
     * @notice encode and decode program using protobuf
     * @param program program encoded in bytes
     */
    function interpret(bytes memory program, address relayer) public onlyOwnerOrCreator {
        SDK.interpretProgram(program, relayer, routerAddress, origin);
    }
}
