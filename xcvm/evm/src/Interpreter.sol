// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;

import "@lazyledger/protobuf3-solidity-lib/contracts/ProtobufLib.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";
import "@openzeppelin/contracts/token/ERC20/extensions/IERC20Metadata.sol";
import "./interfaces/IInterpreter.sol";
import "./interfaces/IGateway.sol";
import "./BytesLib.sol";
import "forge-std/console.sol";

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
    address public gatewayAddress;
    IGateway.Origin origin;

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
        require(
            keccak256(abi.encodePacked(msg.sender)) ==
            keccak256(origin.account) ||
            msg.sender == creator
        );
        _;
    }

    constructor(IGateway.Origin memory _origin, address _gatewayAddress) {
        creator = msg.sender;
        gatewayAddress = _gatewayAddress;
        origin = _origin;
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
        console.logBytes(program[0 : pos]);
        console.log(888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading Account
        uint64 field;
        ProtobufLib.WireType _type;
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(99991, success, field, uint256(_type));
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
        console.logBytes(program[0 : pos]);
        console.log(11888, success, pos, size);
        require(success, "decode embedded message failed");

        console.logBytes(program[pos : pos + size]);
        account = bytesToAddress(program[pos : pos + size]);
        console.log(account);
        newPos = pos + size;
    }

    function _handleUnit(bytes calldata program, uint64 pos, address tokenAddress)
    internal
    returns (uint256 amount, uint64 newPos)
    {
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;
        console.log("handling unit");

        // read ratio message body
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(52888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading Unit
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(6444, success, field, uint256(_type));
        require(success, "decode key failed");
        require(field == 1, "decode key failed");
        require(
            _type == ProtobufLib.WireType.Varint,
            "decode type is not embedded messages"
        );
        uint64 unit;
        (success, pos, unit) = ProtobufLib.decode_varint(
            pos,
            program
        );
        console.log(111, unit);
        // reading balance type
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        require(field == 2, "decode key failed");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );


        uint256 nominator;
        uint256 denominator;
        (nominator, denominator, newPos) = _handleRatio(program, pos);
        uint256 decimals = IERC20Metadata(tokenAddress).decimals();
        amount = uint256(unit) * (10 ** decimals) + nominator * (10 ** decimals) / denominator;
        console.log(uint256(unit) * (10 ** decimals));
        console.log(nominator * (10 ** decimals) / denominator);
        console.log(amount);
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

        console.logBytes(program[0 : pos]);
        // read ratio message body
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(62888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading ratio denominator
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(5443, success, field, uint256(_type));
        require(success, "decode key failed");
        // require(field == 1, "decode key failed");
        require(
            _type == ProtobufLib.WireType.Varint,
            "decode type is not embedded messages"
        );

        (success, pos, nominator) = ProtobufLib.decode_varint(
            pos,
            program
        );
        console.log(9999999, nominator);

        // reading ratio nominator
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
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
        console.log("nominator", denominator);
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
        console.logBytes(program[0 : pos]);
        console.log(42888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading ratio denominator
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
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

    function _handleAssetAmount(
        bytes calldata program,
        uint64 pos
    ) internal returns (uint256 amount, uint64 newPos) {
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;

        // read balance message body
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(32888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading asset id 
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(5444, success, field, uint256(_type));
        require(field == 1, "decode key failed");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );

        address asset;
        (asset, pos) = _handleAssetId(program, pos);
        console.log("asset address", asset);
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(5444, success, field, uint256(_type));
        require(field == 2 || field == 3, "decode key failed");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );


        if (field == 2) {
            // ratio
            uint256 nominator;
            uint256 denominator;
            (nominator, denominator, newPos) = _handleRatio(
                program,
                pos
            );
            amount = IERC20(asset).balanceOf(address(this)) * nominator / denominator;
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
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(32888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading balance type
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
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
            (nominator, denominator, newPos) = _handleRatio(
                program,
                pos
            );
            amount = IERC20(assetAddress).balanceOf(address(this)) * nominator / denominator;
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

    function _handleAssetId(bytes calldata program, uint64 pos) internal returns (address asset, uint64 newPos){
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;

        // read asset message body
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(22888, success, pos, size);
        require(success, "decode embedded message failed");

        // reading asset message
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(5444, success, field, uint256(_type));
        require(field == 1, "not uint64 asset id");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.Varint,
            "decode type is not embedded messages"
        );
        // decode asset id
        uint64 assetId;
        (success, newPos, assetId) = ProtobufLib.decode_uint64(
            pos,
            program
        );
        require(success, "decode key failed");

        asset = IGateway(gatewayAddress).getAsset(
            uint256(assetId)
        );
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
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;
        // reading asset message
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(3444, success, field, uint256(_type));
        require(field == 1, "not asset id");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );


        (asset, pos) = _handleAssetId(program, pos);


        // reading asset message
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(3444, success, field, uint256(_type));
        require(field == 2, "not asset id");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );


        // reading
        (amount, newPos) = _handleBalance(program, asset, pos);
    }

    function _handleAssets(bytes calldata program, uint64 pos, address to)
    internal
    returns (uint64 newPos, address[] memory assetAddresses, uint256[] memory amounts)
    {
        bool success;
        uint64 field;
        uint64 size;
        ProtobufLib.WireType _type;
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        require(success, "decode embedded message failed");
        uint256 totalAssetsLength = pos + size;
        // TODO HARDCOCDED ARRAY to 10
        assetAddresses = new address[](10);
        amounts = new uint256[](10);
        uint256 count;
        while (pos < totalAssetsLength) {
            (assetAddresses[count], amounts[count], pos) = _handleAsset(program, pos);
            IERC20(assetAddresses[count]).transfer(
                to,
                amounts[count]
            );
            count += 1;
        }
        newPos = pos;
    }

    function _handleTransfer(bytes calldata program, uint64 pos)
    internal
    returns (uint64 newPos)
    {
        // reading transfer instruction
        bool success;
        uint64 size;
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(666, success, pos, size);
        require(success, "decode embedded message failed");

        uint64 field;
        ProtobufLib.WireType _type;
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(77, success, field, uint256(_type));
        require(success, "decode key failed");

        // account
        address account;
        if (field == 1) {
            (account, pos) = _handelAccount(program, pos);
        } else if (field == 2) {} else {
            revert("no valid field");
        }

        console.log("handle assets");
        console.logBytes(program[0 : pos]);
        // read asset info
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(9999, success, field, uint256(_type));
        require(field == 3, "not assets key");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );

        // read assets info and transfer asset funds
        (newPos,,) = _handleAssets(program, pos, account);
    }

    function _handleBindingValue(bytes calldata program, uint64 pos) internal returns (bytes memory valueToReplace, uint64 newPos) {

        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;
        uint64 valueType;

        // read binding value
        //self
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.log(666444, success, pos, size);
        require(success, "decode embedded message failed");

        (success, pos, valueType, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(3333333333333333332326, success, valueType, uint256(_type));
        require(success, "decode key failed");


        if (valueType == 1) {
            //self
            (success, pos, size) = ProtobufLib.decode_embedded_message(
                pos,
                program
            );
            newPos = pos + size;
            valueToReplace = abi.encode(address(this));
        } else if (valueType == 2) {
            // relayer
        } else if (valueType == 3) {
            //result
        } else if (valueType == 4) {
            console.log("handling assetAmount type");
            uint256 amount;
            (amount, newPos) = _handleAssetAmount(program, pos);
            console.log("amount", amount);
            valueToReplace = abi.encode(uint256(amount));
            //balance
        } else if (valueType == 5) {
            // asset id
            address asset;
            (asset, newPos) = _handleAssetId(program, pos);
            console.log('asset id', asset);
            console.log('newPos', newPos);
            console.log(asset);
            valueToReplace = abi.encode(asset);
            console.logBytes(valueToReplace);
        } else {
            revert("woring binding value type");
        }
    }

    function _handleBinding(bytes calldata program, uint64 pos, bytes memory payload)
    internal
    returns (uint64 position, bytes memory valueToReplace, uint64 newPos)
    {
        bool success;
        uint64 size;
        uint64 field;
        ProtobufLib.WireType _type;

        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(3333333333333333332329, success, field, uint256(_type));
        //require(field == 1, "not position");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is a LengthDelimited"
        );

        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(3333444, success, pos, size);
        require(success, "decode embedded message failed");

        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(3333333333333333332324, success, field, uint256(_type));
        require(field == 1, "not position");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.Varint,
            "decode type is a varint"
        );

        (success, pos, position) = ProtobufLib.decode_uint32(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(888, success, pos, position);
        require(success, "decode embedded message failed");

        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(3333333333333333332323, success, field, uint256(_type));
        require(field == 2, "not binding value");
        require(success, "decode key failed");
        require(
            _type == ProtobufLib.WireType.LengthDelimited,
            "decode type is not embedded messages"
        );

        (valueToReplace, pos) = _handleBindingValue(program, pos);
        newPos = pos;
    }

    function _handleNetwork(bytes calldata program, uint64 pos) internal
    returns (uint256 networkId, uint64 newPos)
    {
        // reading network 
        bool success;
        uint64 size;
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(666, success, pos, size);
        require(success, "decode embedded message failed");

        uint64 field;
        ProtobufLib.WireType _type;
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );

        require(success, "decode key failed");
        require(field == 1, "not network");
        require(_type == ProtobufLib.WireType.Varint, "not varint");

        (success, newPos, networkId) = ProtobufLib.decode_uint64(
            pos,
            program
        );
        console.logBytes(program[:newPos]);
        console.log("networkId", networkId);
        require(success, "decode value failed");
    }


    function _handleSecurity(bytes calldata program, uint64 pos) internal
    returns (IGateway.BridgeSecurity security, uint64 newPos)
    {

        // reading network
        bool success;
        int32 value;
        (success, newPos, value) = ProtobufLib.decode_enum(
            pos,
            program
        );
        require(success, "decode key failed");
        security = IGateway.BridgeSecurity(uint32(value));
    }


    function _handleSalt(bytes calldata program, uint64 pos) internal
    returns (uint64 salt, uint64 newPos)
    {
        // reading salt
        bool success;
        uint64 size;
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(666, success, pos, size);
        require(success, "decode embedded message failed");

        uint64 field;
        ProtobufLib.WireType _type;
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );

        require(success, "decode key failed");
        require(field == 1, "not salt");
        require(_type == ProtobufLib.WireType.Varint, "not varint");

        (success, newPos, salt) = ProtobufLib.decode_uint64(
            pos,
            program
        );
        require(success, "decode value failed");
    }

    function _handleProgram(bytes calldata program, uint64 pos)
    internal
    returns (bytes memory subProgram, uint64 newPos) {

        // reading network
        bool success;
        uint64 size;
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        require(success, "decode embedded message failed");
        console.logBytes(program[0 : pos]);
        subProgram = program[pos : pos + size];
        newPos = pos + size;
    }

    function _handleSpawnParams(bytes calldata program, uint64 pos)
        internal
        returns (uint64 newPos, uint256 maxPos, uint256 networkId, IGateway.BridgeSecurity security, uint256 salt, bytes memory spawnedProgram) {
        // reading spawm instruction
        bool success;
        uint64 field;
        uint64 size;
        ProtobufLib.WireType _type;
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(666, success, pos, size);
        maxPos = pos + size;
        require(success, "decode embedded message failed");

        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(77, success, field, uint256(_type));
        console.log(11111111111111);
        require(success, "decode key failed");
        require(field == 1, "not network");

        (networkId, pos) = _handleNetwork(program, pos);

        // read bridgeSecurity
        /// (success, pos, size) = ProtobufLib.decode_embedded_message(
        ///     pos,
        ///     program
        /// );
        /// console.logBytes(program[0 : pos]);
        /// console.log(666, success, pos, size);
        /// require(success, "decode embedded message failed");

        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(77, success, field, uint256(_type));
        console.log(11111111111111);
        require(success, "decode key failed");
        require(field == 2, "not security field");
        require(_type == ProtobufLib.WireType.Varint, "not security");

        (security, pos) = _handleSecurity(program, pos);
        console.log("security ", uint256(security));

        // read salt
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(77, success, field, uint256(_type));
        console.log(11111111111111);
        require(success, "decode key failed");
        require(field == 3, "not salt");
        require(_type == ProtobufLib.WireType.LengthDelimited, "not salt");

        (salt, pos) = _handleSalt(program, pos);


        // read program
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(77, success, field, uint256(_type));
        console.log(11111111111111);
        require(success, "decode key failed");
        require(field == 4, "not program");
        require(_type == ProtobufLib.WireType.LengthDelimited, "not program");

        (spawnedProgram, newPos) = _handleProgram(program, pos);
        console.logBytes(program[0: newPos]);
    }

    function _handleSpawn(bytes calldata program, uint64 pos)
    internal
    returns (uint64 newPos)
    {
        (uint64 pos, uint256 maxPos, uint256 networkId, IGateway.BridgeSecurity security, uint256 salt, bytes memory spawnedProgram) = _handleSpawnParams(program, pos);
        address bridgeAddress = IGateway(gatewayAddress).getBridge(networkId, security);
        // TODO The fund should be pulled by the Bridge or sent from here??? which is more secure??
        console.log("handle assets");
        bool success;
        uint64 field;
        (success, pos, field, ) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        require(field == 5, "not assets key");
        require(success, "decode key failed");

        address[] memory assetAddresses;
        uint256[] memory amounts;
        if (pos < maxPos) {
            (newPos, assetAddresses, amounts) = _handleAssets(program, pos, bridgeAddress);
        }
        IGateway(gatewayAddress).emitSpawn(origin.account, networkId, security, salt, spawnedProgram, assetAddresses, amounts);
    }

    function _replaceBytesByPosition(bytes memory payload, uint64 position, bytes memory s) internal returns (bytes memory) {
        bytes memory temp = new bytes(payload.length + s.length - 1);
        uint256 count = 0;
        for (uint256 i = 0; i < payload.length; i++) {
            if (i != position) {
                temp[i + count] = payload[i];
            } else {
                for (uint256 j = 0; j < s.length; j++) {
                    temp[i + count] = s[count];
                    count ++;
                }
                count -= 1;
            }
        }
        return temp;
    }

    function _handleCall(bytes calldata program, uint64 pos)
    internal
    returns (uint64 newPos)
    {
        // reading call instruction
        bool success;
        uint64 size;
        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(666, success, pos, size);
        uint256 maxPos = pos + size;
        require(success, "decode embedded message failed");

        uint64 field;
        ProtobufLib.WireType _type;
        (success, pos, field, _type) = ProtobufLib.decode_key(
            pos,
            program
        );
        console.logBytes(program[0 : pos]);
        console.log(77, success, field, uint256(_type));
        console.log(11111111111111);
        require(success, "decode key failed");
        require(field == 1, "not payloaded");

        (success, pos, size) = ProtobufLib.decode_embedded_message(
            pos,
            program
        );

        console.logBytes(program[0 : pos]);
        console.log(success, pos, size);
        require(success, "decode embedded message failed");
        bytes memory payload = program[pos : pos + size];
        bytes memory finalPayload;
        console.logBytes(payload);
        pos = pos + size;
        //bytes memory bindingValues;

        if (pos < maxPos) {
            console.logBytes(program[0 : pos]);
            console.log(success, pos, size);
            (success, pos, field, _type) = ProtobufLib.decode_key(
                pos,
                program
            );
            console.logBytes(program[0 : pos]);
            console.log(2323, success, field, uint256(_type));
            require(field == 2, "not bindings");
            require(success, "decode key failed");
            require(
                _type == ProtobufLib.WireType.LengthDelimited,
                "decode type is not embedded messages"
            );
            (success, pos, size) = ProtobufLib.decode_embedded_message(
                pos,
                program
            );
            console.logBytes(program[0 : pos]);
            console.log(999, success, pos, size);
            require(success, "decode embedded message failed");
            uint64 totalBindingsLength = pos + size;

            uint64 positionToRight;

            while (pos < totalBindingsLength) {
                uint64 position;
                bytes memory valueToReplace;
                (position, valueToReplace, pos) = _handleBinding(program, pos, payload);
                console.log("binding");
                console.logBytes(payload);
                console.log(position);
                console.logBytes(valueToReplace);
                console.log('position to right', positionToRight);
                payload = _replaceBytesByPosition(payload, position + positionToRight, valueToReplace);
                console.logBytes(payload);
                console.logBytes(valueToReplace);
                positionToRight += uint64(valueToReplace.length) - 1;
                console.log("replaced payload");
                console.logBytes(payload);
                console.logBytes(abi.encode(payload));
            }
        }
        address addr;

        //console.logBytes(payload[:20]);
        console.log("payload");
        //get the address from first bytes
        (addr) = abi.decode(payload, (address));
        console.log("sending transaction", addr);
        finalPayload = payload.slice(32, payload.length - 32);
        console.logBytes(finalPayload);
        console.logBytes(finalPayload);
        (bool succ, bytes memory result) = addr.call(finalPayload);
        console.logBytes(result);
        require(succ, "error calling target");
        newPos = pos;
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
        console.logBytes(program[0 : pos]);
        console.log(success, pos, size);
        require(success, "decode embedded message failed");
        uint64 totalInstructionsLength = pos + size;
        while (pos < totalInstructionsLength) {
            // reading instruction message
            (success, pos, field, _type) = ProtobufLib.decode_key(
                pos,
                program
            );
            console.logBytes(program[0 : pos]);
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
            console.logBytes(program[0 : pos]);
            console.log(44444, success, pos, size);
            require(success, "decode embedded message failed");

            uint64 instruction;
            (success, pos, instruction, _type) = ProtobufLib
            .decode_key(pos, program);
            console.logBytes(program[0 : pos]);
            console.log(5555, success, field, uint256(_type));
            require(success, "decode key failed");

            if (instruction == uint64(OPERATION.TRANSFER)) {
                pos = _handleTransfer(program, pos);
                console.log(pos, totalInstructionsLength);
            } else if (
                instruction == uint64(OPERATION.SPAWN)
            ) {
                pos = _handleSpawn(program, pos);
                console.log(pos, totalInstructionsLength);

            } else if (instruction == uint8(OPERATION.CALL)) {
                console.log(132312312312312);
                pos = _handleCall(program, pos);
                console.log(pos, totalInstructionsLength);
            }
        }
    }

    function createProgram() public returns (bytes memory program) {}
}
