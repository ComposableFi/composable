pragma solidity ^0.8.13;
pragma abicoder v2;

import "forge-std/console.sol";
import "forge-std/Test.sol";

import "../src/Gateway.sol";
import "../src/mocks/ERC20Mock.sol";
import "../utils/util.sol";
import "../src/Interpreter.sol";

contract test_Interpreter is Test {

    Utils internal utils;

    address internal bridge1;
    address internal bridge2;
    address internal user;
    address internal owner;
    uint256 internal defaultTokenAmount = 100000 * 1e18;
    ERC20Mock internal assetToken1;
    ERC20Mock internal assetToken2;
    Interpreter internal interpreter;

    fallback() external payable {}

    receive() external payable {}


    function setUp() public {
        utils = new Utils(vm);

        address payable[] memory users = utils.createUsers(6);


        owner = users[0];
        user = users[1];

        defaultTokenAmount = 10 * 10 ** 18;
        assetToken1 = new ERC20Mock("Asset Token 1", "AT1", owner, defaultTokenAmount);
        assetToken2 = new ERC20Mock("Asset Token 2", "AT2", owner, defaultTokenAmount);

        vm.prank(owner);
        interpreter = new Interpreter(bytes("test"), owner);
    }

    function testRunProgram() public {
        bytes memory input = hex"0a330a310a2f0a210a1fd317f7f4577a7b9d5a69df3c17a17871ee9a07cf36ef6efd71f7c56fddb6eb1a0a0a020801120412020864";
        vm.prank(owner);
        interpreter.interpret(input);
    }

}
