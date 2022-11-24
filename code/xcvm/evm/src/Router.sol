// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import "./Interpreter.sol";
import "./interfaces/IRouter.sol";
import "./interfaces/IIbcBridge.sol";

contract Router is Ownable, IRouter {
    mapping(uint256 => mapping(bytes => address)) public userInterpreter;

    mapping(address => Bridge) public bridgesInfo;
    // TODO ? do we have only one bridge per network and security
    mapping(uint256 => mapping(BridgeSecurity => address)) public bridges;
    mapping(uint256 => address) public assets;

    event InstanceCreated(uint256 networkId, bytes user, address instance);

    event AddOwners(address sender, uint256 networkId, BridgeSecurity security, address[] owners);

    event RemoveOwners(address sender, uint256 networkId, BridgeSecurity security, address[] owners);

    event Spawn(
        bytes account,
        uint256 networkId,
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

    function getBridge(uint256 networkId, BridgeSecurity security) external view returns (address) {
        return bridges[networkId][security];
    }

    function registerAsset(address assetAddress, uint128 assetId) external onlyOwner {
        require(assetAddress != address(0), "Router: invalid address");
        assets[assetId] = assetAddress;
    }

    function unregisterAsset(uint128 assetId) external onlyOwner {
        delete assets[assetId];
    }

    function registerBridge(
        address bridgeAddress,
        BridgeSecurity security,
        uint256 networkId
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
        bytes memory program,
        address[] memory _assets,
        uint256[] memory _amounts
    ) public override payable onlyBridge returns (bool){
        // a program is a result of spawn function, pull the assets from the bridge to the interpreter
        address payable interpreterAddress = getOrCreateInterpreter(origin);
        _provisionAssets(interpreterAddress, _assets, _amounts);

        IInterpreter(interpreterAddress).interpret(program, msg.sender);
        return true;
    }

    function createInterpreter(Origin memory origin) public {
        address interpreterAddress = userInterpreter[origin.networkId][origin.account];
        require(interpreterAddress == address(0), "Interpreter already exists");
        getOrCreateInterpreter(origin);
    }

    function getOrCreateInterpreter(Origin memory origin) public returns (address payable) {
        address interpreterAddress = userInterpreter[origin.networkId][origin.account];
        if (interpreterAddress == address(0)) {
            //interpreterAddress = address(new Interpreter(networkId, account));
            require(
                bridgesInfo[msg.sender].security == BridgeSecurity.Deterministic,
                "For creating a new interpreter, the sender should be a deterministic bridge"
            );
            interpreterAddress = address(new Interpreter(origin, address(this)));
            userInterpreter[origin.networkId][origin.account] = interpreterAddress;

            emit InstanceCreated(origin.networkId, origin.account, interpreterAddress);
        }
        return payable(interpreterAddress);
    }

    function emitSpawn(
        bytes memory account,
        uint256 networkId,
        BridgeSecurity security,
        bytes memory salt,
        bytes memory spawnedProgram,
        address[] memory assetAddresses,
        uint128[] memory assetIds,
        uint256[] memory amounts
    ) override external {
        address payable interpreterAddress = getOrCreateInterpreter(Origin(uint32(networkId), account));
        require(interpreterAddress == msg.sender, "Router: sender is not an interpreter address");
        emit Spawn(account, networkId, security, salt, spawnedProgram, assetAddresses, amounts);
        if (security == BridgeSecurity.Deterministic) {
            // send through ibc
            IIbcBridge(bridges[networkId][security]).sendProgram(account, uint32(networkId), salt, spawnedProgram, assetIds, amounts);
        }
    }
}
