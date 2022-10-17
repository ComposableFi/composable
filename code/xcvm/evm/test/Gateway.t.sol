pragma solidity ^0.8.13;
pragma abicoder v2;

import "forge-std/console.sol";
import "forge-std/Test.sol";

import "../src/Gateway.sol";
import "../src/mocks/ERC20Mock.sol";
import "../utils/util.sol";
import "../src/interfaces/IInterpreter.sol";

contract test_Gateway is Test {
    Utils internal utils;

    address internal bridge1;
    address internal bridge2;
    address internal user;
    address internal owner;
    uint256 internal defaultTokenAmount = 100000 * 1e18;
    ERC20Mock internal assetToken1;
    ERC20Mock internal assetToken2;
    Gateway internal gateway;

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
        gateway = new Gateway();
    }

    function testInitParams() public {
        assertEq(gateway.owner(), owner);
    }

    function testRegisterBridgeFailed() public {
        vm.expectRevert("Ownable: caller is not the owner");
        gateway.registerBridge(bridge1, Gateway.BridgeSecurity(1), 1);
        vm.startPrank(owner);
        vm.expectRevert("Gateway: invalid address");
        gateway.registerBridge(address(0), Gateway.BridgeSecurity(1), 1);
        vm.expectRevert("Gateway: should not disable bridge while registering bridge");
        gateway.registerBridge(address(1), Gateway.BridgeSecurity(0), 1);
        vm.stopPrank();
    }

    function testRegisterBridge() public {
        vm.prank(owner);
        gateway.registerBridge(bridge1, Gateway.BridgeSecurity(1), 1);
    }

    function testUnregisterBrigdgeFailed() public {
        vm.expectRevert("Ownable: caller is not the owner");
        gateway.unregisterBridge(bridge1);
    }

    function testUnregisterBridge() public {
        vm.prank(owner);
        gateway.registerBridge(bridge1, Gateway.BridgeSecurity(1), 1);
        vm.prank(owner);
        gateway.unregisterBridge(bridge1);
    }

    function testRegisterAssetAddressFailed(uint128 assetId) public {
        vm.expectRevert("Ownable: caller is not the owner");
        gateway.registerAsset(address(assetToken1), assetId);
        vm.startPrank(owner);
        vm.expectRevert("Gateway: invalid address");
        gateway.registerAsset(address(0), assetId);
        vm.stopPrank();
    }

    function testRegisterAseetAddress(uint128 assetId) public {
        vm.prank(owner);
        gateway.registerAsset(address(assetToken1), assetId);
        assertEq(gateway.assets(assetId), address(assetToken1));
    }

    function testUnregisterAssetAddress(uint128 assetId) public {
        vm.startPrank(owner);
        gateway.registerAsset(address(assetToken1), assetId);
        gateway.unregisterAsset(assetId);
        vm.stopPrank();
        assertEq(gateway.assets(assetId), address(0));
    }
}
