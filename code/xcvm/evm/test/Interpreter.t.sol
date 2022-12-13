// SPDX-License-Identifier: MIT
pragma solidity ^0.8.14;
pragma abicoder v2;

import "forge-std/console.sol";
import "forge-std/Test.sol";

import "../src/Router.sol";
import "../src/mocks/ERC20Mock.sol";
import "../utils/util.sol";
import "../src/Interpreter.sol";
import "../src/interfaces/IRouter.sol";

contract test_Interpreter is Test {
    Utils internal utils;

    address internal bridge1;
    address internal bridge2;
    address internal user;
    address internal owner;
    uint256 internal defaultTokenAmount = 100000 * 1e18;
    address internal interpreterAddress;
    ERC20Mock internal assetToken1;
    ERC20Mock internal assetToken2;
    Interpreter internal interpreter;
    Router internal router;

    fallback() external payable {}

    receive() external payable {}

    function setUp() public {
        utils = new Utils(vm);

        address payable[] memory users = utils.createUsers(6);

        owner = users[0];
        user = users[1];

        router = new Router();
        //register owner as the bridge
        router.registerBridge(user, IRouter.BridgeSecurity(1), 1);

        vm.prank(user);
        router.createInterpreter(IRouter.Origin({networkId: 1, account: abi.encodePacked(owner)}), "salt");
        interpreterAddress = router.userInterpreter(1, abi.encodePacked(owner), "salt");
        console.log(interpreterAddress);
        ERC20Mock erc20 = new ERC20Mock("test", "test", interpreterAddress, 100 ether);
        router.registerAsset(address(erc20), 1);

        vm.prank(owner);
        interpreter = new Interpreter(IRouter.Origin({account: bytes("test"), networkId: 1}), owner, "salt");
    }

    function testRunProgram(address relayerAddress) private {
        //bytes memory input = hex"0a330a310a2f0a210a1fd317f7f4577a7b9d5a69df3c17a17871ee9a07cf36ef6efd71f7c56fddb6eb1a0a0a020801120412020864";

        bytes
            memory input = hex"0a3a0a381a360a1a01a9059cbb70997970c51812dc3a010c7d01b50e0d17dc79c80212180a08080012042a0208010a0c081912082206120408c0843d";
        vm.prank(address(router));
        Interpreter(payable(interpreterAddress)).interpret(input, relayerAddress);
    }
}
