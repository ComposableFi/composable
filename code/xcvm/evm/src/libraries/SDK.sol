// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;

import "protobuf3-solidity-lib/ProtobufLib.sol";
import "openzeppelin-contracts/token/ERC20/IERC20.sol";
import "openzeppelin-contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "../interfaces/IRouter.sol";
import "bytes-utils/BytesLib.sol";
import "../libraries/xc/xcvm.sol";

library SDK {

    using BytesLib for bytes;

    struct SpawnPacket {
        Spawn[] spawn;
        Program program;
    }

    struct Spawn {
        Program program;
    }

    /// as program interpetered, operatios poped out
    /// for each operation, it goes to relevant instuction index to pop
    struct Program {
        OPERATION[] operation;            
        /// actually recursive definition (which is not part of solidity language)
        uint8[] spawns;
    }   

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

    enum OPERATION {
        NONE,
        TRANSFER,
        SPAWN,
        CALL,
        QUERY
    }

    struct AssetInfos {
        address[] assetAddresses;
        uint128[] assetIds;
        uint256[] amounts;
    }

    function bytesToAddress(bytes memory bys) private pure returns (address addr) {
        assembly {
            addr := mload(add(bys, 20))
        }
    }

    function _checkField(
        bytes memory program,
        uint64 expectedField,
        ProtobufLib.WireType expectedFieldType,
        uint64 pos
    ) private pure returns (uint64 newPos) {
        // reading program message
        bool success;
        uint64 _field;
        ProtobufLib.WireType _type;
        (success, newPos, _field, _type) = ProtobufLib.decode_key(pos, program);

        require(success, "decode key failed");
        require(_field == expectedField, "field validation failed");
        require(_type == expectedFieldType, "type validation failed");
    }

    function _getMessageLength(bytes memory program, uint64 pos) private pure returns (uint64 size, uint64 newPos) {
        // reading instruction message
        bool success;
        (success, newPos, size) = ProtobufLib.decode_embedded_message(pos, program);
        require(success, "decode embedded message failed");
    }

    function _handleAccount(bytes memory program, uint64 pos) internal view returns (address account, uint64 newPos) {
        // read account info
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);

        // reading Account
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        account = bytesToAddress(program.slice(pos, size));
        newPos = pos + size;
    }

    function _handleUint128(bytes memory program, uint64 pos) internal pure returns (uint128 value, uint64 newPos) {
        bool success;
        uint64 size;
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
        bytes memory program,
        uint64 pos,
        address tokenAddress
    ) internal view returns (uint256 amount, uint64 newPos) {
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

    function _handleRatio(bytes memory program, uint64 pos)
        internal pure
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

    function _handleAbsolute(bytes memory program, uint64 pos) internal pure returns (uint256 amount, uint64 newPos) {
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);

        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (amount, newPos) = _handleUint128(program, pos);
    }

    function _handleAssetAmount(bytes memory program, uint64 pos, address routerAddress) internal view returns (uint256 amount, uint64 newPos) {
        uint64 size;
        bool success;
        uint64 field;
        ProtobufLib.WireType _type;
        // read balance message body
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);

        address asset;
        uint128 assetId;
        (asset, assetId, pos) = _handleAssetId(program, pos, routerAddress);

        pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);
        (amount, newPos) = _handleBalance(program, asset, pos);
    }

    function _handleBalance(
        bytes memory program,
        address assetAddress,
        uint64 pos
    ) internal view returns (uint256 amount, uint64 newPos) {
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

    function _handleLocalId(bytes memory program, uint64 pos) internal pure returns(address assetAddress, uint64 newPos) {
        uint64 size;
        bool success;
        uint64 field;
        ProtobufLib.WireType _type;
        // read balance message body
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        assetAddress = bytesToAddress(program.slice(pos, size));
        newPos = pos + size;
    }

    function _handleGlobalId(bytes memory program, uint64 pos) internal view returns(uint128 assetId, uint64 newPos) {
        uint64 size;
        bool success;
        uint64 field;
        ProtobufLib.WireType _type;
        // read balance message body
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (assetId, newPos) = _handleUint128(program, pos);
    }

    function _handleAssetId(bytes memory program, uint64 pos, address routerAddress) internal view returns (address asset, uint128 assetId, uint64 newPos) {
        uint64 size;
        // read asset message body
        (size, pos) = _getMessageLength(program, pos);

        bool success;
        uint64 field;
        ProtobufLib.WireType _type;
        (success, pos, field, _type) = ProtobufLib.decode_key(pos, program);
        require(field == 1 || field == 2, "invalid key field");
        require(success, "decode key failed");

        // asset id
        if (field == 1) {
            (assetId, newPos) = _handleGlobalId(program, pos);
            asset = IRouter(routerAddress).getAsset(uint256(assetId));
        } else if (field == 2) {
            (asset, newPos) = _handleLocalId(program, pos);
            assetId = uint128(IRouter(routerAddress).getAssetIdByLocalId(asset));
        } else {
            revert("no valid field");
        }
    }

    function _handleAsset(bytes memory program, uint64 pos, address routerAddress)
        internal view
        returns (
            address asset,
            uint128 assetId,
            uint256 amount,
            uint64 newPos
        )
    {
        // reading asset message
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (asset, assetId, pos) = _handleAssetId(program, pos, routerAddress);

        // reading asset balance message
        pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);
        // reading
        (amount, newPos) = _handleBalance(program, asset, pos);
    }

    function _getAssetListLength(bytes memory program, uint64 pos, uint256 totalAssetsLength, address routerAddress) internal view returns (uint256 length){
        //TODO optimization 
        while (pos < totalAssetsLength) {
            (, , , pos) = _handleAsset(program, pos, routerAddress);
            length += 1;
        }
    } 

    function _handleAssets(
        bytes memory program,
        uint64 pos,
        address routerAddress
    )
        internal
        returns (
            uint64 newPos,
            AssetInfos memory assetInfos
        )
    {
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        uint256 totalAssetsLength = pos + size;
        uint256 assetListLength = _getAssetListLength(program, pos, totalAssetsLength, routerAddress);

        assetInfos.assetAddresses = new address[](assetListLength);
        assetInfos.assetIds = new uint128[](assetListLength);
        assetInfos.amounts = new uint256[](assetListLength);
        uint256 count;
        while (pos < totalAssetsLength) {
            (assetInfos.assetAddresses[count], assetInfos.assetIds[count], assetInfos.amounts[count], pos) = _handleAsset(program, pos, routerAddress);
            count += 1;
        }
        newPos = pos;
    }


    function _handleTransfer(bytes memory program, uint64 pos, address relayer, address routerAddress) internal returns (uint64 newPos) {
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
        AssetInfos memory assetInfos;
        (newPos, assetInfos) = _handleAssets(program, pos, routerAddress);
        for (uint256 count = 0; count < assetInfos.assetAddresses.length; count++){
            IERC20(assetInfos.assetAddresses[count]).transfer(account, assetInfos.amounts[count]);
        }
    }

    function _handleBindingValue(bytes memory program, uint64 pos, address relayer, address routerAddress)
        internal view
        returns (bytes memory valueToReplace, uint64 newPos)
    {
        bool success;
        uint64 size;
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
            //relayer
            (success, pos, size) = ProtobufLib.decode_embedded_message(pos, program);
            newPos = pos + size;
            valueToReplace = abi.encode(relayer);
        } else if (valueType == 3) {
            //TODO result
        } else if (valueType == 4) {
            uint256 amount;
            (amount, newPos) = _handleAssetAmount(program, pos, routerAddress);
            valueToReplace = abi.encode(uint256(amount));
            //balance
        } else if (valueType == 5) {
            // global id
            address asset;
            uint256 assetId;
            (assetId, newPos) = _handleGlobalId(program, pos);
            asset = IRouter(routerAddress).getAsset(uint256(assetId));
            valueToReplace = abi.encode(asset);
        } else {
            revert("wrong binding value type");
        }
    }

    function _handleBinding(
        bytes memory program,
        uint64 pos,
        bytes memory payload,
        address relayer,
        address routerAddress
    )
        internal view
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
        (valueToReplace, pos) = _handleBindingValue(program, pos, relayer, routerAddress);
        newPos = pos;
    }

    function _handleNetwork(bytes memory program, uint64 pos) internal pure returns (uint128 networkId, uint64 newPos) {
        // reading network
        uint64 size;
        bool success;
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (networkId, newPos) = _handleUint128(program, pos);
    }

    function _handleSecurity(bytes memory program, uint64 pos)
        internal pure
        returns (IRouter.BridgeSecurity security, uint64 newPos)
    {
        // reading network
        bool success;
        int32 value;
        (success, newPos, value) = ProtobufLib.decode_enum(pos, program);
        require(success, "decode key failed");
        security = IRouter.BridgeSecurity(uint32(value));
    }

    function _handleSalt(bytes memory program, uint64 pos) internal pure returns (bytes memory salt, uint64 newPos) {
        // reading salt
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        newPos = pos + size;
        salt = program.slice(pos, size);
    }

    function _handleProgram(bytes memory program, uint64 pos)
        internal pure
        returns (bytes memory subProgram, uint64 newPos)
    {
        // reading program
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        subProgram = program.slice(pos, size);
        newPos = pos + size;
    }

    function _handleSpawnParams(bytes memory program, uint64 pos)
        internal pure
        returns (
            uint64 newPos,
            uint256 maxPos,
            uint128 networkId,
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

    function _handleSpawn(bytes memory program, uint64 pos, address routerAddress, IRouter.Origin memory origin) internal returns (uint64 newPos) {

        uint256 maxPos;
        uint128 networkId;
        IRouter.BridgeSecurity security;
        bytes memory salt;
        bytes memory spawnedProgram;

        (pos, maxPos, networkId, security, salt, spawnedProgram) = _handleSpawnParams(program, pos); // TODO The fund should be pulled by the Bridge or sent from here??? which is more secure??
        pos = _checkField(program, 5, ProtobufLib.WireType.LengthDelimited, pos);

        AssetInfos memory assetInfos;
        if (pos < maxPos) {
            (newPos, assetInfos) = _handleAssets(program, pos, routerAddress);
        }

        address to = IRouter(routerAddress).getBridge(networkId, security);
        for (uint256 count = 0; count < assetInfos.assetAddresses.length; count++){
            IERC20(assetInfos.assetAddresses[count]).transfer(to, assetInfos.amounts[count]);
        }

        IRouter(routerAddress).emitSpawn(
            origin.account,
            networkId,
            security,
            salt,
            spawnedProgram,
            assetInfos.assetAddresses,
            assetInfos.assetIds,
            assetInfos.amounts
        );
    }

    function _replaceBytesByPosition(
        bytes memory payload,
        uint64 position,
        bytes memory s
    ) internal pure returns (bytes memory) {
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

    function _handleCall(bytes memory program, uint64 pos, address relayer, address routerAddress) internal returns (uint64 newPos) {
        // reading call instruction
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        uint256 maxPos = pos + size;
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        bytes memory payload = program.slice(pos, size);
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
                (position, valueToReplace, pos) = _handleBinding(program, pos, payload, relayer, routerAddress);
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

    function interpretProgram(bytes memory program, address relayer, address routerAddress, IRouter.Origin memory origin) public {
        // reading program tag message
        uint64 pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, 0);
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        bytes memory tag = program.slice(pos, size);
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
                pos = _handleTransfer(program, pos, relayer, routerAddress);
            } else if (instruction == uint64(OPERATION.SPAWN)) {
                pos = _handleSpawn(program, pos, routerAddress, origin);
            } else if (instruction == uint8(OPERATION.CALL)) {
                pos = _handleCall(program, pos, relayer, routerAddress);
            }
        }
    }

    // ibc package specific functions
    function _handleInterpreterOrigin(bytes memory program, uint64 pos) public pure returns(bytes memory originInterpreter, uint64 newPos) {
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (size, pos) = _getMessageLength(program, pos);
        originInterpreter = program.slice(pos, size);
        newPos = pos + size;
    }

    function _handleUserOrigin(bytes memory program, uint64 pos) public view returns(address account, uint128 networkId, uint64 newPos) {
        uint64 size;
        (size, pos) = _getMessageLength(program, pos);
        pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        (account, pos) = _handleAccount(program, pos);
        pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);
        (networkId, newPos) = _handleNetwork(program, pos);
    }

    // view functions to generate xcvm bytecode using protobuf

    function generateUint128(uint128 n) public pure returns (bytes memory u128) {

        uint64 highBits = uint64(n >> 64);
        uint64 lowBits = uint64(n);
        return abi.encodePacked(ProtobufLib.encode_key(1, 0), ProtobufLib.encode_uint64(highBits), ProtobufLib.encode_key(2, 0), ProtobufLib.encode_uint64(lowBits));
    }

    function generateAbsolute(uint128 n) public pure returns (bytes memory absolute) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(abi.encodePacked(generateUint128(n)))
        );
    }

    function generateRatio(uint128 nominator, uint128 denominator) public pure returns (bytes memory ratio) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(generateUint128(nominator)),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(generateUint128(denominator))
        );
    }

    function generateUnit(uint128 integer, bytes memory ratio) public pure returns (bytes memory unit) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(generateUint128(integer)),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(ratio)
        );
    }

    function generateBalanceByRatio(bytes memory ratio) public pure returns (bytes memory balance) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(ratio)
        );
    }

    function generateBalanceByAbsolute(bytes memory absolute) public pure returns (bytes memory balance) {
        return abi.encodePacked(
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(absolute)
        );
    }

    function generateBalanceByUnit(bytes memory _unit) public pure returns (bytes memory balance) {
        return abi.encodePacked(
            ProtobufLib.encode_key(3, 2),
            ProtobufLib.encode_length_delimited(_unit)
        );
    }

    function generateAccount(bytes memory _account) public pure returns (bytes memory account) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_account)
        );
    }

    function generateGlobalId(uint128 _globalId) public pure returns (bytes memory globalId) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(generateUint128(_globalId))
        );
    }

    function generateLocalId(bytes memory _localId) public pure returns (bytes memory localId) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_localId)
        );
    }

    function generateAssetIdByGlobalId(bytes memory globalId) public pure returns (bytes memory assetId) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(globalId)
        );
    }

    function generateAssetIdByLocalId(bytes memory local) public pure returns (bytes memory assetId) {
        return abi.encodePacked(
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(local)
        );
    }

    function generateAsset(bytes memory _assetId, bytes memory _balance) public pure returns (bytes memory asset) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_assetId),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_balance)
        );
    }

    function generateSelf(uint32 _self) public pure returns (bytes memory self) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 0),
            ProtobufLib.encode_uint32(_self)
        );
    }

    function generateRelayer(uint32 _relayer) public pure returns (bytes memory relayer) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 0),
            ProtobufLib.encode_uint32(_relayer)
        );
    }

    function generateResult(uint32 _result) public pure returns (bytes memory result) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 0),
            ProtobufLib.encode_uint32(_result)
        );
    }

    function generateAssetAmount(bytes memory assetId, bytes memory balance) public pure returns (bytes memory assetAmount) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(assetId),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(balance)
        );
    }

    function generateBindingValueByAssetAmount(bytes memory _assetAmount) public pure returns (bytes memory bindingValue) {
        return abi.encodePacked(
            ProtobufLib.encode_key(4, 2),
            ProtobufLib.encode_length_delimited(_assetAmount)
        );
    }

    function generateBindingValueByGlobalId(bytes memory _globalId) public pure returns (bytes memory bindingValue) {
        return abi.encodePacked(
            ProtobufLib.encode_key(5, 2),
            ProtobufLib.encode_length_delimited(_globalId)
        );
    }

    function generateBinding(uint32 position, bytes memory _bindingValue) public pure returns (bytes memory binding) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 0),
            ProtobufLib.encode_uint32(uint32(position)),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_bindingValue)
        );
    }

    function generateBindings(bytes[] memory _bindings) public pure returns (bytes memory bindings) {
        for (uint i = 0; i < _bindings.length; i++) {
            bindings = abi.encodePacked(bindings, ProtobufLib.encode_key(1, 2), ProtobufLib.encode_length_delimited(_bindings[i]));
        }
    }

    function generateTransferByAccount(bytes memory _account, bytes[] memory _assets) public pure returns (bytes memory transfer) {
        transfer = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_account)
        );
        for (uint i = 0; i < _assets.length; i++) {
            transfer = abi.encodePacked(transfer, ProtobufLib.encode_key(3, 2), ProtobufLib.encode_length_delimited(_assets[i]));
        }
    }

    function generateSalt(bytes memory _salt) public pure returns (bytes memory salt) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_salt)
        );
    }

    function generateNetwork(uint128 _network) public pure returns (bytes memory network) {
        return abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(abi.encodePacked(generateUint128(_network)))
        );
    }

    function generateSpawn(
        bytes memory _network,
        int32 _security,
        bytes memory _salt,
        bytes memory _spawnedProgram,
        bytes[] memory _assets
    ) public pure returns (bytes memory spawn) {
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
    ) public pure returns (bytes memory call) {
        call = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_payload),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_bindings)
        );
    }

    function generateInstructionByTransfer(
        bytes memory _transfer
    ) public pure returns (bytes memory instruction) {
        instruction = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_transfer)
        );
    }

    function generateInstructionByCall(
        bytes memory _call
    ) public pure returns (bytes memory instruction) {
        instruction = abi.encodePacked(
            ProtobufLib.encode_key(3, 2),
            ProtobufLib.encode_length_delimited(_call)
        );
    }

    function generateInstructionBySpawn(
        bytes memory _spawn
    ) public pure returns (bytes memory instruction) {
        instruction = abi.encodePacked(
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_spawn)
        );
    }

    function generateInstructionByQuery(
        bytes memory _query
    ) public pure returns (bytes memory instruction) {
        instruction = abi.encodePacked(
            ProtobufLib.encode_key(4, 2),
            ProtobufLib.encode_length_delimited(_query)
        );
    }

    function generateInstructions(bytes[] memory _instructions) public pure returns (bytes memory instructions) {
        for (uint i = 0; i < _instructions.length; i++) {
            instructions = abi.encodePacked(instructions, ProtobufLib.encode_key(1, 2), ProtobufLib.encode_length_delimited(_instructions[i]));
        }
    }

    function generateProgram(
        bytes memory _tag,
        bytes memory _instructions
    ) public pure returns (bytes memory program) {
        program = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(_tag),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(_instructions)
        );
    }

    // IBC spawn program encoder and decoder
    function generateUserOrigin(bytes memory account, uint128 networkId) public pure returns (bytes memory userOrigin) {
        userOrigin = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(generateAccount(account)),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(generateNetwork(networkId))
        );
    }
    
    function generateInterpreterOrigin(bytes memory account) public pure returns (bytes memory interpreterOrigin) {
        interpreterOrigin = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(generateAccount(account))
        );
    }

    function generateIBCSpawn(
        bytes memory interpreterOrigin,
        bytes memory userOrigin,
        bytes memory _salt,
        bytes memory _spawnedProgram,
        uint128[] memory assetIds,
        uint256[] memory amounts
    ) public pure returns (bytes memory spawn) {
        spawn = abi.encodePacked(
            ProtobufLib.encode_key(1, 2),
            ProtobufLib.encode_length_delimited(interpreterOrigin),
            ProtobufLib.encode_key(2, 2),
            ProtobufLib.encode_length_delimited(userOrigin),
            ProtobufLib.encode_key(3, 2),
            ProtobufLib.encode_length_delimited(generateSalt(_salt)),
            ProtobufLib.encode_key(4, 2),
            ProtobufLib.encode_length_delimited(_spawnedProgram)
        );
        for (uint i = 0; i < assetIds.length; i++) {
            bytes memory assetId = generateAssetIdByGlobalId(generateGlobalId(assetIds[i]));
            bytes memory asset = generateAsset(
                    assetId, generateBalanceByAbsolute(
                        generateAbsolute(uint128(amounts[i]))
                    ));
            spawn = abi.encodePacked(spawn, ProtobufLib.encode_key(5, 2), ProtobufLib.encode_length_delimited(asset));
        }
    }

    

    function decodeIBCSpawn(
        bytes memory program,
        address routerAddress
    ) public 
    returns (
        bytes memory originInterpreter, IRouter.Origin memory origin, bytes memory spawnedProgram, bytes memory salt, address[] memory assetAddresses, uint256[] memory assetAmounts)
    {
        CvmXcvmPacket.Data memory packet = CvmXcvmPacket.decode(program);
        require(false, "next PRs will switch from manual parser to generated");
        // // reading spawn instruction
        // uint64 size;
        // uint64 pos;

        // pos = _checkField(program, 1, ProtobufLib.WireType.LengthDelimited, pos);
        // (originInterpreter, pos) = _handleInterpreterOrigin(program, pos);

        // pos = _checkField(program, 2, ProtobufLib.WireType.LengthDelimited, pos);
        // uint128 networkId;
        // address account;
        // (account, networkId, pos) = _handleUserOrigin(program, pos);

        // origin.account = abi.encodePacked(account);
        // pos = _checkField(program, 5, ProtobufLib.WireType.LengthDelimited, pos);
        // // transfer assets to router
        // (, AssetInfos memory assetInfos) = _handleAssets(
        //     program,
        //     pos,
        //     routerAddress
        // );
        // for (uint256 count = 0; count < assetInfos.assetAddresses.length; count++){
        //     IERC20(assetInfos.assetAddresses[count]).transfer(routerAddress, assetInfos.amounts[count]);
        // }
        // assetAddresses = assetInfos.assetAddresses;
        // assetAmounts = assetInfos.amounts;
    }
}