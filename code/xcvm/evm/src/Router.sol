// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
pragma experimental ABIEncoderV2;

import "openzeppelin-contracts/access/Ownable.sol";
import "openzeppelin-contracts/token/ERC20/IERC20.sol";

import "./Interpreter.sol";
import "./interfaces/IRouter.sol";
import "./interfaces/IIbcBridge.sol";

contract Router is Ownable, IRouter {
    // network => account => salt
    mapping(uint128 => mapping(bytes => mapping(bytes => address))) public userInterpreter;

    mapping(address => Bridge) public bridgesInfo;
    // TODO ? do we have only one bridge per network and security
    mapping(uint128 => mapping(BridgeSecurity => address)) public bridges;
    mapping(uint256 => address) public assets;
    mapping(address => uint256) public assetIds;

    event InstanceCreated(uint128 networkId, bytes user, bytes salt, address instance);

    event AddOwners(address sender, uint128 networkId, BridgeSecurity security, address[] owners);

    event RemoveOwners(address sender, uint128 networkId, BridgeSecurity security, address[] owners);

    event Spawn(
        bytes account,
        uint128 networkId,
        BridgeSecurity security,
        bytes salt,
        bytes spawnedProgram,
        address[] assetAddresses,
        uint256[] amounts
    );

    modifier onlyBridge() {
        require(bridgesInfo[msg.sender].security != BridgeSecurity(0));
        _;
    }

    constructor() {
        // enable trustless bridge;
    }

    function getAsset(uint256 assetId) external view returns (address) {
        return assets[assetId];
    }

    function getAssetIdByLocalId(address asset) external view returns(uint256) {
        return assetIds[asset];
    }

    function getBridge(uint128 networkId, BridgeSecurity security) external view returns (address) {
        return bridges[networkId][security];
    }

    function registerAsset(address assetAddress, uint128 assetId) external onlyOwner {
        require(assetAddress != address(0), "Router: invalid address");
        assets[assetId] = assetAddress;
        assetIds[assetAddress] = assetId;
    }

    function unregisterAsset(uint128 assetId) external onlyOwner {
        delete assetIds[assets[assetId]];
        delete assets[assetId];
    }

    function registerBridge(
        address bridgeAddress,
        BridgeSecurity security,
        uint128 networkId
    ) external onlyOwner {
        require(bridges[networkId][security] == address(0), "Router: this type of bridge already registered");
        require(bridgeAddress != address(0), "Router: invalid address");
        require(bridgesInfo[bridgeAddress].security == BridgeSecurity(0), "Router: bridge already enabled");
        require(security != BridgeSecurity(0), "Router: should not disable bridge while registering bridge");
        bridgesInfo[bridgeAddress].security = security;
        bridgesInfo[bridgeAddress].networkId = networkId;
        bridges[networkId][security] = bridgeAddress;
    }

    function unregisterBridge(address bridgeAddress) external onlyOwner {
        require(bridgesInfo[bridgeAddress].security != BridgeSecurity(0), "Router: bridge already disabled");
        delete bridges[bridgesInfo[bridgeAddress].networkId][bridgesInfo[bridgeAddress].security];
        bridgesInfo[bridgeAddress].security = BridgeSecurity(0);
        bridgesInfo[bridgeAddress].networkId = 0;
    }

    //// TODO ? is the bridge who's gonna to provide internetwork assets transfer?
    function _provisionAssets(
        address payable interpreterAddress,
        address[] memory erc20AssetList,
        uint256[] memory amounts
    ) internal {
        require(
            erc20AssetList.length == amounts.length,
            "Router: asset list size should be equal to amount list size"
        );
        if (msg.value > 0) {
            bool sent = interpreterAddress.send(msg.value);
            require(sent, "Failed to send Ether");
        }
        for (uint256 i = 0; i < erc20AssetList.length; i++) {
            IERC20(erc20AssetList[i]).transferFrom(msg.sender, interpreterAddress, amounts[i]);
        }
    }

    function runProgram(
        Origin memory origin,
        bytes memory salt,
        bytes memory program,
        address[] memory _assets,
        uint256[] memory _amounts
    ) public override payable onlyBridge returns (bool){
        // a program is a result of spawn function, pull the assets from the bridge to the interpreter
        address payable interpreterAddress = getOrCreateInterpreter(origin, salt);
        _provisionAssets(interpreterAddress, _assets, _amounts);

        IInterpreter(interpreterAddress).interpret(program, msg.sender);
        return true;
    }

    function createInterpreter(Origin memory origin, bytes memory salt) public returns(address payable) {
        address interpreterAddress = userInterpreter[origin.networkId][origin.account][salt];
        require(interpreterAddress == address(0), "Interpreter already exists");
        return getOrCreateInterpreter(origin, salt);
    }

    function getOrCreateInterpreter(Origin memory origin, bytes memory salt) public returns (address payable) {
        address interpreterAddress = userInterpreter[origin.networkId][origin.account][salt];
        if (interpreterAddress == address(0)) {
            //interpreterAddress = address(new Interpreter(networkId, account));
            require(
                bridgesInfo[msg.sender].security == BridgeSecurity.Deterministic,
                "For creating a new interpreter, the sender should be a deterministic bridge"
            );
            interpreterAddress = address(new Interpreter(origin, address(this), salt));
            userInterpreter[origin.networkId][origin.account][salt] = interpreterAddress;

            emit InstanceCreated(origin.networkId, origin.account, salt, interpreterAddress);
        }
        return payable(interpreterAddress);
    }

    function emitSpawn(
        bytes memory account,
        uint128 networkId,
        BridgeSecurity security,
        bytes memory salt,
        bytes memory spawnedProgram,
        address[] memory assetAddresses,
        uint128[] memory _assetIds,
        uint256[] memory amounts
    ) override external {
        address payable interpreterAddress = getOrCreateInterpreter(Origin(networkId, account), IInterpreter(msg.sender).salt());
        require(interpreterAddress == msg.sender, "Router: sender is not an interpreter address");
        emit Spawn(account, networkId, security, salt, spawnedProgram, assetAddresses, amounts);
        if (security == BridgeSecurity.Deterministic) {
            // send through ibc
            IIbcBridge(bridges[networkId][security]).sendProgram(account, networkId, salt, spawnedProgram, _assetIds, amounts);
        }
    }
}
