// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;

import "@lazyledger/protobuf3-solidity-lib/contracts/ProtobufLib.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "./interfaces/IInterpreter.sol";
import "./interfaces/IRouter.sol";
import "./BytesLib.sol";

/**
 * @title Interpreter
 * @notice Custom interpreter
 */
contract Interpreter is IInterpreter {
    using BytesLib for bytes;

    enum OPERATION {
        NONE,
        TRANSFER,
        SPAWN,
        CALL,
        QUERY
    }
    address public creator;
    address public routerAddress;
    mapping(address => bool) public owners;
    IRouter.Origin origin;

    enum BindingValueType {
        NONE,
        ADDRESS,
        UINT256,
        BYTES,
        BALANCE
    }

    struct Binding {
        uint32 position;
        bytes bindingValue;
        BindingValueType bindingType;
    }

    modifier onlyOwnerOrCreator() {
        require(keccak256(abi.encodePacked(msg.sender)) == keccak256(origin.account) || owners[msg.sender] || address(this) == msg.sender);
        _;
    }

    constructor(IRouter.Origin memory _origin, address _routerAddress) {
        owners[msg.sender] = true;
        creator = msg.sender;
        routerAddress = _routerAddress;
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

    function bytesToAddress(bytes memory bys) private pure returns (address addr) {
        assembly {
            addr := mload(add(bys, 20))
        }
    }

    function _checkField(
        bytes calldata program,
        uint64 expectedField,
        ProtobufLib.WireType expectedFieldType,
        uint64 pos
    ) private returns (uint64 newPos) {
        // reading program message
        bool success;
        uint64 _field;
        ProtobufLib.WireType _type;
        (success, newPos, _field, _type) = ProtobufLib.decode_key(pos, program);

        require(success, "decode key failed");
        require(_field == expectedField, "field validation failed");
        require(_type == expectedFieldType, "type validation failed");
    }

    function _getMessageLength(bytes calldata program, uint64 pos) private returns (uint64 size, uint64 newPos) {
        // reading instruction message
        bool success;
        (success, newPos, size) = ProtobufLib.decode_embedded_message(pos, program);
        require(success, "decode embedded message failed");
    }

    function _handleAccount(bytes calldata program, uint64 pos) internal returns (address account, uint64 newPos) {
        // read account info
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);

        // reading Account
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        account = bytesToAddress(program[pos:pos + size]);
        newPos = pos + size;
    }

    function _handleUint128(bytes calldata program, uint64 pos) internal returns (uint128 value, uint64 newPos) {
        bool success;
        uint64 size;
        ProtobufLib.WireType _type;
        // read uint128 message body
        (size, pos) = _getMessageLength(program, pos);

        pos = _checkField(program, 1, ProtobufLib.WireType.Varint, pos);
        uint64 highBits;
        (success, pos, highBits) = ProtobufLib.decode_uint64(pos, program);

        pos = _checkField(program, 2, ProtobufLib.WireType.Varint, pos);
        uint64 lowBits;
        (success, newPos, lowBits) = ProtobufLib.decode_uint64(pos, program);
        value = uint128(highBits) * 2**64 + uint128(lowBits);
    }

    function _handleUnit(
        bytes calldata program,
        uint64 pos,
        address tokenAddress
    ) internal returns (uint256 amount, uint64 newPos) {
        uint64 size;
        // read ratio message body
        (size, pos) = _getMessageLength(program, pos);
        // reading Unit
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        uint128 unit;
        (unit, pos) = _handleUint128(program, pos);
        // reading balance type
        pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);

        uint256 nominator;
        uint256 denominator;
        (nominator, denominator, newPos) = _handleRatio(program, pos);
        uint256 decimals = IERC20Metadata(tokenAddress).decimals();
        amount = uint256(unit) * (10**decimals) + (nominator * (10**decimals)) / denominator;
    }

    function _handleRatio(bytes calldata program, uint64 pos)
        internal
        returns (
            uint256 nominator,
            uint256 denominator,
            uint64 newPos
        )
    {
        uint64 size;
        // read ratio message body
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (nominator, pos) = _handleUint128(program, pos);

        pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);
        (denominator, newPos) = _handleUint128(program, pos);
    }

    function _handleAbsolute(bytes calldata program, uint64 pos) internal returns (uint256 amount, uint64 newPos) {
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);

        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (amount, newPos) = _handleUint128(program, pos);
    }

    function _handleAssetAmount(bytes calldata program, uint64 pos) internal returns (uint256 amount, uint64 newPos) {
        uint64 size;
        bool success;
        uint64 field;
        ProtobufLib.WireType _type;
        // read balance message body
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);

        address asset;
        (asset, pos) = _handleAssetId(program, pos);

        (success, pos, field, _type) = ProtobufLib.decode_key(pos, program);
        require(field == 2 || field == 3, "decode key failed");
        require(success, "decode key failed");
        require(_type == ProtobufLib.WireType.LengthDelimited, "decode type is not embedded messages");

        if (field == 2) {
            // ratio
            uint256 nominator;
            uint256 denominator;
            (nominator, denominator, newPos) = _handleRatio(program, pos);
            amount = (IERC20(asset).balanceOf(address(this)) * nominator) / denominator;
        } else if (field == 3) {
            // unit
            (amount, newPos) = _handleUnit(program, pos, asset);
        }
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

        // read balance message body
        (success, pos, size) = ProtobufLib.decode_embedded_message(pos, program);
        require(success, "decode embedded message failed");

        // reading balance type
        (success, pos, field, _type) = ProtobufLib.decode_key(pos, program);
        require(success, "decode key failed");
        require(_type == ProtobufLib.WireType.LengthDelimited, "decode type is not embedded messages");

        if (field == 1) {
            // ratio
            uint256 nominator;
            uint256 denominator;
            (nominator, denominator, newPos) = _handleRatio(program, pos);
            amount = (IERC20(assetAddress).balanceOf(address(this)) * nominator) / denominator;
        } else if (field == 2) {
            // absolute
            (amount, newPos) = _handleAbsolute(program, pos);
        } else if (field == 3) {
            // unit
            (amount, newPos) = _handleUnit(program, pos, assetAddress);
        } else {
            require(false, "unknown balance type");
        }
    }

    function _handleAssetId(bytes calldata program, uint64 pos) internal returns (address asset, uint64 newPos) {
        uint64 size;
        // read asset message body
        (size, pos) = _getMessageLength(program, pos);
        // reading asset message
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        // decode asset id
        uint128 assetId;
        bool success;
        (assetId, newPos) =_handleUint128(program, pos);

        asset = IRouter(routerAddress).getAsset(uint256(assetId));
        require(asset != address(0), "asset not registered");
    }

    function _handleAsset(bytes calldata program, uint64 pos)
        internal
        returns (
            address asset,
            uint256 amount,
            uint64 newPos
        )
    {
        // reading asset message
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (asset, pos) = _handleAssetId(program, pos);

        // reading asset balance message
        pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);
        // reading
        (amount, newPos) = _handleBalance(program, asset, pos);
    }

    function _handleAssets(
        bytes calldata program,
        uint64 pos,
        address to
    )
        internal
        returns (
            uint64 newPos,
            address[] memory assetAddresses,
            uint256[] memory amounts
        )
    {
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        uint256 totalAssetsLength = pos + size;
        // TODO HARDCODED ARRAY to 10
        assetAddresses = new address[](10);
        amounts = new uint256[](10);
        uint256 count;
        while (pos < totalAssetsLength) {
            (assetAddresses[count], amounts[count], pos) = _handleAsset(program, pos);
            IERC20(assetAddresses[count]).transfer(to, amounts[count]);
            count += 1;
        }
        newPos = pos;
    }

    function _handleTransfer(bytes calldata program, uint64 pos, address relayer) internal returns (uint64 newPos) {
        // reading transfer instruction
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);

        bool success;
        uint64 field;
        ProtobufLib.WireType _type;
        (success, pos, field, _type) = ProtobufLib.decode_key(pos, program);
        require(field == 1 || field == 2, "invalid key field");
        require(success, "decode key failed");

        // account
        address account;
        if (field == 1) {
            (account, pos) = _handleAccount(program, pos);
        } else if (field == 2) {
            //self
            (success, pos, size) = ProtobufLib.decode_embedded_message(pos, program);
            newPos = pos + size;
            account = relayer;
        } else {
            revert("no valid field");
        }

        // read asset info
        pos = _checkField(program, 3, ProtobufLib.WireType.LengthDelimited, pos);
        // read assets info and transfer asset funds
        (newPos, , ) = _handleAssets(program, pos, account);
    }

    function _handleBindingValue(bytes calldata program, uint64 pos, address relayer)
        internal
        returns (bytes memory valueToReplace, uint64 newPos)
    {
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;
        uint64 valueType;

        // read binding value
        //self
        (success, pos, size) = ProtobufLib.decode_embedded_message(pos, program);
        require(success, "decode embedded message failed");

        (success, pos, valueType, _type) = ProtobufLib.decode_key(pos, program);
        require(success, "decode key failed");

        if (valueType == 1) {
            //self
            (success, pos, size) = ProtobufLib.decode_embedded_message(pos, program);
            newPos = pos + size;
            valueToReplace = abi.encode(address(this));
        } else if (valueType == 2) {
            //self
            (success, pos, size) = ProtobufLib.decode_embedded_message(pos, program);
            newPos = pos + size;
            valueToReplace = abi.encode(address(this));
        } else if (valueType == 3) {
            //TODO result
        } else if (valueType == 4) {
            uint256 amount;
            (amount, newPos) = _handleAssetAmount(program, pos);
            valueToReplace = abi.encode(uint256(amount));
            //balance
        } else if (valueType == 5) {
            // asset id
            address asset;
            (asset, newPos) = _handleAssetId(program, pos);
            valueToReplace = abi.encode(asset);
        } else {
            revert("wrong binding value type");
        }
    }

    function _handleBinding(
        bytes calldata program,
        uint64 pos,
        bytes memory payload,
        address relayer
    )
        internal
        returns (
            uint64 position,
            bytes memory valueToReplace,
            uint64 newPos
        )
    {
        uint64 size;
        bool success;
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.Varint, pos);
        (success, pos, position) = ProtobufLib.decode_uint32(pos, program);
        require(success, "decode embedded message failed");

        pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);
        (valueToReplace, pos) = _handleBindingValue(program, pos, relayer);
        newPos = pos;
    }

    function _handleNetwork(bytes calldata program, uint64 pos) internal returns (uint256 networkId, uint64 newPos) {
        // reading network
        uint64 size;
        bool success;
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.Varint, pos);
        (success, newPos, networkId) = ProtobufLib.decode_uint64(pos, program);
        require(success, "decode value failed");
    }

    function _handleSecurity(bytes calldata program, uint64 pos)
        internal
        returns (IRouter.BridgeSecurity security, uint64 newPos)
    {
        // reading network
        bool success;
        int32 value;
        (success, newPos, value) = ProtobufLib.decode_enum(pos, program);
        require(success, "decode key failed");
        security = IRouter.BridgeSecurity(uint32(value));
    }

    function _handleSalt(bytes calldata program, uint64 pos) internal returns (bytes memory salt, uint64 newPos) {
        // reading salt
        bool success;
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        newPos = pos + size;
        salt = program[pos: newPos];
    }

    function _handleProgram(bytes calldata program, uint64 pos)
        internal
        returns (bytes memory subProgram, uint64 newPos)
    {
        // reading program
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        subProgram = program[pos:pos + size];
        newPos = pos + size;
    }

    function _handleSpawnParams(bytes calldata program, uint64 pos)
        internal
        returns (
            uint64 newPos,
            uint256 maxPos,
            uint256 networkId,
            IRouter.BridgeSecurity security,
            bytes memory salt,
            bytes memory spawnedProgram
        )
    {
        // reading spawn instruction
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        maxPos = pos + size;

        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (networkId, pos) = _handleNetwork(program, pos);

        pos = _checkField(program, 2, ProtobufLib.WireType.Varint, pos);
        (security, pos) = _handleSecurity(program, pos);

        // read salt
        pos = _checkField(program, 3, ProtobufLib.WireType.LengthDelimited, pos);
        (salt, pos) = _handleSalt(program, pos);

        // read program
        pos = _checkField(program, 4, ProtobufLib.WireType.LengthDelimited, pos);
        (spawnedProgram, newPos) = _handleProgram(program, pos);
    }

    function _handleSpawn(bytes calldata program, uint64 pos) internal returns (uint64 newPos) {
        (
            uint64 pos,
            uint256 maxPos,
            uint256 networkId,
            IRouter.BridgeSecurity security,
            bytes memory salt,
            bytes memory spawnedProgram
        ) = _handleSpawnParams(program, pos);
        address bridgeAddress = IRouter(routerAddress).getBridge(networkId, security);
        // TODO The fund should be pulled by the Bridge or sent from here??? which is more secure??
        pos = _checkField(program, 5, ProtobufLib.WireType.LengthDelimited, pos);

        address[] memory assetAddresses;
        uint256[] memory amounts;
        if (pos < maxPos) {
            (newPos, assetAddresses, amounts) = _handleAssets(program, pos, bridgeAddress);
        }
        IRouter(routerAddress).emitSpawn(
            origin.account,
            networkId,
            security,
            salt,
            spawnedProgram,
            assetAddresses,
            amounts
        );
    }

    function _replaceBytesByPosition(
        bytes memory payload,
        uint64 position,
        bytes memory s
    ) internal returns (bytes memory) {
        bytes memory temp = new bytes(payload.length + s.length - 1);
        uint256 count = 0;
        for (uint256 i = 0; i < payload.length; i++) {
            if (i != position) {
                temp[i + count] = payload[i];
            } else {
                for (uint256 j = 0; j < s.length; j++) {
                    temp[i + count] = s[count];
                    count++;
                }
                count -= 1;
            }
        }
        return temp;
    }

    function _handleCall(bytes calldata program, uint64 pos, address relayer) internal returns (uint64 newPos) {
        // reading call instruction
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        uint256 maxPos = pos + size;
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        bytes memory payload = program[pos:pos + size];
        bytes memory finalPayload;
        pos = pos + size;
        //bytes memory bindingValues;

        if (pos < maxPos) {
            pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);
            (size, pos) = _getMessageLength(program, pos);
            uint64 totalBindingsLength = pos + size;
            uint64 positionToRight;
            while (pos < totalBindingsLength) {
                uint64 position;
                bytes memory valueToReplace;
                (position, valueToReplace, pos) = _handleBinding(program, pos, payload, relayer);
                payload = _replaceBytesByPosition(payload, position + positionToRight, valueToReplace);
                positionToRight += uint64(valueToReplace.length) - 1;
            }
        }
        address addr;

        //get the address from first bytes
        (addr) = abi.decode(payload, (address));
        finalPayload = payload.slice(32, payload.length - 32);
        (bool succ, bytes memory result) = addr.call(finalPayload);
        require(succ, "error calling target");
        newPos = pos;
    }

    /**
     * @notice encode and decode program using protobuf
     * @param program program encoded in bytes
     */
    function interpret(bytes calldata program, address relayer) public onlyOwnerOrCreator {
        // reading program tag message
        uint64 pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, 0);
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        bytes memory tag = program[pos:pos + size];
        pos = pos + size;

        // reading program message
        pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        uint64 totalInstructionsLength = pos + size;
        while (pos < totalInstructionsLength) {
            // reading instruction message
            pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
            (size, pos) = _getMessageLength(program, pos);

            uint64 instruction;
            bool success;
            ProtobufLib.WireType _type;
            (success, pos, instruction, _type) = ProtobufLib.decode_key(pos, program);
            require(success, "decode key failed");

            if (instruction == uint64(OPERATION.TRANSFER)) {
                pos = _handleTransfer(program, pos, relayer);
            } else if (instruction == uint64(OPERATION.SPAWN)) {
                pos = _handleSpawn(program, pos);
            } else if (instruction == uint8(OPERATION.CALL)) {
                pos = _handleCall(program, pos, relayer);
            }
        }
    }

    function generateUint128(uint128 n) public view returns (bytes memory u128) {
        uint64 highBits = uint64(n >> 64);
        uint64 lowBits = uint64(n);
        return abi.encodePacked(ProtobufLib.encode_key(1, 0), ProtobufLib.encode_uint64(highBits), ProtobufLib.encode_key(2, 0), ProtobufLib.encode_uint64(lowBits));
    }

    function generateAbsolute(uint128 n) public view returns (bytes memory absolute) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(abi.encodePacked(generateUint128(n)))
        );
    }

    function generateRatio(uint128 nominator, uint128 denominator) public view returns (bytes memory ratio) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(generateUint128(nominator)),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(generateUint128(denominator))
        );
    }

    function generateUnit(uint128 integer, bytes memory ratio) public view returns (bytes memory unit) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(generateUint128(integer)),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(ratio)
        );
    }

    function generateBalanceByRatio(bytes memory ratio) public view returns (bytes memory balance) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(ratio)
        );
    }

    function generateBalanceByAbsolute(bytes memory absolute) public view returns (bytes memory balance) {
        return abi.encodePacked(
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(absolute)
        );
    }

    function generateBalanceByUnit(bytes memory _unit) public view returns (bytes memory balance) {
        return abi.encodePacked(
            ProtobufLib.encode_key(3, 2),
            ProtobufLib.encode_length_delimited(_unit)
        );
    }

    function generateAccount(bytes memory _account) public view returns (bytes memory account) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_account)
        );
    }

    function generateAssetId(uint128 _assetId) public view returns (bytes memory assetId) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(generateUint128(_assetId))
        );
    }

    function generateAsset(bytes memory _assetId, bytes memory _balance) public view returns (bytes memory asset) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_assetId),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_balance)
        );
    }

    function generateSelf(uint32 _self) public view returns (bytes memory self) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 0),
            ProtobufLib.encode_uint32(_self)
        );
    }

    function generateRelayer(uint32 _relayer) public view returns (bytes memory relayer) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 0),
            ProtobufLib.encode_uint32(_relayer)
        );
    }

    function generateResult(uint32 _result) public view returns (bytes memory result) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 0),
            ProtobufLib.encode_uint32(_result)
        );
    }

    function generateAssetAmount(bytes memory assetId, bytes memory ratio) public view returns (bytes memory assetAmount) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(assetId),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(ratio)
        );
    }

    function generateBindingValueByAssetAmount(bytes memory _assetAmount) public view returns (bytes memory bindingValue) {
        return abi.encodePacked(
            ProtobufLib.encode_key(4, 2),
            ProtobufLib.encode_length_delimited(_assetAmount)
        );
    }

    function generateBindingValueByAssetId(bytes memory _assetId) public view returns (bytes memory bindingValue) {
        return abi.encodePacked(
            ProtobufLib.encode_key(5, 2),
            ProtobufLib.encode_length_delimited(_assetId)
        );
    }

    function generateBinding(uint32 position, bytes memory _bindingValue) public view returns (bytes memory binding) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 0),
            ProtobufLib.encode_uint32(uint32(position)),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_bindingValue)
        );
    }

    function generateBindings(bytes[] memory _bindings) public view returns (bytes memory bindings) {
        for (uint i = 0; i < _bindings.length; i++) {
            bindings = abi.encodePacked(bindings, ProtobufLib.encode_key(1, 2), ProtobufLib.encode_length_delimited(_bindings[i]));
        }
    }

    function generateTransferByAccount(bytes memory _account, bytes[] memory _assets) public view returns (bytes memory transfer) {
        transfer = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_account)
        );
        for (uint i = 0; i < _assets.length; i++) {
            transfer = abi.encodePacked(transfer, ProtobufLib.encode_key(3, 2), ProtobufLib.encode_length_delimited(_assets[i]));
        }
    }

    function generateSalt(bytes memory _salt) public view returns (bytes memory salt) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_salt)
        );
    }

    function generateNetwork(uint32 _network) public view returns (bytes memory network) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 0),
            ProtobufLib.encode_uint32(_network)
        );
    }

    function generateSpawn(
        bytes memory _network,
        int32 _security,
        bytes memory _salt,
        bytes memory _spawnedProgram,
        bytes[] memory _assets
    ) public view returns (bytes memory spawn) {
        spawn = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_network),
            ProtobufLib.encode_key(2, 0),
            ProtobufLib.encode_enum(_security),
            ProtobufLib.encode_key(3, 2),
            ProtobufLib.encode_length_delimited(_salt),
            ProtobufLib.encode_key(4, 2),
            ProtobufLib.encode_length_delimited(_spawnedProgram)
        );
        for (uint i = 0; i < _assets.length; i++) {
            spawn = abi.encodePacked(spawn, ProtobufLib.encode_key(5, 2), ProtobufLib.encode_length_delimited(_assets[i]));
        }
    }

    function generateCall(
        bytes memory _payload,
        bytes memory _bindings
    ) public view returns (bytes memory call) {
        call = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_payload),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_bindings)
        );
    }

    function generateInstructionByTransfer(
        bytes memory _transfer
    ) public view returns (bytes memory instruction) {
        instruction = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_transfer)
        );
    }

    function generateInstructionByCall(
        bytes memory _call
    ) public view returns (bytes memory instruction) {
        instruction = abi.encodePacked(
            ProtobufLib.encode_key(3, 2),
            ProtobufLib.encode_length_delimited(_call)
        );
    }

    function generateInstructionBySpawn(
        bytes memory _spawn
    ) public view returns (bytes memory instruction) {
        instruction = abi.encodePacked(
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_spawn)
        );
    }

    function generateInstructionByQuery(
        bytes memory _query
    ) public view returns (bytes memory instruction) {
        instruction = abi.encodePacked(
            ProtobufLib.encode_key(4, 2),
            ProtobufLib.encode_length_delimited(_query)
        );
    }

    function generateInstructions(bytes[] memory _instructions) public view returns (bytes memory instructions) {
        for (uint i = 0; i < _instructions.length; i++) {
            instructions = abi.encodePacked(instructions, ProtobufLib.encode_key(1, 2), ProtobufLib.encode_length_delimited(_instructions[i]));
        }
    }

    function generateProgram(
        bytes memory _tag,
        bytes memory _instructions
    ) public view returns (bytes memory program) {
        program = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_tag),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_instructions)
        );
    }
}
