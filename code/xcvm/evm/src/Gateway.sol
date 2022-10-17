// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import "./Interpreter.sol";
import "./interfaces/IGateway.sol";

contract Gateway is Ownable, IGateway {
    mapping(uint256 => mapping(bytes => address)) public userInterpreter;

    mapping(address => Bridge) public bridgesInfo;
    // TODO ? do we have only one bridgge per network and security
    mapping(uint256 => mapping(BridgeSecurity => address)) public bridges;
    mapping(uint256 => address) public assets;

    event InstanceCreated(uint256 networkId, bytes user, address instance);

    event Spawn(
        bytes account,
        uint256 networkId,
        BridgeSecurity security,
        uint256 salt,
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

    function getAsset(uint256 assetId) external returns (address) {
        return assets[assetId];
    }

    function getBridge(uint256 networkId, BridgeSecurity security) external returns (address) {
        return bridges[networkId][security];
    }

    function registerAsset(address assetAddress, uint128 assetId) external onlyOwner {
        require(assetAddress != address(0), "Gateway: invalid address");
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
        require(bridges[networkId][security] == address(0), "Gateway: this type of bridge already registered");
        require(bridgeAddress != address(0), "Gateway: invalid address");
        require(bridgesInfo[bridgeAddress].security == BridgeSecurity(0), "Gateway: bridge already enabled");
        require(security != BridgeSecurity(0), "Gateway: should not disable bridge while registering bridge");
        bridgesInfo[bridgeAddress].security = security;
        bridgesInfo[bridgeAddress].networkId = networkId;
        bridges[networkId][security] = bridgeAddress;
    }

    function unregisterBridge(address bridgeAddress) external onlyOwner {
        require(bridgesInfo[bridgeAddress].security != BridgeSecurity(0), "Gateway: bridge already disabled");
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
            "Gateway: asset list size shuold be equal to amount list size"
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
    ) external payable onlyBridge {
        // a program is a result of spawn function, pull the assets from the bridge to the interpreter
        address payable interpreterAddress = _getOrCreateInterpreter(origin);
        _provisionAssets(interpreterAddress, _assets, _amounts);

        IInterpreter(interpreterAddress).interpret(program);
    }

    function createInterpreter(Origin memory origin) public {
        address interpreterAddress = userInterpreter[origin.networkId][origin.account];
        require(interpreterAddress == address(0), "Interpreter already exists");
        _getOrCreateInterpreter(origin);
    }

    function _getOrCreateInterpreter(Origin memory origin) private returns (address payable) {
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
        uint256 salt,
        bytes memory spawnedProgram,
        address[] memory assetAddresses,
        uint256[] memory amounts
    ) external {
        address payable interpreterAddress = _getOrCreateInterpreter(Origin(uint32(networkId), account));
        require(interpreterAddress == msg.sender, "Gateway: sender is not an interpreter address");
        emit Spawn(account, networkId, security, salt, spawnedProgram, assetAddresses, amounts);
    }
}
