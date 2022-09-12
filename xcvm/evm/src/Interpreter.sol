// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;

import "@lazyledger/protobuf3-solidity-lib/contracts/ProtobufLib.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "./interfaces/IInterpreter.sol";
import "./interfaces/IGateway.sol";
import "forge-std/console.sol";

/**
 * @title Interpreter
 * @notice Custom interpreter
 */
contract Interpreter is IInterpreter {
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

    function bytesToAddress(bytes memory bys)
        private
        pure
        returns (address addr)
    {
        assembly {
            addr := mload(add(bys, 20))
        }
    }

    function _handelAccount(bytes calldata program, uint64 pos)
        internal
        returns (address account, uint64 newPos)
    {
        // read account info
        bool success;
        uint64 size;
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading Account
        uint64 field;
        ProtobufLib.WireType _type;
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(9999, success, field, uint256(_type));
        require(field == 1, "not asset id");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );

        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(11888, success, pos, size);
        require(success, "decode embedded message failed");

        console.logBytes(program[pos:pos + size]);
        account = bytesToAddress(program[pos:pos + size]);
        console.log(account);
        newPos = pos + size;
    }

    function _handleRatio(bytes calldata program, uint64 pos)
        internal
        returns (
            uint256 nominator,
            uint256 denominator,
            uint64 newPos
        )
    {
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;

        // read ratio message body
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(42888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading ratio denominator
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(5444, success, field, uint256(_type));
        require(success, "decode key failed");
        require(field == 1, "decode key failed");
        require(
            _type == ProtobufLib.WireType.Varint,
            "decode type is not embedded messages"
        );

        (success, pos, nominator) = ProtobufLib.decode_varint(
            pos,
            program
        );

        // reading ratio nominator
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(5444, success, field, uint256(_type));
        require(success, "decode key failed");
        require(field == 2, "decode key failed");
        require(
            _type == ProtobufLib.WireType.Varint,
            "decode type is not embedded messages"
        );

        (success, newPos, denominator) = ProtobufLib.decode_varint(
            pos,
            program
        );
    }

    function _handleAbsolute(bytes calldata program, uint64 pos)
        internal
        returns (uint256 amount, uint64 newPos)
    {
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;

        // read ratio message body
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(42888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading ratio denominator
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(5444, success, field, uint256(_type));
        require(success, "decode key failed");
        require(field == 1, "decode key failed");
        require(
            _type == ProtobufLib.WireType.Varint,
            "decode type is not embedded messages"
        );

        (success, newPos, amount) = ProtobufLib.decode_varint(
            pos,
            program
        );
        console.log(6666, amount);
    }

    function _handleBalance(
        bytes calldata program,
        address assetAddress,
        uint64 pos
    ) internal returns (uint256 amount, uint64 newPos) {
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;

        // reading balance message
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(3444, success, field, uint256(_type));
        require(field == 2, "not balance key id");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );

        // read balance message body
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(32888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading balance type
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(5444, success, field, uint256(_type));
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );

        if (field == 1) {
            // ratio
            uint256 nominator;
            uint256 denominator;
            (nominator, denominator, pos) = _handleRatio(
                program,
                newPos
            );
            amount =
                (IERC20(assetAddress).balanceOf(address(this)) *
                    nominator) /
                denominator;
        } else if (field == 2) {
            // absolute
            (amount, newPos) = _handleAbsolute(program, pos);
        } else if (field == 3) {
            // unit
            //TODO
        } else {
            require(false, "unknown balance type");
        }
    }

    function _handleAsset(bytes calldata program, uint64 pos)
        internal
        returns (
            address asset,
            uint256 amount,
            uint64 newPos
        )
    {
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;
        // reading asset message
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(3444, success, field, uint256(_type));
        require(field == 1, "not asset id");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );

        // read asset message body
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(22888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading asset message
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(5444, success, field, uint256(_type));
        require(field == 1, "not uint64 asset id");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.Varint,
            "decode type is not embedded messages"
        );
        // decode asset id
        uint64 assetId;
        (success, pos, assetId) = ProtobufLib.decode_uint64(
            pos,
            program
        );
        require(success, "decode key failed");

        address assetAddress = IGateway(gatewayAddress).assets(uint256(assetId));
        //require(assetAddress != address(0), "asset not registered");

        // reading
        (amount, newPos) = _handleBalance(program, assetAddress, pos);
    }

    function _handleAssets(bytes calldata program, uint64 pos)
        internal
        returns (address assets, uint64 newPos)
    {
        console.log("handle assets");
        console.logBytes(program[0:pos]);
        // read asset info
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;
        // reading Account
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(9999, success, field, uint256(_type));
        require(field == 3, "not assets key");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );

        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        require(success, "decode embedded message failed");
        uint256 totalAssetsLength = pos + size;
        while (pos < totalAssetsLength) {
            address asset;
            uint256 amount;
            (asset, amount, pos) = _handleAsset(program, pos);
            // TODO transfer asset to account
        }
        newPos = pos;
    }

    function _handleTransfer(bytes calldata program, uint64 pos)
        internal
        returns(uint64 newPos)
    {
        // reading transfer instruction
        bool success;
        uint64 size;
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(666, success, pos, size);
        require(success, "decode embedded message failed");

        uint64 field;
        ProtobufLib.WireType _type;
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(77, success, field, uint256(_type));
        require(success, "decode key failed");

        // account
        if (field == 1) {
            address account;
            (account, pos) = _handelAccount(program, pos);
        } else if (field == 2) {} else {
            revert("no valid field");
        }

        // read assets info
        address assets;
        (assets, newPos) = _handleAssets(program, pos);
    }

    /**
     * @notice encode and decode program using protobuf
     * @param program program encoded in bytes
     */
    function interpret(bytes calldata program)
        public
        onlyOwnerOrCreator
    {
        console.logBytes(program);
        // reading program message
        (
            bool success,
            uint64 pos,
            uint64 field,
            ProtobufLib.WireType _type
        ) = ProtobufLib.decode_key(0, program);
        console.log(pos);
        console.log(field);
        console.log(success);
        console.log(uint256(_type));

        require(success, "decode key failed");
        require(field == 1, "should be Program");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "should be Program"
        );
        console.log(222);

        uint64 size;
        uint32 val;

        // reading instruction message
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0:pos]);
        console.log(success, pos, size);
        require(success, "decode embedded message failed");
        uint64 totalInstructionsLength = pos + size;
        while (pos < totalInstructionsLength) {
            // reading instruction message
            (success, pos, field, _type) = ProtobufLib.decode_key(
                pos,
                program
            );
            console.logBytes(program[0:pos]);
            console.log(2323, success, field, uint256(_type));
            require(field == 1, "not instruction");
            require(success, "decode key failed");
            require(
                _type == ProtobufLib.WireType.LengthDelimited,
                "decode type is not embedded messages"
            );

            // reading instruction size
            (success, pos, size) = ProtobufLib
                .decode_embedded_message(pos, program);
            console.logBytes(program[0:pos]);
            console.log(44444, success, pos, size);
            require(success, "decode embedded message failed");

            uint64 instruction;
            (success, pos, instruction, _type) = ProtobufLib
                .decode_key(pos, program);
            console.logBytes(program[0:pos]);
            console.log(5555, success, field, uint256(_type));
            require(success, "decode key failed");

            if (instruction == uint64(OPERATION.TRANSFER)) {
                pos = _handleTransfer(program, pos);
                console.log(pos, totalInstructionsLength);
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

    function createProgram() public returns (bytes memory program) {}
}
