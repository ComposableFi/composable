// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
import "../libraries/SDK.sol";

contract SDKMock {
    function generateUint128(uint128 n) public pure returns (bytes memory u128) {
        return SDK.generateUint128(n);
    }

    function generateAbsolute(uint128 n) public pure returns (bytes memory absolute) {
        return SDK.generateAbsolute(n);
    }

    function generateRatio(uint128 nominator, uint128 denominator) public pure returns (bytes memory ratio) {
        return SDK.generateRatio(nominator, denominator);
    }

    function generateUnit(uint128 integer, bytes memory ratio) public pure returns (bytes memory unit) {
        return SDK.generateUnit(integer, ratio);
    }

    function generateBalanceByRatio(bytes memory ratio) public pure returns (bytes memory balance) {
        return SDK.generateBalanceByRatio(ratio);
    }

    function generateBalanceByAbsolute(bytes memory absolute) public pure returns (bytes memory balance) {
        return SDK.generateBalanceByAbsolute(absolute);
    }

    function generateBalanceByUnit(bytes memory _unit) public pure returns (bytes memory balance) {
        return SDK.generateBalanceByUnit(_unit);
    }

    function generateAccount(bytes memory _account) public pure returns (bytes memory account) {
        return SDK.generateAccount(_account);
    }

    function generateGlobalId(uint128 _globalId) public pure returns (bytes memory globalId) {
        return SDK.generateGlobalId(_globalId);
    }

    function generateLocalId(bytes memory _localId) public pure returns (bytes memory localId) {
        return SDK.generateLocalId(_localId);
    }

    function generateAssetIdByGlobalId(bytes memory _globalId) public pure returns (bytes memory assetId) {
        return SDK.generateAssetIdByGlobalId(_globalId);
    }

    function generateAssetIdByLocalId(bytes memory _localId) public pure returns (bytes memory assetId) {
        return SDK.generateAssetIdByLocalId(_localId);
    }

    function generateAsset(bytes memory _assetId, bytes memory _balance) public pure returns (bytes memory asset) {
        return SDK.generateAsset(_assetId, _balance);
    }

    function generateSelf(uint32 _self) public pure returns (bytes memory self) {
        return SDK.generateSelf(_self);
    }

    function generateRelayer(uint32 _relayer) public pure returns (bytes memory relayer) {
        return SDK.generateRelayer(_relayer);
    }

    function generateResult(uint32 _result) public pure returns (bytes memory result) {
        return SDK.generateResult(_result);
    }

    function generateAssetAmount(bytes memory assetId, bytes memory ratio) public pure returns (bytes memory assetAmount) {
        return SDK.generateAssetAmount(assetId, ratio);
    }

    function generateBindingValueByAssetAmount(bytes memory _assetAmount) public pure returns (bytes memory bindingValue) {
        return SDK.generateBindingValueByAssetAmount(_assetAmount);
    }

    function generateBindingValueByGlobalId(bytes memory _globalId) public pure returns (bytes memory bindingValue) {
        return SDK.generateBindingValueByGlobalId(_globalId);
    }

    function generateBinding(uint32 position, bytes memory _bindingValue) public pure returns (bytes memory binding) {
        return SDK.generateBinding(position, _bindingValue);
    }

    function generateBindings(bytes[] memory _bindings) public pure returns (bytes memory bindings) {
        return SDK.generateBindings(_bindings);
    }

    function generateTransferByAccount(bytes memory _account, bytes[] memory _assets) public pure returns (bytes memory transfer) {
        return SDK.generateTransferByAccount(_account, _assets);
    }

    function generateSalt(bytes memory _salt) public pure returns (bytes memory salt) {
        return SDK.generateSalt(_salt);
    }

    function generateNetwork(uint128 _network) public pure returns (bytes memory network) {
        return SDK.generateNetwork(_network);
    }

    function generateSpawn(
        bytes memory _network,
        int32 _security,
        bytes memory _salt,
        bytes memory _spawnedProgram,
        bytes[] memory _assets
    ) public pure returns (bytes memory spawn) {
        return SDK.generateSpawn(_network, _security, _salt, _spawnedProgram, _assets);
    }

    function generateCall(
        bytes memory _payload,
        bytes memory _bindings
    ) public pure returns (bytes memory call) {
        return SDK.generateCall(_payload, _bindings);
    }

    function generateInstructionByTransfer(
        bytes memory _transfer
    ) public pure returns (bytes memory instruction) {
        return SDK.generateInstructionByTransfer(_transfer);
    }

    function generateInstructionByCall(
        bytes memory _call
    ) public pure returns (bytes memory instruction) {
        return SDK.generateInstructionByCall(_call);
    }

    function generateInstructionBySpawn(
        bytes memory _spawn
    ) public pure returns (bytes memory instruction) {
        return SDK.generateInstructionBySpawn(_spawn);
    }

    function generateInstructionByQuery(
        bytes memory _query
    ) public pure returns (bytes memory instruction) {
        return SDK.generateInstructionByQuery(_query);
    }

    function generateInstructions(bytes[] memory _instructions) public pure returns (bytes memory instructions) {
        return SDK.generateInstructions(_instructions);
    }

    function generateProgram(
        bytes memory _tag,
        bytes memory _instructions
    ) public pure returns (bytes memory program) {
        return SDK.generateProgram(_tag, _instructions);
    }
}