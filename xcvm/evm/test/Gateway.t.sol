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

        defaultTokenAmount = 10 * 10 ** 18;
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


/*
    function signPermitForUser(address tokenAddress, address to, uint256 value, uint256 deadline) public returns (uint8 v, bytes32 r, bytes32 s){
        ERC20Mock token = ERC20Mock(tokenAddress);
        bytes32 structHash = keccak256(abi.encode(_PERMIT_TYPEHASH, user, to, value, token.nonces(user), deadline));
        bytes32 hash = ECDSA.toTypedDataHash(token.DOMAIN_SEPARATOR(), structHash);
        (v, r, s) = vm.sign(userPrivateKey, hash);
    }

    function testProvideLiquidity(uint256 mDucaIn, uint256 lpDucaOut, uint256 deadline) public {
        vm.assume(mDucaIn > 0);
        vm.assume(mDucaIn < defaultTokenAmount);
        vm.assume(deadline > block.timestamp + 100);

        // approve
        (uint8 v, bytes32 r, bytes32 s) = signPermitForUser(address(mDuca), address(stabilityPool), mDucaIn, deadline);

        vm.startPrank(validatorAddress);
        vm.expectEmit(true, true, false, true);
        emit LiquidityProvided(validatorAddress, user, mDucaIn, lpDucaOut);
        stabilityPool.provideLiquidity(user, mDucaIn, lpDucaOut, PermitSignature(deadline, v, r, s));
        assertEq(stabilityPool.balanceOf(user), lpDucaOut);
        vm.stopPrank();
    }

    function testRemoveLiquidity(uint256 mDucaIn, uint256 lpDucaOut, uint256 deadline) public {
        vm.assume(mDucaIn < defaultTokenAmount);
        vm.assume(deadline > block.timestamp + 100);
        //approve
        (uint8 v, bytes32 r, bytes32 s) = signPermitForUser(address(mDuca), address(stabilityPool), mDucaIn, deadline);

        vm.startPrank(validatorAddress);
        stabilityPool.provideLiquidity(user, mDucaIn, lpDucaOut, PermitSignature(deadline, v, r, s));
        vm.expectEmit(true, true, false, true);
        emit LiquidityWithdrawn(validatorAddress, user, lpDucaOut, mDucaIn);
        stabilityPool.withdrawLiquidity(user, lpDucaOut, mDucaIn);
        assertEq(stabilityPool.balanceOf(user), 0);
    }

    function testSwapDuca(uint256 ducaIn, uint256 mDucaOut, uint256 deadline) public {
        vm.assume(ducaIn < defaultTokenAmount / 2 && ducaIn > 0);
        vm.assume(deadline > block.timestamp + 100);
        vm.assume(mDucaOut > 0 && mDucaOut < defaultTokenAmount);

        //approve
        (uint8 v, bytes32 r, bytes32 s) = signPermitForUser(address(duca), address(stabilityPool), ducaIn, deadline);
        (uint8 v2, bytes32 r2, bytes32 s2) = signPermitForUser(address(mDuca), address(stabilityPool), mDucaOut, deadline);

        vm.prank(user);
        mDuca.transfer(address(stabilityPool), mDucaOut);

        vm.startPrank(validatorAddress);
        stabilityPool.swapDuca(user, ducaIn, mDucaOut, PermitSignature(deadline, v, r, s), 0);
        // add liquidity
        stabilityPool.provideLiquidity(user, mDucaOut, 1, PermitSignature(deadline, v2, r2, s2));
        (v, r, s) = signPermitForUser(address(duca), address(stabilityPool), ducaIn, deadline);
        vm.expectEmit(true, true, false, true);
        emit DucaSwapped(validatorAddress, user, ducaIn, mDucaOut, 0);
        uint256 oldBalance = mDuca.balanceOf(user);
        stabilityPool.swapDuca(user, ducaIn, mDucaOut, PermitSignature(deadline, v, r, s), 0);
        assertEq(mDuca.balanceOf(feeAddress), 0);
        assertEq(mDuca.balanceOf(user), oldBalance + mDucaOut);
        vm.stopPrank();
    }

    function testSwapDucaWithFee(uint256 ducaIn, uint256 mDucaOut, uint256 fee, uint256 deadline) public {
        vm.assume(ducaIn < defaultTokenAmount && ducaIn > 0);
        vm.assume(deadline > block.timestamp + 100);
        vm.assume(mDucaOut > 0 && mDucaOut < defaultTokenAmount);
        vm.assume(fee < defaultTokenAmount - mDucaOut);
        (uint8 v, bytes32 r, bytes32 s) = signPermitForUser(address(duca), address(stabilityPool), ducaIn, deadline);
        (uint8 v2, bytes32 r2, bytes32 s2) = signPermitForUser(address(mDuca), address(stabilityPool), mDucaOut + fee, deadline);
        vm.startPrank(validatorAddress);
        stabilityPool.provideLiquidity(user, mDucaOut + fee, 1, PermitSignature(deadline, v2, r2, s2));
        vm.expectEmit(true, true, false, true);
        emit DucaSwapped(validatorAddress, user, ducaIn, mDucaOut, fee);
        uint256 oldBalance = mDuca.balanceOf(user);
        stabilityPool.swapDuca(user, ducaIn, mDucaOut, PermitSignature(deadline, v, r, s), fee);
        assertEq(mDuca.balanceOf(user), oldBalance + mDucaOut);
        assertEq(mDuca.balanceOf(feeAddress), fee);
        vm.stopPrank();
    }

    function testSwapMDucaPoolWithFee(uint256 mDucaIn, uint256 ducaOut, uint256 fee, uint256 deadline) public {
        vm.assume(mDucaIn < defaultTokenAmount && mDucaIn > 0);
        vm.assume(deadline > block.timestamp + 100);
        vm.assume(ducaOut > 0 && ducaOut < defaultTokenAmount);
        vm.assume(fee < defaultTokenAmount - mDucaIn);
        vm.prank(user);
        (uint8 v, bytes32 r, bytes32 s) = signPermitForUser(address(mDuca), address(stabilityPool), mDucaIn + fee, deadline);
        // add duca into stability pool
        vm.prank(user);
        duca.transfer(address(stabilityPool), ducaOut);
        vm.startPrank(validatorAddress);
        uint256 oldBalance = duca.balanceOf(user);
        vm.expectEmit(true, true, false, true);
        emit MDucaSwapped(validatorAddress, user, mDucaIn, ducaOut, fee);
        stabilityPool.swapMDuca(user, mDucaIn , ducaOut, PermitSignature(deadline, v, r, s), fee);
        assertEq(mDuca.balanceOf(feeAddress), fee);
        assertEq(duca.balanceOf(user), oldBalance + ducaOut);
        vm.stopPrank();
    }

    function testSwapMDucaUsingDucaInStabilityPool(uint256 mDucaIn, uint256 ducaOut, uint256 deadline) public {
        vm.assume(mDucaIn < defaultTokenAmount && mDucaIn > 0);
        vm.assume(deadline > block.timestamp + 100);
        vm.assume(ducaOut > 0 && ducaOut < defaultTokenAmount);
        (uint8 v, bytes32 r, bytes32 s) = signPermitForUser(address(mDuca), address(stabilityPool), mDucaIn, deadline);
        // add duca into stability pool
        vm.prank(user);
        duca.transfer(address(stabilityPool), ducaOut);
        vm.startPrank(validatorAddress);
        vm.expectEmit(true, true, false, true);
        emit MDucaSwapped(validatorAddress, user, mDucaIn, ducaOut, 0);
        uint256 oldBalance = duca.balanceOf(user);
        stabilityPool.swapMDuca(user, mDucaIn, ducaOut, PermitSignature(deadline, v, r, s), 0);
        assertEq(mDuca.balanceOf(feeAddress), 0);
        assertEq(duca.balanceOf(user), oldBalance + ducaOut);
        vm.stopPrank();
    }


    function testSwapMDucaMintingDucaFromTheMint(uint256 mDucaIn, uint256 ducaOut, uint256 deadline) public {
        vm.assume(mDucaIn < defaultTokenAmount && mDucaIn > 0);
        vm.assume(deadline > block.timestamp + 100);
        vm.assume(ducaOut > 0 && ducaOut < defaultTokenAmount);
        vm.prank(user);
        (uint8 v, bytes32 r, bytes32 s) = signPermitForUser(address(mDuca), address(stabilityPool), mDucaIn, deadline);
        vm.startPrank(validatorAddress);
        vm.expectEmit(true, true, false, true);
        emit MDucaSwapped(validatorAddress, user, mDucaIn, ducaOut, 0);
        uint256 oldBalance = duca.balanceOf(user);
        stabilityPool.swapMDuca(user, mDucaIn, ducaOut, PermitSignature(deadline, v, r, s), 0);
        assertEq(mDuca.balanceOf(feeAddress), 0);
        // the mint is mocked so no token is added to user
        assertEq(duca.balanceOf(user), 0 + oldBalance);
        vm.stopPrank();
    }

    function testSwapMDucaUsingRemainingDucaAndMintingNew(uint256 mDucaIn, uint256 ducaOut, uint256 deadline) public {
        vm.assume(mDucaIn < defaultTokenAmount && mDucaIn > 0);
        mDuca.approve(address(stabilityPool), mDucaIn);
        vm.assume(deadline > block.timestamp + 100);
        vm.assume(ducaOut > 0 && ducaOut < defaultTokenAmount);
        (uint8 v, bytes32 r, bytes32 s) = signPermitForUser(address(mDuca), address(stabilityPool), mDucaIn, deadline);
        // add part of ducaOut into stability pool
        vm.prank(user);
        duca.transfer(address(stabilityPool), ducaOut / 2);
        vm.startPrank(validatorAddress);
        vm.expectEmit(true, true, false, true);
        uint256 oldBalance = duca.balanceOf(user);
        assertEq(mDuca.balanceOf(feeAddress), 0);
        emit MDucaSwapped(validatorAddress, user, mDucaIn, ducaOut, 0);
        stabilityPool.swapMDuca(user, mDucaIn, ducaOut, PermitSignature(deadline, v, r, s), 0);
        // the mint is mocked so only the sent tokens is added to user
        assertEq(duca.balanceOf(user), oldBalance + ducaOut / 2);
        vm.stopPrank();
    }

    function testStabilityPoolSetter() public {
        vm.expectRevert('AccessModifiers: not DAOAddress or foundationAddress');
        stabilityPool.setDucaAddress(user);
        vm.expectRevert('AccessModifiers: not DAOAddress or foundationAddress');
        stabilityPool.setReserveAddress(user);
        vm.startPrank(daoAddress);
        stabilityPool.setDucaAddress(user);
        stabilityPool.setReserveAddress(user);
        vm.stopPrank();
        assertEq(stabilityPool.ducaAddress(), user);
        assertEq(stabilityPool.reserveAddress(), user);
    }

    function testERC2612Permit(uint256 allowedAmount, uint256 deadline) public {
        vm.assume(allowedAmount > 0);
        assertEq(stabilityPool.nonces(user), 0);
        vm.assume(deadline > block.timestamp + 100);

        bytes32 structHash = keccak256(abi.encode(_PERMIT_TYPEHASH, user, daoAddress, allowedAmount, stabilityPool.nonces(user), deadline));
        bytes32 hash = ECDSA.toTypedDataHash(stabilityPool.DOMAIN_SEPARATOR(), structHash);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPrivateKey, hash);
        address signer = ECDSAUpgradeable.recover(hash, v, r, s);


        assertEq(user, signer);
        vm.expectEmit(true, true, false, true);
        emit Approval(user, daoAddress, allowedAmount);
        stabilityPool.permit(user, daoAddress, allowedAmount, deadline, v, r, s);
    }

    function testERC2612PermitDeadlineExpired(uint256 allowedAmount, uint256 deadline) public {
        vm.assume(allowedAmount > 0);
        vm.assume(deadline < block.timestamp);

        bytes32 structHash = keccak256(abi.encode(_PERMIT_TYPEHASH, user, daoAddress, allowedAmount, stabilityPool.nonces(user), deadline));
        bytes32 hash = ECDSA.toTypedDataHash(stabilityPool.DOMAIN_SEPARATOR(), structHash);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPrivateKey, hash);

        vm.expectRevert("ERC20Permit: expired deadline");
        stabilityPool.permit(user, daoAddress, allowedAmount, deadline, v, r, s);
    }


    function testERC2612PermitInvalidHash(uint256 allowedAmount, uint256 deadline) public {
        vm.assume(allowedAmount > 0);
        vm.assume(deadline > block.timestamp + 100);
        bytes32 structHash = keccak256(abi.encode(_PERMIT_TYPEHASH, user, daoAddress, allowedAmount, stabilityPool.nonces(user), deadline));
        bytes32 hash = ECDSA.toTypedDataHash(stabilityPool.DOMAIN_SEPARATOR(), structHash);
        (uint8 v, bytes32 r, bytes32 s) = vm.sign(userPrivateKey, hash);

        vm.expectRevert("ERC20Permit: invalid signature");
        stabilityPool.permit(user, daoAddress, allowedAmount - 1, deadline, v, r, s);
    }
    */
}
