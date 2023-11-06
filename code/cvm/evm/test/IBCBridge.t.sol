// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
pragma abicoder v2;

import "forge-std/console.sol";
import "forge-std/Test.sol";

import "../src/Router.sol";
import "../src/mocks/ERC20Mock.sol";
import "../utils/util.sol";
import "../src/interfaces/IInterpreter.sol";
import "../src/libraries/SDK.sol";
import "yui-ibc/core/25-handler/IBCHandler.sol";
import "yui-ibc/core/24-host/IBCHost.sol";
import "../src/IbcBridge.sol";

contract test_IBCBridge is Test {
    Utils internal utils;

    IBCBridge internal bridge;
    address internal user;
    address internal owner;
    address internal ibcHost;
    address internal ibcHandler;
    uint256 internal defaultTokenAmount = 100000 * 1e18;
    ERC20Mock internal assetToken1;
    ERC20Mock internal assetToken2;
    Router internal router;

    fallback() external payable {}

    receive() external payable {}

    function setUp() public {
        utils = new Utils(vm);
        address payable[] memory users = utils.createUsers(6);

        owner = users[0];
        ibcHost = users[1];
        ibcHandler = users[2];
        user = users[3];

        defaultTokenAmount = 10 * 10**18;
        assetToken1 = new ERC20Mock("Asset Token 1", "AT1", owner, defaultTokenAmount);
        assetToken2 = new ERC20Mock("Asset Token 2", "AT2", owner, defaultTokenAmount);

        vm.prank(owner);
        router = new Router();

        bridge = new IBCBridge(address(router), IBCHost(ibcHost), IBCHandler(ibcHandler));
    }

    function testInitParams() public {
        assertEq(router.owner(), owner);
        assertEq(bridge.routerAddress(), address(router));
    }

    function testEncodeAndDecodeIBCProgram(address routerAddress, uint128 networkId, bytes memory salt, bytes memory spawnedProgram) public {
        vm.assume(routerAddress != address(0));
        uint128[] memory assetIds; 
        uint256[] memory amounts;
        assetIds = new uint128[](1);
        assetIds[0] = 1;
        amounts = new uint256[](1);
        amounts[0] = 1 ether;
        bytes memory accountInBytes = abi.encodePacked(address(123));

        // transfer 1 ether to this contract to simulate the asset transfer
        vm.prank(owner);
        assetToken1.transfer(address(this), 1 ether);
        vm.mockCall(
            routerAddress,
            abi.encodeWithSelector(IRouter.getAsset.selector),
            abi.encode(address(assetToken1))
        );

        bytes memory ibcProgram = SDK.generateIBCSpawn(SDK.generateInterpreterOrigin(accountInBytes), 
            SDK.generateUserOrigin(accountInBytes, networkId), salt, spawnedProgram, assetIds, amounts);
        (   
            , 
            IRouter.Origin memory expectedOrigin, 
            bytes memory expectedSpawnedProgram, 
            bytes memory expectedSalt, 
            address[] memory expectedAssetAddresses, 
            uint256[] memory expectedAssetAmounts
         ) = SDK.decodeIBCSpawn(ibcProgram, routerAddress);

         assertEq(expectedOrigin.networkId, networkId);
         assertEq(expectedOrigin.account, accountInBytes);
         assertEq(expectedSpawnedProgram, spawnedProgram);
         assertEq(expectedSalt, salt);
         assertEq(expectedAssetAddresses[0], address(assetToken1));
         assertEq(expectedAssetAmounts[0], 1 ether);
         // asset sent to router
         assertEq(assetToken1.balanceOf(routerAddress), 1 ether);
    }
}
