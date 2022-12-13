// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
pragma abicoder v2;

import "forge-std/console.sol";
import "forge-std/Test.sol";

import "../src/Router.sol";
import "../src/mocks/ERC20Mock.sol";
import "../utils/util.sol";
import "../src/interfaces/IRouter.sol";
import "../src/interfaces/IInterpreter.sol";

contract test_Router is Test {
    Utils internal utils;

    address internal bridge1;
    address internal bridge2;
    address internal user;
    address internal owner;
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
        bridge1 = users[1];
        bridge2 = users[2];
        user = users[3];

        defaultTokenAmount = 10 * 10**18;
        assetToken1 = new ERC20Mock("Asset Token 1", "AT1", owner, defaultTokenAmount);
        assetToken2 = new ERC20Mock("Asset Token 2", "AT2", owner, defaultTokenAmount);

        vm.prank(owner);
        router = new Router();
    }

    function testInitParams() public {
        assertEq(router.owner(), owner);
    }

    function testRegisterBridgeFailed() public {
        vm.expectRevert("Ownable: caller is not the owner");
        router.registerBridge(bridge1, IRouter.BridgeSecurity(1), 1);
        vm.startPrank(owner);
        vm.expectRevert("Router: invalid address");
        router.registerBridge(address(0), IRouter.BridgeSecurity(1), 1);
        vm.expectRevert("Router: should not disable bridge while registering bridge");
        router.registerBridge(address(1), IRouter.BridgeSecurity(0), 1);
        vm.stopPrank();
    }

    function testRegisterBridge() public {
        vm.prank(owner);
        router.registerBridge(bridge1, IRouter.BridgeSecurity(1), 1);
    }

    function testUnregisterBridgeFailed() public {
        vm.expectRevert("Ownable: caller is not the owner");
        router.unregisterBridge(bridge1);
    }

    function testUnregisterBridge() public {
        vm.prank(owner);
        router.registerBridge(bridge1, IRouter.BridgeSecurity(1), 1);
        vm.prank(owner);
        router.unregisterBridge(bridge1);
    }

    function testRegisterAssetAddressFailed(uint128 assetId) public {
        vm.expectRevert("Ownable: caller is not the owner");
        router.registerAsset(address(assetToken1), assetId);
        vm.startPrank(owner);
        vm.expectRevert("Router: invalid address");
        router.registerAsset(address(0), assetId);
        vm.stopPrank();
    }

    function testRegisterAssetAddress(uint128 assetId) public {
        vm.prank(owner);
        router.registerAsset(address(assetToken1), assetId);
        assertEq(router.assets(assetId), address(assetToken1));
    }

    function testUnregisterAssetAddress(uint128 assetId) public {
        vm.startPrank(owner);
        router.registerAsset(address(assetToken1), assetId);
        router.unregisterAsset(assetId);
        vm.stopPrank();
        assertEq(router.assets(assetId), address(0));
    }


    function testCreateInterpreter(uint128 networkId, bytes memory account, bytes memory salt) public {
        vm.prank(owner);
        router.registerBridge(bridge1, IRouter.BridgeSecurity(1), 1);

        vm.prank(bridge1);
        IRouter.Origin memory origin = IRouter.Origin(networkId, account);
        address payable interpreterAddress = router.createInterpreter(origin, salt);
        assertTrue(interpreterAddress != address(0));
    }

    function testCreateInterpreterWithSameSalt(uint128 networkId, bytes memory account, bytes memory salt) public {
        vm.prank(owner);
        router.registerBridge(bridge1, IRouter.BridgeSecurity(1), 1);

        vm.prank(bridge1);
        IRouter.Origin memory origin = IRouter.Origin(networkId, account);
        address payable interpreterAddress = router.createInterpreter(origin, salt);
        assertTrue(interpreterAddress != address(0));

        vm.prank(bridge1);
        vm.expectRevert('Interpreter already exists');
        router.createInterpreter(origin, salt);
    }

    function testCreateInterpreterWithDifferentSalt(uint128 networkId, bytes memory account, bytes memory salt, bytes memory salt2) public {
        vm.prank(owner);
        router.registerBridge(bridge1, IRouter.BridgeSecurity(1), 1);

        vm.prank(bridge1);
        IRouter.Origin memory origin = IRouter.Origin(networkId, account);
        address payable interpreterAddress = router.createInterpreter(origin, salt);
        assertTrue(interpreterAddress != address(0));

        vm.prank(bridge1);
        address payable interpreterAddress2 = router.createInterpreter(origin, salt2);
        assertTrue(interpreterAddress2 != address(0));

        assertTrue(interpreterAddress != interpreterAddress2);

    }
}
