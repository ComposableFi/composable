// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;

import "@lazyledger/protobuf3-solidity-lib/contracts/ProtobufLib.sol";

/**
 * @title Interpreter
 * @notice Custom interpreter
 */
contract Interpreter {
    enum OPERATION {NONE, PUSH, POP, ADD, SUB, MUL}
    uint8[] public userStack;
    address public owner;
    address public creator;

    modifier onlyOwnerOrCreator {
        require(msg.sender == owner || msg.sender == creator);
        _;
    }

    constructor(address _owner) {
        owner = _owner;
        creator = msg.sender;
    }

    receive() external payable {}
    
    /**
     * @notice pop value from stack and return it
     * due to array.pop from solidity doesn't return the popped value
     */
    function _popAndGet() private returns (uint8 value) {
        value = userStack[userStack.length - 1];
        userStack.pop();
    }

    /**
     * @notice assuming that all operation parameters are uint8, each instruction is 2bytes which contains [operation|param]
     * @param instructions uint16[]
     */
    function interpret(uint16[] calldata instructions) public onlyOwnerOrCreator {
        for (uint256 i = 0; i < instructions.length; i++) {
            uint16 step = instructions[i];
            uint8 opcode = uint8(step >> 8);
            // get first byte
            uint8 arg = uint8(step);
            // get the last byte
            if (opcode == uint8(OPERATION.PUSH)) {
                userStack.push(arg);
            } else if (opcode == uint8(OPERATION.POP)) {
                require(userStack.length >= arg, "stack length is not enough");
                for (uint8 j = 0; j < arg; j++) {
                    userStack.pop();
                }
            } else if (opcode == uint8(OPERATION.ADD)) {
                uint8 x = _popAndGet();
                uint8 y = _popAndGet();
                userStack.push(x + y);
            } else if (opcode == uint8(OPERATION.SUB)) {
                uint8 x = _popAndGet();
                uint8 y = _popAndGet();
                userStack.push(x - y);
            } else if (opcode == uint8(OPERATION.MUL)) {
                uint8 x = _popAndGet();
                uint8 y = _popAndGet();
                userStack.push(x * y);
            }
        }
    }

    /**
     * @notice encode and decode program using protobuf
     * @param program program encoded in bytes
     */
    function interpretWithProtoBuff(bytes calldata program) public onlyOwnerOrCreator {
        (
            bool success,
            uint64 pos,
            uint64 field,
            ProtobufLib.WireType _type
        ) = ProtobufLib.decode_key(0, program);
        require(success, "decode key failed");
        require(field == 1, "should be Program");
        require(_type == ProtobufLib.WireType.LengthDelimited, "should be Program");


        uint64 size;
        uint32 val;

        (
            success,
            pos,
            size
        ) = ProtobufLib.decode_embedded_message(pos, program);
        require(success, "decode embedded message failed");
        uint64 totalLength = pos + size;
        while (pos < totalLength) {
            (
                success,
                pos,
                field,
            _type
            ) = ProtobufLib.decode_key(pos, program);
            require(field == 1, "not instruction");
            require(success, "decode key failed");

            (
                success,
                pos,
                size
            ) = ProtobufLib.decode_embedded_message(pos, program);
            require(success, "decode embedded message failed");
            uint64 instruction;
            (
                success,
                pos,
                instruction,
                _type
            ) = ProtobufLib.decode_key(pos, program);
            require(success, "decode key failed");

            (
                success,
                pos,
                size
            ) = ProtobufLib.decode_embedded_message(pos, program);
            require(success, "decode embedded message failed");

            if (instruction == uint64(OPERATION.PUSH)) {
                (
                    success,
                    pos,
                    field,
                    _type
                ) = ProtobufLib.decode_key(pos, program);
                (
                    success,
                    pos,
                    val
                ) = ProtobufLib.decode_uint32(pos, program);
                require(success, "decode key failed");
                userStack.push(uint8(val));
            } else if (instruction == uint64(OPERATION.POP)) {
                (
                    success,
                    pos,
                    field,
                    _type
                ) = ProtobufLib.decode_key(pos, program);
                (
                    success,
                    pos,
                    val
                ) = ProtobufLib.decode_uint32(pos, program);
                require(success, "decode key failed");
                require(userStack.length >= val, "stack length is not enough");
                for (uint8 j = 0; j < val; j++) {
                    userStack.pop();
                }
            } else if (instruction == uint8(OPERATION.ADD)) {
                uint8 x = _popAndGet();
                uint8 y = _popAndGet();
                userStack.push(x + y);
            } else if (instruction == uint8(OPERATION.SUB)) {
                uint8 x = _popAndGet();
                uint8 y = _popAndGet();
                userStack.push(x - y);
            } else if (instruction == uint8(OPERATION.MUL)) {
                uint8 x = _popAndGet();
                uint8 y = _popAndGet();
                userStack.push(x * y);
            }
        }
    }
}