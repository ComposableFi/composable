// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
pragma experimental ABIEncoderV2;

import "@openzeppelin/contracts/access/Ownable.sol";
import "@openzeppelin/contracts/token/ERC20/IERC20.sol";

import "./Interpreter.sol";
import "./interfaces/IRouter.sol";

contract Router is Ownable, IRouter {
    enum BridgeSecurity {
        Disabiled,
        Deterministic,
        Probabilistic,
        Optimistic
    }

    struct Bridge {
        uint256 chainId;
        BridgeSecurity security;
    }

    mapping(uint256 => mapping(bytes => address))
        public userInterpreter;

    mapping(address => Bridge) bridges;

    event InstanceCreated(
        uint256 networkId,
        bytes user,
        address instance
    );

    modifier onlyBridge() {
        require(bridges[msg.sender].security != BridgeSecurity(0));
        _;
    }

    constructor() {
        // enable trustless bridge;
    }

    function registerBridge(
        address bridgeAddress,
        BridgeSecurity security,
        uint256 chainId
    ) external onlyOwner {
        require(
            bridges[bridgeAddress].security == BridgeSecurity(0),
            "Gateway: bridge already enabled"
        );
        require(
            security != BridgeSecurity(0),
            "Gateway: should not disable bridge while registering bridge"
        );
        bridges[bridgeAddress].security = security;
        bridges[bridgeAddress].chainId = chainId;
    }

    function unregisterBridge(address bridgeAddress)
        external
        onlyOwner
    {
        require(
            bridges[bridgeAddress].security != BridgeSecurity(0),
            "Gateway: bridge already disabled"
        );
        bridges[bridgeAddress].security = BridgeSecurity(0);
        bridges[bridgeAddress].chainId = 0;
    }

    // TODO ? is the bridge who's gonna to provide internetwork assets transfer?
    function provisionAssets(
        Origin memory origin,
        address[] calldata erc20AssetList,
        uint256[] calldata amounts
    ) external payable onlyBridge {
        require(
            erc20AssetList.length == amounts.length,
            "Gateway: asset list size shuold be equal to amount list size"
        );
        address payable interpreterAddress = _getOrCreateInterpreter(origin);
        if (msg.value > 0) {
            bool sent = interpreterAddress.send(msg.value);
            require(
                sent,
                "Failed to send Ether"
            );
        }
        for (uint256 i = 0; i < erc20AssetList.length; i++) {
            IERC20(erc20AssetList[i]).transferFrom(
                msg.sender,
                interpreterAddress,
                amounts[i]
            );
        }
    }

    function runProgram(Origin memory origin, bytes calldata program)
        external
        onlyBridge
    {
        Interpreter(_getOrCreateInterpreter(origin))
            .interpretWithProtoBuff(program);
    }

    function _getOrCreateInterpreter(Origin memory origin)
        private
        returns (address payable)
    {
        address interpreterAddress = userInterpreter[
            origin.networkId
        ][origin.account];
        if (interpreterAddress == address(0)) {
            //interpreterAddress = address(new Interpreter(networkId, account));
            require(
                bridges[msg.sender].security ==
                    BridgeSecurity.Deterministic,
                "For creating a new interpreter, the sender should be a deterministic bridge"
            );
            interpreterAddress = address(1);
            userInterpreter[origin.networkId][
                origin.account
            ] = interpreterAddress;

            emit InstanceCreated(
                origin.networkId,
                origin.account,
                interpreterAddress
            );
        }
        return payable(interpreterAddress);
    }
}
