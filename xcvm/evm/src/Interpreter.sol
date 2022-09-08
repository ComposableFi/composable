// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;

import "@lazyledger/protobuf3-solidity-lib/contracts/ProtobufLib.sol";
import "./interfaces/IInterpreter.sol";

/**
 * @title Interpreter
 * @notice Custom interpreter
 */
contract Interpreter is IInterpreter{
    enum OPERATION {
        NONE,
        TRANSFER,
        SPAWN,
        QUERY
    }
    bytes public owner;
    address public creator;
    address public gatewayAddress;

    modifier onlyOwnerOrCreator() {
        require(
            keccak256(abi.encodePacked(msg.sender)) ==
                keccak256(owner) ||
                msg.sender == creator
        );
        _;
    }

    constructor(bytes memory _owner, address _gatewayAddress) {
        owner = _owner;
        creator = msg.sender;
        gatewayAddress = _gatewayAddress;
    }

    receive() external payable {}
    
    /**
     * @notice encode and decode program using protobuf
     * @param program program encoded in bytes
     */
    function interpret(bytes calldata program)
        public
        onlyOwnerOrCreator
    {
        (
            bool success,
            uint64 pos,
            uint64 field,
            ProtobufLib.WireType _type
        ) = ProtobufLib.decode_key(0, program);
        require(success, "decode key failed");
        require(field == 1, "should be Program");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "should be Program"
        );

        uint64 size;
        uint32 val;

        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        require(success, "decode embedded message failed");
        uint64 totalLength = pos + size;
        while (pos < totalLength) {
            (success, pos, field, _type) = ProtobufLib.decode_key(
                pos,
                program
            );
            require(field == 1, "not instruction");
            require(success, "decode key failed");

            (success, pos, size) = ProtobufLib
                .decode_embedded_message(pos, program);
            require(success, "decode embedded message failed");
            uint64 instruction;
            (success, pos, instruction, _type) = ProtobufLib
                .decode_key(pos, program);
            require(success, "decode key failed");

            (success, pos, size) = ProtobufLib
                .decode_embedded_message(pos, program);
            require(success, "decode embedded message failed");

            if (instruction == uint64(OPERATION.TRANSFER)) {
                (success, pos, field, _type) = ProtobufLib.decode_key(
                    pos,
                    program
                );
                (success, pos, val) = ProtobufLib.decode_uint32(
                    pos,
                    program
                );
                require(success, "decode key failed");
            } else if (instruction == uint64(OPERATION.SPAWN)) {
                (success, pos, field, _type) = ProtobufLib.decode_key(
                    pos,
                    program
                );
                (success, pos, val) = ProtobufLib.decode_uint32(
                    pos,
                    program
                );
                require(success, "decode key failed");
                for (uint8 j = 0; j < val; j++) {}
            } else if (instruction == uint8(OPERATION.QUERY)) {}
        }
    }

    function createProgram() public returns (bytes memory program){

    }
}
