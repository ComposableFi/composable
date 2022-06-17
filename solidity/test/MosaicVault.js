const { ethers, upgrades, network } = require("hardhat");
const { keccak } = require("ethereumjs-util");

const { expect, deployProxy } = require("./test_env.js");
const { fork_network, fork_reset, mine_blocks } = require("./utils/network_fork");
const impersonateAccount = require("./utils/impersonate_account");

const initialSupply = ethers.utils.parseEther("100");
const allowance = ethers.utils.parseEther("10");
const remoteNetworkID = 1;
const remoteTokenRatio = 1000;
const blocksForActiveLiquidity = 90;
const amountToSend = new ethers.BigNumber.from(1).mul(10 ** 6);
const randomAddress = ethers.utils.getAddress("0xffffffffffffffffffffffffffffffffffffffff");
const VAULT_ROLE = ethers.utils.id("MOSAIC_VAULT");
const EMPTY_BYTES = ethers.utils.toUtf8Bytes("");

const USDC_ADDRESS = process.env.USDC_ADDRESS;
const USDT_ADDRESS = process.env.USDT_ADDRESS;
const BINANCE_WALLET_ADDRESS = process.env.BINANCE_WALLET_ADDRESS;

let NETWORK_ID;

let owner, tokensHolder, feeAccount;
let vaultConfig,
  ERC20Contract,
  ERC20Contract2,
  MosaicVaultContract,
  mosaicHolding,
  USDTContract,
  ERC20ContractIOUAddress,
  ERC20Contract2IOUAddress;

describe("MosaicVault - Test network", async () => {
  let mockWeth;
  beforeEach(async () => {
    [owner, tokensHolder, feeAccount] = await ethers.getSigners();

    // set 10 ether as the owner balance
    await network.provider.send("hardhat_setBalance", [
      tokensHolder.address,
      ethers.utils.parseEther("10").toHexString(),
    ]);

    await mockContracts();
    mockWeth = await (await ethers.getContractFactory("SampleWETH")).connect(owner).deploy();
    await vaultConfig.connect(owner).setWethAddress(mockWeth.address, 0, initialSupply);
    await vaultConfig
      .connect(owner)
      .addTokenInNetwork(mockWeth.address, randomAddress, remoteNetworkID, remoteTokenRatio);
  });

  describe("Test MosaicVault upgradable functionality", async () => {
    it("should upgrade", async () => {
      const MosaicVault = await ethers.getContractFactory("MosaicVault");

      const initial = await upgrades.deployProxy(MosaicVault, [vaultConfig.address]);
      const upgraded = await upgrades.upgradeProxy(initial.address, MosaicVault);
      await expect(initial.address).to.be.eq(upgraded.address);
    });
  });

  describe("Test MosaicVault Token functionalities", async () => {
    it("Not whitelisted token", async () => {
      await expect(
        MosaicVaultContract.connect(tokensHolder).transferERC20ToLayer(
          amountToSend,
          randomAddress,
          randomAddress,
          remoteNetworkID,
          0,
          randomAddress,
          0,
          0,
          false
        )
      ).to.be.revertedWith("ERR: TOKEN NOT WHITELISTED DESTINATION");
    });

    it("Adding token contract by non-owner", async () => {
      await expect(
        vaultConfig
          .connect(tokensHolder)
          .addWhitelistedToken(ERC20Contract.address, 0, initialSupply)
      ).to.be.revertedWith("Ownable: caller is not the owner");
    });

    it("Test receipt token decimals", async () => {
      const factory = await ethers.getContractFactory("SampleUSDC");
      const sampleUSDC = await factory.connect(owner).deploy(initialSupply);

      await vaultConfig.connect(owner).addWhitelistedToken(sampleUSDC.address, 0, initialSupply);

      const iouTokenContract = await getIOUTokenContract(sampleUSDC.address);
      const decimalsIOU = await iouTokenContract.decimals();
      const decimalsUSDC = await sampleUSDC.decimals();

      expect(decimalsIOU).to.eq(decimalsUSDC);
    });

    it("Remove whitelisted token", async () => {
      await expect(
        vaultConfig.connect(tokensHolder).removeWhitelistedToken(ERC20Contract.address)
      ).to.be.revertedWith("Ownable: caller is not the owner");

      await expect(vaultConfig.connect(owner).removeWhitelistedToken(ERC20Contract.address))
        .to.emit(vaultConfig, "TokenWhitelistRemoved")
        .withArgs(ERC20Contract.address);
    });

    it("Deposit zero amount", async () => {
      await expect(
        MosaicVaultContract.connect(tokensHolder).transferERC20ToLayer(
          0,
          ERC20Contract.address,
          ethers.constants.AddressZero,
          remoteNetworkID,
          0,
          ERC20Contract.address,
          0,
          0,
          false
        )
      ).to.be.revertedWith("ERR: AMOUNT");
    });

    it("Deposit", async () => {
      const depositAmount = 10000;
      const initialBalance = await ERC20Contract.balanceOf(mosaicHolding.address);

      await ERC20Contract.connect(tokensHolder).approve(MosaicVaultContract.address, depositAmount);

      await vaultConfig
        .connect(owner)
        .addTokenInNetwork(ERC20Contract.address, randomAddress, remoteNetworkID, remoteTokenRatio);

      await expect(
        MosaicVaultContract.connect(tokensHolder).transferERC20ToLayer(
          depositAmount,
          ERC20Contract.address,
          ethers.constants.AddressZero,
          remoteNetworkID,
          0,
          ERC20Contract.address,
          0,
          0,
          false
        )
      ).to.emit(MosaicVaultContract, "TransferInitiated");

      const finalBalance = await ERC20Contract.balanceOf(mosaicHolding.address);

      expect(finalBalance).to.eql(initialBalance.add(depositAmount));
    });

    it("Withdraw more than allowed should fail", async () => {
      await expect(
        MosaicVaultContract.connect(owner).withdrawTo(
          tokensHolder.address, //_accountTo
          allowance.add(ethers.BigNumber.from(1)), //amount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee: 0,
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: ethers.utils.formatBytes32String("HelloWorld"),
            amountToSwapToNative: 0,
            minAmountOutNative: 0,
            nativeSwapperId: 0,
          },
          EMPTY_BYTES
        )
      ).to.be.revertedWith("ERR: VAULT BAL");
    });

    it("Withdraw more than current available liquidity should fail", async () => {
      const currentLiquidityAvailable = await mosaicHolding.getTokenLiquidity(
        ERC20Contract.address,
        []
      );
      const amountToWithdraw = currentLiquidityAvailable.add(ethers.BigNumber.from(1));

      await vaultConfig
        .connect(owner)
        .addTokenInNetwork(ERC20Contract.address, randomAddress, remoteNetworkID, remoteTokenRatio);

      await expect(
        MosaicVaultContract.connect(owner).withdrawTo(
          tokensHolder.address, //_accountTo
          amountToWithdraw, //amount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee: allowance.add(ethers.BigNumber.from(1)),
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: ethers.utils.formatBytes32String("HelloWorld"),
            amountToSwapToNative: 0,
            minAmountOutNative: 0,
            nativeSwapperId: 0,
          },
          EMPTY_BYTES
        )
      ).to.be.revertedWith("ERR: VAULT BAL");
    });

    it("Only withdrawer can withdraw", async () => {
      await expect(
        MosaicVaultContract.connect(tokensHolder).withdrawTo(
          tokensHolder.address, //_accountTo
          10, //amount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee: allowance.add(ethers.BigNumber.from(1)),
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: ethers.utils.formatBytes32String("HelloWorld"),
            amountToSwapToNative: 0,
            minAmountOutNative: 0,
            nativeSwapperId: 0,
          },
          EMPTY_BYTES
        )
      ).to.be.revertedWith("ERR: PERMISSIONS");
    });

    it("Withdraw correctly", async () => {
      const withdrawAmount = ethers.BigNumber.from(100);
      const baseFee = ethers.BigNumber.from(1);
      const initialBalance = await mosaicHolding.getTokenLiquidity(ERC20Contract.address, []);
      const initialOwnerBalance = await ERC20Contract.balanceOf(owner.address);
      const id = ethers.utils.formatBytes32String("HelloWorld3");

      await expect(
        MosaicVaultContract.connect(owner).withdrawTo(
          tokensHolder.address, //_accountTo
          withdrawAmount, //amount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee,
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: id,
            amountToSwapToNative: 0,
            minAmountOutNative: 0,
            nativeSwapperId: 0,
          },
          EMPTY_BYTES
        )
      ).to.emit(MosaicVaultContract, "WithdrawalCompleted");
      const finalOwnerBalance = await ERC20Contract.balanceOf(owner.address);
      const lastID = await MosaicVaultContract.lastWithdrawID();
      expect(finalOwnerBalance.sub(initialOwnerBalance).eq(baseFee)).to.true;
      expect(lastID).to.eql(id);
    });

    it("Withdraw with the same ID should fail", async () => {
      const id = ethers.utils.formatBytes32String("HelloWorld3");
      await MosaicVaultContract.connect(owner).withdrawTo(
        tokensHolder.address, //_accountTo
        10000, //amount
        ERC20Contract.address, //tokenIn
        ERC20Contract.address, //tokenOut
        0, //amountOutMin
        {
          feePercentage: 0,
          baseFee: 0,
          investmentStrategy: ethers.constants.AddressZero,
          investmentStrategies: [],
          investmentStrategiesData: [],
          ammId: 0,
          id: id,
          amountToSwapToNative: 0,
          minAmountOutNative: 0,
          nativeSwapperId: 0,
        },
        EMPTY_BYTES
      );
      await expect(
        MosaicVaultContract.connect(owner).withdrawTo(
          tokensHolder.address, //_accountTo
          10000, //amount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee: 0,
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: id,
            amountToSwapToNative: 0,
            minAmountOutNative: 0,
            nativeSwapperId: 0,
          },
          EMPTY_BYTES
        )
      ).to.be.revertedWith("ERR: WITHDRAWN");
    });

    it("Set new withdrawer correctly", async () => {
      const depositAmount = 10000;

      await MosaicVaultContract.connect(owner).transferOwnership(tokensHolder.address);
      const id = ethers.utils.formatBytes32String("HelloWorld4");
      await expect(
        MosaicVaultContract.connect(tokensHolder).withdrawTo(
          tokensHolder.address, //_accountTo
          depositAmount, //amount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee: 0,
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: id,
            amountToSwapToNative: 0,
            minAmountOutNative: 0,
            nativeSwapperId: 0,
          },
          EMPTY_BYTES
        )
      ).to.emit(MosaicVaultContract, "WithdrawalCompleted");
    });

    it("Withdraw swapping to native but ableToPerformSmallBalanceSwap is false should fail", async () => {
      const depositAmount = 10000;
      await expect(
        MosaicVaultContract.connect(owner).withdrawTo(
          tokensHolder.address, //_accountTo
          depositAmount, //amount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee: 0,
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: ethers.utils.formatBytes32String("HelloWorld"),
            amountToSwapToNative: 100,
            minAmountOutNative: 10,
            nativeSwapperId: 1,
          },
          EMPTY_BYTES
        )
      ).to.be.revertedWith("ERR: UNABLE");
    });

    it("Withdraw swapping to native but no NativeSwapper has been set should fail", async () => {
      const depositAmount = 10000;
      await vaultConfig.connect(owner).setAbleToPerformSmallBalanceSwap(true);
      await expect(
        MosaicVaultContract.connect(owner).withdrawTo(
          tokensHolder.address, //_accountTo
          depositAmount, //amount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee: 0,
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: ethers.utils.formatBytes32String("HelloWorld"),
            amountToSwapToNative: 100,
            minAmountOutNative: 10,
            nativeSwapperId: 1,
          },
          EMPTY_BYTES
        )
      ).to.be.revertedWith("ERR: NOT SET");
    });

    it("Withdraw with too much amountToSwapToNative should fail", async () => {
      const depositAmount = 10000;
      await vaultConfig.connect(owner).setAbleToPerformSmallBalanceSwap(true);
      await expect(
        MosaicVaultContract.connect(owner).withdrawTo(
          tokensHolder.address, //_accountTo
          depositAmount, //amount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee: 0,
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: ethers.utils.formatBytes32String("HelloWorld"),
            amountToSwapToNative: depositAmount * 2, // Too much!
            minAmountOutNative: 10,
            nativeSwapperId: 1,
          },
          EMPTY_BYTES
        )
      ).to.be.revertedWith("ERR: TOO HIGH");
    });
  });

  describe("MosaicVault testing for unlock and refund transfer funds", async () => {
    it("Transfer funds correctly", async () => {
      const balanceBefore = await getERC20Balance(ERC20Contract.address, tokensHolder.address);
      await ERC20Contract.connect(tokensHolder).approve(MosaicVaultContract.address, amountToSend);
      const x = await expect(
        MosaicVaultContract.connect(tokensHolder).transferERC20ToLayer(
          amountToSend,
          ERC20Contract.address,
          ERC20Contract.address,
          remoteNetworkID,
          100,
          ERC20Contract.address,
          0,
          0,
          false
        )
      ).to.emit(MosaicVaultContract, "TransferInitiated");
      const balanceAfter = await getERC20Balance(ERC20Contract.address, tokensHolder.address);
      expect(balanceBefore).to.eq(balanceAfter.add(amountToSend));
    });

    it("Transfer funds to a different token correctly", async () => {
      const balanceBefore = await getERC20Balance(ERC20Contract.address, tokensHolder.address);
      await ERC20Contract.connect(tokensHolder).approve(MosaicVaultContract.address, amountToSend);
      await expect(
        MosaicVaultContract.connect(tokensHolder).transferERC20ToLayer(
          amountToSend,
          ERC20Contract.address,
          ERC20Contract.address,
          remoteNetworkID,
          100,
          USDC_ADDRESS,
          2,
          0,
          false
        )
      ).to.emit(MosaicVaultContract, "TransferInitiated");
      const balanceAfter = await getERC20Balance(ERC20Contract.address, tokensHolder.address);
      expect(balanceBefore).to.eq(balanceAfter.add(amountToSend));
    });
  });

  describe("Testing ETH transfer", async () => {
    it("Successfully transfer ETH to network", async () => {
      const ethAmountToSend = ethers.utils.parseEther("1");
      const balanceBefore = await ethers.provider.getBalance(tokensHolder.address);
      expect(balanceBefore).to.not.eq(0);
      const tx = await MosaicVaultContract.connect(tokensHolder).transferETHToLayer(
        tokensHolder.address,
        remoteNetworkID,
        100,
        ERC20Contract.address,
        0,
        0,
        false,
        { value: ethAmountToSend }
      );
      await expect(tx).to.emit(MosaicVaultContract, "TransferInitiated");
      const balanceAfter = await ethers.provider.getBalance(tokensHolder.address);
      const gasUsed = balanceBefore.sub(tx.value.add(balanceAfter));
      expect(balanceBefore).to.eq(balanceAfter.add(gasUsed).add(ethAmountToSend));
    });

    it("Failed transfer ETH to network", async () => {
      await expect(
        MosaicVaultContract.connect(tokensHolder).transferETHToLayer(
          tokensHolder.address,
          remoteNetworkID,
          100,
          ERC20Contract.address,
          0,
          0,
          false
        )
      ).to.revertedWith("ERR: AMOUNT");
    });

    it("successfully transfer ETH to network for different token", async () => {
      const ethAmountToSend = ethers.utils.parseEther("1");
      const balanceBefore = await ethers.provider.getBalance(tokensHolder.address);
      const tx = await MosaicVaultContract.connect(tokensHolder).transferETHToLayer(
        tokensHolder.address,
        remoteNetworkID,
        100,
        USDC_ADDRESS,
        2,
        0,
        false,
        { value: ethAmountToSend }
      );
      await expect(tx).to.emit(MosaicVaultContract, "TransferInitiated");
      const balanceAfter = await ethers.provider.getBalance(tokensHolder.address);
      const gasUsed = balanceBefore.sub(tx.value.add(balanceAfter));
      expect(balanceBefore).to.eq(balanceAfter.add(gasUsed).add(ethAmountToSend));
    });
  });

  describe("Test IOU token", async () => {
    it("Should create IOU token when add whitelisted token", async () => {
      expect(ERC20ContractIOUAddress).to.be.properAddress;

      const iouTokenContract = await ethers.getContractAt("IOUToken", ERC20ContractIOUAddress);
      const iouTokenOwner = await iouTokenContract.connect(tokensHolder).owner();
      expect(iouTokenOwner).to.eq(MosaicVaultContract.address);

      const underlyingTokenAddress = await iouTokenContract.connect(tokensHolder).underlyingToken();
      expect(underlyingTokenAddress).to.eq(ERC20Contract.address);
    });

    it("Should mint equal amount of IOU token when provide liquidity", async () => {
      await ERC20Contract.connect(tokensHolder).approve(MosaicVaultContract.address, allowance);
      const iouTokenContract = await getIOUTokenContract(ERC20Contract.address);
      const beforeDepositTokenBalance = await iouTokenContract.balanceOf(tokensHolder.address);
      await MosaicVaultContract.connect(tokensHolder).provideActiveLiquidity(
        allowance,
        ERC20Contract.address,
        blocksForActiveLiquidity
      );
      const afterDepositTokenBalance = await iouTokenContract.balanceOf(tokensHolder.address);
      expect(afterDepositTokenBalance.sub(beforeDepositTokenBalance)).to.eq(allowance);
    });

    it("Should burn IOU token on withdraw", async () => {
      const withdrawAmount = allowance.sub(ethers.BigNumber.from(1));
      const iouTokenContract = await ethers.getContractAt("IOUToken", ERC20ContractIOUAddress);
      const holderBalanceBeforeIOU = await iouTokenContract.balanceOf(tokensHolder.address);
      const holderBalanceBeforeERC = await ERC20Contract.balanceOf(tokensHolder.address);

      await expect(
        MosaicVaultContract.connect(tokensHolder).withdrawLiquidityRequest(
          ERC20ContractIOUAddress,
          withdrawAmount,
          ERC20Contract.address,
          tokensHolder.address,
          0,
          EMPTY_BYTES,
          NETWORK_ID,
          [0, 0, false]
        )
      ).to.emit(MosaicVaultContract, "WithdrawRequest");

      const baseFee = ethers.BigNumber.from(10).pow(9);
      const randomAddress1 = ethers.utils.getAddress("0xfffffffffffffffffffffffffffffffffffff123");
      await MosaicVaultContract.connect(owner).setRelayer(randomAddress1);

      await expect(
        MosaicVaultContract.connect(owner).withdrawLiquidity(
          tokensHolder.address, //_receiver
          withdrawAmount, //_amountIn
          withdrawAmount, //requestedAmount
          ERC20Contract.address, //tokenIn
          ERC20Contract.address, //tokenOut
          0, //amountOutMin
          {
            feePercentage: 0,
            baseFee: baseFee,
            investmentStrategy: ethers.constants.AddressZero,
            investmentStrategies: [],
            investmentStrategiesData: [],
            ammId: 0,
            id: ethers.utils.id("id"),
            amountToSwapToNative: 0,
            minAmountOutNative: 0,
            nativeSwapperId: 0,
          },
          EMPTY_BYTES //data
        )
      ).to.emit(MosaicVaultContract, "LiquidityWithdrawn");

      const holderBalanceAfterIOU = await iouTokenContract.balanceOf(tokensHolder.address);
      expect(holderBalanceBeforeIOU).to.eq(holderBalanceAfterIOU.add(withdrawAmount));

      const holderBalanceAfterERC = await ERC20Contract.balanceOf(tokensHolder.address);
      expect(holderBalanceBeforeERC).to.eq(holderBalanceAfterERC.sub(withdrawAmount).add(baseFee));
    });

    it("should fail to mint IOU token by other user than owner", async () => {
      const iouTokenContract = await getIOUTokenContract(ERC20Contract.address);
      await expect(
        iouTokenContract.connect(tokensHolder).mint(tokensHolder.address, 1)
      ).to.revertedWith("Ownable: caller is not the owner");
    });

    it("should fail to burn IOU token by other user than owner", async () => {
      await ERC20Contract.connect(tokensHolder).approve(MosaicVaultContract.address, allowance);
      await MosaicVaultContract.connect(tokensHolder).provideActiveLiquidity(
        allowance,
        ERC20Contract.address,
        blocksForActiveLiquidity
      );

      const iouTokenContract = await getIOUTokenContract(ERC20Contract.address);
      // hardhat can't call overload function https://github.com/ethers-io/ethers.js/issues/407
      await expect(
        iouTokenContract.connect(tokensHolder)["burn(address,uint256)"](tokensHolder.address, 1)
      ).to.revertedWith("Ownable: caller is not the owner");
    });
  });

  describe("Mosaic Vault Withdraw Liquidity to different token on same network ", async () => {
    beforeEach(async () => {
      const UniswapWrapper = await deployProxy("UniswapWrapper", [
        process.env.UNISWAP_ADDRESS,
        process.env.UNISWAP_QUOTER_ADDRESS,
      ]);
      await UniswapWrapper.deployed();
      await vaultConfig.addSupportedAMM(2, UniswapWrapper.address);
    });

    it("Should fail if selected token on same network is not whitelisted", async () => {
      await vaultConfig.removeWhitelistedToken(ERC20Contract.address);
      await expect(
        MosaicVaultContract.connect(tokensHolder).withdrawLiquidityRequest(
          ERC20ContractIOUAddress,
          allowance,
          ERC20Contract.address,
          tokensHolder.address,
          2,
          EMPTY_BYTES,
          NETWORK_ID,
          [0, 0, false]
        )
      ).to.be.revertedWith("ERR: TOKEN NOT WHITELISTED");
    });

    it("Should fail if IOU balance is low", async () => {
      await ERC20Contract2.connect(owner).transfer(mosaicHolding.address, allowance);

      await vaultConfig
        .connect(owner)
        .addWhitelistedToken(ERC20Contract2.address, 0, initialSupply);
      await vaultConfig
        .connect(owner)
        .addTokenInNetwork(
          ERC20Contract2.address,
          randomAddress,
          remoteNetworkID,
          remoteTokenRatio
        );

      const iouTokenContract = await getIOUTokenContract(ERC20Contract2.address);
      await expect(
        MosaicVaultContract.connect(tokensHolder).withdrawLiquidityRequest(
          iouTokenContract.address,
          allowance,
          ERC20Contract.address,
          tokensHolder.address,
          2,
          EMPTY_BYTES,
          NETWORK_ID,
          [allowance, 0, false]
        )
      ).to.be.revertedWith("ERR: BALANCE");
    });
  });

  describe("MosaicVault Withdraw Liquidity to different network ", async () => {
    it("Should fail if token is not whitelisted in current network", async () => {
      const tempToken = await deployERC20("Temp token", "TMP", initialSupply);
      const remoteTokenAddress = await vaultConfig.remoteTokenAddress(
        remoteNetworkID,
        tempToken.address
      );
      await vaultConfig.connect(owner).addWhitelistedToken(tempToken.address, 0, initialSupply);
      const whitelistedToken = await vaultConfig.whitelistedTokens(tempToken.address);
      const tempTokenIOUAddress = whitelistedToken.underlyingIOUAddress;
      await expect(remoteTokenAddress).to.eq(ethers.constants.AddressZero);
      await expect(
        MosaicVaultContract.connect(tokensHolder).withdrawLiquidityRequest(
          tempTokenIOUAddress,
          allowance,
          ERC20Contract.address,
          tokensHolder.address,
          0,
          EMPTY_BYTES,
          remoteNetworkID,
          [allowance, 0, false]
        )
      ).to.be.revertedWith("ERR: TOKEN");
    });

    it("should withdraw liquidity to a different network", async () => {
      await expect(
        MosaicVaultContract.connect(tokensHolder).withdrawLiquidityRequest(
          ERC20ContractIOUAddress,
          allowance,
          ERC20Contract.address,
          tokensHolder.address,
          0,
          EMPTY_BYTES,
          NETWORK_ID,
          [allowance, 0, false]
        )
      ).to.emit(MosaicVaultContract, "WithdrawRequest");
    });

    it("should fail to withdraw before the number of block user specify", async () => {
      const tempToken = await deployERC20("Temp token", "TMP", initialSupply);
      await tempToken.connect(owner).transfer(tokensHolder.address, initialSupply);
      await tempToken.connect(tokensHolder).approve(MosaicVaultContract.address, allowance);
      await vaultConfig.connect(owner).addWhitelistedToken(tempToken.address, 0, initialSupply);
      await vaultConfig
        .connect(owner)
        .addTokenInNetwork(tempToken.address, randomAddress, remoteNetworkID, remoteTokenRatio);
      await MosaicVaultContract.connect(tokensHolder).provideActiveLiquidity(
        allowance,
        tempToken.address,
        blocksForActiveLiquidity
      );
      const iouTempTokenContract = await getIOUTokenContract(tempToken.address);

      await expect(
        MosaicVaultContract.connect(tokensHolder).withdrawLiquidityRequest(
          iouTempTokenContract.address,
          allowance,
          tempToken.address,
          tokensHolder.address,
          0,
          EMPTY_BYTES,
          NETWORK_ID,
          [0, 0, false]
        )
      ).to.be.revertedWith("ERR: LIQUIDITY");
    });
  });

  describe("MosaicVault Withdraw Liquidity to different network and different token ", async () => {
    beforeEach(async () => {
      const UniswapWrapper = await deployProxy("UniswapWrapper", [
        process.env.UNISWAP_ADDRESS,
        process.env.UNISWAP_QUOTER_ADDRESS,
      ]);
      await UniswapWrapper.deployed();
      await vaultConfig.addSupportedAMM(2, UniswapWrapper.address);
    });

    it("should fail if networkID is unsupported in current network", async () => {
      await vaultConfig
        .connect(owner)
        .addWhitelistedToken(ERC20Contract2.address, 0, initialSupply);
      await vaultConfig
        .connect(owner)
        .addTokenInNetwork(
          ERC20Contract2.address,
          randomAddress,
          remoteNetworkID,
          remoteTokenRatio
        );

      await expect(
        MosaicVaultContract.connect(tokensHolder).withdrawLiquidityRequest(
          ERC20ContractIOUAddress,
          allowance,
          ERC20Contract2.address,
          tokensHolder.address,
          2,
          EMPTY_BYTES,
          2,
          [allowance, 0, false]
        )
      ).to.be.revertedWith("ERR: TOKEN");
    });
  });

  describe("testing unpause and pause network", async () => {
    it("successfully pause network", async () => {
      const networkId = 1;
      await expect(vaultConfig.connect(owner).pauseNetwork(networkId))
        .to.emit(vaultConfig, "PauseNetwork")
        .withArgs(owner.address, networkId);
      let state = await vaultConfig.pausedNetwork(networkId);
      expect(state).to.true;
    });

    it("successfully unpause network", async () => {
      const networkId = 1;
      await expect(vaultConfig.connect(owner).unpauseNetwork(networkId))
        .to.emit(vaultConfig, "UnpauseNetwork")
        .withArgs(owner.address, networkId);
      let state = await vaultConfig.pausedNetwork(networkId);
      expect(state).to.false;
    });
  });

  describe("testing fee settings", async () => {
    it("Only owner can set new fee", async () => {
      await expect(vaultConfig.connect(tokensHolder).setMinFee(1)).to.be.revertedWith(
        "Ownable: caller is not the owner"
      );
      await expect(vaultConfig.connect(tokensHolder).setMaxFee(600)).to.be.revertedWith(
        "Ownable: caller is not the owner"
      );
    });

    it("successfully setMinFee", async () => {
      const fee = 50;
      await expect(vaultConfig.connect(owner).setMinFee(fee))
        .to.emit(vaultConfig, "MinFeeChanged")
        .withArgs(fee);
    });

    it("fail to setMinFee", async () => {
      const fee = 600;
      await expect(vaultConfig.connect(owner).setMinFee(fee)).to.revertedWith("ERR: MIN > MAX");
    });

    it("successfully setMaxFee", async () => {
      const fee = 50;
      await expect(vaultConfig.connect(owner).setMaxFee(fee))
        .to.emit(vaultConfig, "MaxFeeChanged")
        .withArgs(fee);
    });

    it("fail to setMaxFee", async () => {
      const fee = 20;
      await expect(vaultConfig.connect(owner).setMinFee(50))
        .to.emit(vaultConfig, "MinFeeChanged")
        .withArgs(50);
      await expect(vaultConfig.connect(owner).setMaxFee(fee)).to.revertedWith("ERR: MIN > MAX");
    });
  });

  describe("Fallback", async () => {
    it("Should send eth", async () => {
      const WethContract = await ethers.getContractAt("IWETH", mockWeth.address);
      const balanceBefore = await WethContract.balanceOf(mosaicHolding.address);

      await tokensHolder.sendTransaction({
        to: MosaicVaultContract.address,
        value: ethers.utils.parseEther("1"),
      });

      const balanceAfter = await WethContract.balanceOf(mosaicHolding.address);
      await expect(balanceAfter.sub(balanceBefore)).to.be.equals(ethers.utils.parseEther("1"));
    });
  });

  async function getIOUTokenContract(whitelistedToken) {
    const iouTokenContract = await vaultConfig.whitelistedTokens(whitelistedToken);
    return ethers.getContractAt("IOUToken", iouTokenContract.underlyingIOUAddress);
  }
});

describe("MosaicVault - Tests mainnet fork", async () => {
  let binanceWallet;
  beforeEach(async () => {
    await fork_network();
    await impersonateAccount(BINANCE_WALLET_ADDRESS);
    binanceWallet = await ethers.getSigner(BINANCE_WALLET_ADDRESS);
    [owner, tokensHolder, feeAccount] = await ethers.getSigners();

    await mockContracts();
    await vaultConfig.connect(owner).addWhitelistedToken(USDC_ADDRESS, 0, initialSupply);
    await vaultConfig
      .connect(owner)
      .addTokenInNetwork(USDC_ADDRESS, randomAddress, remoteNetworkID, remoteTokenRatio);
    await vaultConfig.connect(owner).addWhitelistedToken(USDT_ADDRESS, 0, initialSupply);
    await vaultConfig
      .connect(owner)
      .addTokenInNetwork(USDT_ADDRESS, randomAddress, remoteNetworkID, remoteTokenRatio);

    // WETH
    await vaultConfig.connect(owner).setWethAddress(process.env.WETH_ADDRESS, 0, initialSupply);
    await vaultConfig
      .connect(owner)
      .addTokenInNetwork(
        process.env.WETH_ADDRESS,
        randomAddress,
        remoteNetworkID,
        remoteTokenRatio
      );

    // provide liquidity
    const USDc = await ethers.getContractAt("IERC20", USDC_ADDRESS);
    await USDc.connect(binanceWallet).approve(MosaicVaultContract.address, allowance);
    await MosaicVaultContract.connect(binanceWallet).provideActiveLiquidity(
      amountToSend,
      USDC_ADDRESS,
      blocksForActiveLiquidity
    );

    USDTContract = await ethers.getContractAt("IERC20", USDT_ADDRESS);
  });

  afterEach(async () => {
    await fork_reset();
  });

  it("should fail because exchange address not set", async () => {
    await expect(
      MosaicVaultContract.connect(owner).withdrawLiquidity(
        tokensHolder.address, //_receiver
        amountToSend, //amount
        amountToSend, //requestedAmount
        USDC_ADDRESS, //tokenIn
        USDT_ADDRESS, //tokenOut
        amountToSend.sub(900000), //amountOutMin
        {
          feePercentage: 0,
          baseFee: 0,
          investmentStrategy: ethers.constants.AddressZero,
          investmentStrategies: [],
          investmentStrategiesData: [],
          ammId: 0,
          id: ethers.utils.id("random"),
          amountToSwapToNative: 0,
          minAmountOutNative: 0,
          nativeSwapperId: 0,
        },
        EMPTY_BYTES
      )
    ).to.revertedWith("ERR: AMM");
  });

  it("Withdraw swapping to native but ableToPerformSmallBalanceSwap is false should fail", async () => {
    await expect(
      MosaicVaultContract.connect(owner).withdrawLiquidity(
        tokensHolder.address, //_receiver
        amountToSend, //amount
        amountToSend, //requestedAmount
        USDC_ADDRESS, //tokenIn
        USDT_ADDRESS, //tokenOut
        amountToSend.sub(900000), //amountOutMin
        {
          feePercentage: 0,
          baseFee: 0,
          investmentStrategy: ethers.constants.AddressZero,
          investmentStrategies: [],
          investmentStrategiesData: [],
          ammId: 0,
          id: ethers.utils.id("random"),
          amountToSwapToNative: 1000,
          minAmountOutNative: 10,
          nativeSwapperId: 1,
        },
        EMPTY_BYTES
      )
    ).to.be.revertedWith("ERR: UNABLE");
  });

  it("Withdraw swapping to native but no NativeSwapper has been set should fail", async () => {
    await vaultConfig.connect(owner).setAbleToPerformSmallBalanceSwap(true);
    await expect(
      MosaicVaultContract.connect(owner).withdrawLiquidity(
        tokensHolder.address, //_receiver
        amountToSend, //amount
        amountToSend, //requestedAmount
        USDC_ADDRESS, //tokenIn
        USDT_ADDRESS, //tokenOut
        amountToSend.sub(900000), //amountOutMin
        {
          feePercentage: 0,
          baseFee: 0,
          investmentStrategy: ethers.constants.AddressZero,
          investmentStrategies: [],
          investmentStrategiesData: [],
          ammId: 0,
          id: ethers.utils.id("random"),
          amountToSwapToNative: 10000,
          minAmountOutNative: 10,
          nativeSwapperId: 1,
        },
        EMPTY_BYTES
      )
    ).to.be.revertedWith("ERR: NOT SET");
  });

  it("Withdraw with too much amountToSwapToNative should fail", async () => {
    await vaultConfig.connect(owner).setAbleToPerformSmallBalanceSwap(true);
    await expect(
      MosaicVaultContract.connect(owner).withdrawLiquidity(
        tokensHolder.address, //_receiver
        amountToSend, //amount
        amountToSend, //requestedAmount
        USDC_ADDRESS, //tokenIn
        USDT_ADDRESS, //tokenOut
        amountToSend.sub(900000), //amountOutMin
        {
          feePercentage: 0,
          baseFee: 0,
          investmentStrategy: ethers.constants.AddressZero,
          investmentStrategies: [],
          investmentStrategiesData: [],
          ammId: 0,
          id: ethers.utils.id("random"),
          amountToSwapToNative: amountToSend.mul(2),
          minAmountOutNative: 10,
          nativeSwapperId: 1,
        },
        EMPTY_BYTES
      )
    ).to.be.revertedWith("ERR: TOO HIGH");
  });

  it("Withdraw swapping to native correctly", async () => {
    const UniswapWrapper = await deployProxy("UniswapWrapper", [
      process.env.UNISWAP_ADDRESS,
      process.env.UNISWAP_QUOTER_ADDRESS,
    ]);
    await UniswapWrapper.deployed();
    await vaultConfig.addSupportedAMM(2, UniswapWrapper.address);

    const encodedDataSwap = getEncodedSwapData();
    const MosaicNativeSwapperContract = await (
      await ethers.getContractFactory("MosaicNativeSwapperETH")
    )
      .connect(owner)
      .deploy(process.env.UNISWAP_ROUTER_02_ADDRESS);
    await MosaicNativeSwapperContract.deployed();

    await vaultConfig
      .connect(owner)
      .addSupportedMosaicNativeSwapper(1, MosaicNativeSwapperContract.address);
    await expect(
      MosaicVaultContract.connect(owner).withdrawLiquidity(
        tokensHolder.address, //_receiver
        amountToSend, //amount
        amountToSend, //requestedAmount
        USDC_ADDRESS, //tokenIn
        USDT_ADDRESS, //tokenOut
        amountToSend.sub(900000), //amountOutMin
        {
          feePercentage: 0,
          baseFee: 0,
          investmentStrategy: ethers.constants.AddressZero,
          investmentStrategies: [],
          investmentStrategiesData: [],
          ammId: 2,
          id: ethers.utils.id("random"),
          amountToSwapToNative: 1000,
          minAmountOutNative: 10,
          nativeSwapperId: 1,
        },
        encodedDataSwap
      )
    ).to.emit(MosaicNativeSwapperContract, "SwappedToNative");
  });

  it("uniswap: should withdraw liquidity to a different network and different token", async () => {
    const UniswapWrapper = await deployProxy("UniswapWrapper", [
      process.env.UNISWAP_ADDRESS,
      process.env.UNISWAP_QUOTER_ADDRESS,
    ]);
    await UniswapWrapper.deployed();
    await vaultConfig.addSupportedAMM(2, UniswapWrapper.address);

    const balanceBefore = await USDTContract.balanceOf(tokensHolder.address);
    const encodedDataSwap = getEncodedSwapData();

    const transaction = await MosaicVaultContract.connect(owner).withdrawLiquidity(
      tokensHolder.address, //_receiver
      amountToSend, //amount
      amountToSend, //requestedAmount
      USDC_ADDRESS, //tokenIn
      USDT_ADDRESS, //tokenOut
      amountToSend.sub(900000), //amountOutMin
      {
        feePercentage: 0,
        baseFee: 0,
        investmentStrategy: ethers.constants.AddressZero,
        investmentStrategies: [],
        investmentStrategiesData: [],
        ammId: 2,
        id: ethers.utils.id("random"),
        amountToSwapToNative: 0,
        minAmountOutNative: 0,
        nativeSwapperId: 0,
      },
      encodedDataSwap
    );
    await transaction.wait();

    const balanceAfter = await USDTContract.balanceOf(tokensHolder.address);
    expect(balanceAfter).to.be.above(balanceBefore);
  });

  it("sushiswap: should withdraw liquidity to a different network and different token", async () => {
    const SushiswapWrapper = await deployProxy("SushiswapWrapper", [
      process.env.SUSHISWAP_ROUTER_ADDRESS,
    ]);
    await SushiswapWrapper.deployed();
    await vaultConfig.addSupportedAMM(3, SushiswapWrapper.address);

    const balanceBefore = await USDTContract.balanceOf(tokensHolder.address);
    const blockNum = await ethers.provider.getBlockNumber();
    const curBlock = await ethers.provider.getBlock(blockNum);
    const deadline = ethers.utils.defaultAbiCoder.encode(["uint"], [curBlock.timestamp + 15]);

    const transaction = await MosaicVaultContract.connect(owner).withdrawLiquidity(
      tokensHolder.address, //_receiver
      amountToSend, //amount
      amountToSend, //requestedAmount
      USDC_ADDRESS, //tokenIn
      USDT_ADDRESS, //tokenOut
      amountToSend.sub(900000), //amountOutMin
      {
        feePercentage: 0,
        baseFee: 0,
        investmentStrategy: ethers.constants.AddressZero,
        investmentStrategies: [],
        investmentStrategiesData: [],
        ammId: 3,
        id: ethers.utils.id("random"),
        amountToSwapToNative: 0,
        minAmountOutNative: 0,
        nativeSwapperId: 0,
      },
      deadline
    );

    await transaction.wait();

    const balanceAfter = await USDTContract.balanceOf(tokensHolder.address);
    expect(balanceAfter).to.be.above(balanceBefore);
  });

  it("uniswapV2: should withdraw liquidity to a different network and different token", async () => {
    const UniswapV2Wrapper = await deployProxy("UniswapV2Wrapper", [
      process.env.UNISWAP_ROUTER_02_ADDRESS,
    ]);
    const ammId = 4;
    await UniswapV2Wrapper.deployed();
    await vaultConfig.addSupportedAMM(ammId, UniswapV2Wrapper.address);

    const balanceBefore = await USDTContract.balanceOf(tokensHolder.address);
    const blockNum = await ethers.provider.getBlockNumber();
    const curBlock = await ethers.provider.getBlock(blockNum);
    const deadline = ethers.utils.defaultAbiCoder.encode(["uint"], [curBlock.timestamp + 15]);

    const transaction = await MosaicVaultContract.connect(owner).withdrawLiquidity(
      tokensHolder.address, //_receiver
      amountToSend, //amount
      amountToSend, //requestedAmount
      USDC_ADDRESS, //tokenIn
      USDT_ADDRESS, //tokenOut
      amountToSend.sub(900000), //amountOutMin
      {
        feePercentage: 0,
        baseFee: 0,
        investmentStrategy: ethers.constants.AddressZero,
        investmentStrategies: [],
        investmentStrategiesData: [],
        ammId,
        id: ethers.utils.id("random"),
        amountToSwapToNative: 0,
        minAmountOutNative: 0,
        nativeSwapperId: 0,
      },
      deadline
    );

    await transaction.wait();

    const balanceAfter = await USDTContract.balanceOf(tokensHolder.address);
    expect(balanceAfter).to.be.above(balanceBefore);
  });

  it("curve: should withdraw liquidity to a different network and different token", async () => {
    const CurveWrapper = await deployProxy("CurveWrapper", []);
    await CurveWrapper.deployed();
    await vaultConfig.addSupportedAMM(4, CurveWrapper.address);
    const balanceBefore = await USDTContract.balanceOf(tokensHolder.address);
    const dt = ethers.utils.defaultAbiCoder.encode(
      ["address", "uint256", "int128", "int128", "uint256", "uint256"],
      ["0xbebc44782c7db0a1a60cb6fe97d0b483032ff1c7", "0", "1", "2", "1", "2"]
    );

    const transaction = await MosaicVaultContract.connect(owner).withdrawLiquidity(
      tokensHolder.address, //_receiver
      amountToSend, //amount
      amountToSend, //requestedAmount
      USDC_ADDRESS, //tokenIn
      USDT_ADDRESS, //tokenOut
      0, //amountOutMin
      {
        feePercentage: 0,
        baseFee: 0,
        investmentStrategy: ethers.constants.AddressZero,
        investmentStrategies: [],
        investmentStrategiesData: [],
        ammId: 4,
        id: ethers.utils.id("random"),
        amountToSwapToNative: 0,
        minAmountOutNative: 0,
        nativeSwapperId: 0,
      },
      dt
    );

    await transaction.wait();
    const balanceAfter = await USDTContract.balanceOf(tokensHolder.address);
    expect(balanceAfter).to.be.above(balanceBefore);
  });

  it("should successfully withdraw to another token", async () => {
    const UniswapWrapper = await deployProxy("UniswapWrapper", [
      process.env.UNISWAP_ADDRESS,
      process.env.UNISWAP_QUOTER_ADDRESS,
    ]);
    await UniswapWrapper.deployed();

    await vaultConfig.addSupportedAMM(2, UniswapWrapper.address);
    const USDC = await ethers.getContractAt("IERC20", USDC_ADDRESS);

    await USDC.connect(binanceWallet).approve(MosaicVaultContract.address, amountToSend);
    await MosaicVaultContract.connect(binanceWallet).provideActiveLiquidity(
      amountToSend,
      USDC_ADDRESS,
      blocksForActiveLiquidity
    );

    USDTContract = await ethers.getContractAt("IERC20", USDT_ADDRESS);

    const balanceBefore = await USDTContract.balanceOf(binanceWallet.address);
    const encodedDataSwap = getEncodedSwapData();
    await mine_blocks(blocksForActiveLiquidity);

    const transaction = await MosaicVaultContract.connect(owner).withdrawLiquidity(
      binanceWallet.address, //_receiver
      amountToSend, //_amountIn
      amountToSend, //requestedAmount
      USDC_ADDRESS, //tokenIn
      USDT_ADDRESS, //tokenOut
      amountToSend.sub(900000), //amountOutMin
      {
        feePercentage: 0,
        baseFee: 0,
        investmentStrategy: ethers.constants.AddressZero,
        investmentStrategies: [],
        investmentStrategiesData: [],
        ammId: 2,
        id: ethers.utils.id("random"),
        amountToSwapToNative: 0,
        minAmountOutNative: 0,
        nativeSwapperId: 0,
      },
      encodedDataSwap
    );
    await transaction.wait();
    const balanceAfter = await USDTContract.balanceOf(binanceWallet.address);
    expect(balanceAfter).to.be.above(balanceBefore);
  });

  function getEncodedSwapData(
    deadline = Math.floor(Date.now() / 1000) + 30 * 60,
    sqrtPriceLimitX96 = 0,
    fee = 3000
  ) {
    return ethers.utils.defaultAbiCoder.encode(
      ["uint256", "uint160", "uint24"],
      [deadline, sqrtPriceLimitX96, fee]
    );
  }
});

async function mockContracts() {
  mosaicHolding = await deployProxy("MosaicHolding", [owner.address]);
  vaultConfig = await deployProxy("MosaicVaultConfig", [mosaicHolding.address]);

  MosaicVaultContract = await deployProxy("MosaicVault", [vaultConfig.address]);
  await mosaicHolding.setUniqRole(VAULT_ROLE, MosaicVaultContract.address);
  await vaultConfig.connect(owner).setVault(MosaicVaultContract.address);

  const iouTokenFactory = await (await ethers.getContractFactory("TokenFactory"))
    .connect(owner)
    .deploy(MosaicVaultContract.address, vaultConfig.address);

  await vaultConfig.connect(owner).setTokenFactoryAddress(iouTokenFactory.address);

  ERC20Contract = await deployERC20("Token", "TKN", initialSupply);
  await ERC20Contract.connect(owner).transfer(tokensHolder.address, initialSupply);

  ERC20Contract2 = await deployERC20("Token2", "TKN2", initialSupply);

  await vaultConfig.connect(owner).addWhitelistedToken(ERC20Contract.address, 0, initialSupply);
  await MosaicVaultContract.connect(owner).setRelayer(owner.address);
  const whitelistedToken = await vaultConfig.whitelistedTokens(ERC20Contract.address);
  const whitelistedToken2 = await vaultConfig.whitelistedTokens(ERC20Contract2.address);
  ERC20ContractIOUAddress = whitelistedToken.underlyingIOUAddress;
  ERC20Contract2IOUAddress = whitelistedToken2.underlyingIOUAddress;
  await vaultConfig
    .connect(owner)
    .addTokenInNetwork(ERC20Contract.address, randomAddress, remoteNetworkID, remoteTokenRatio);

  await ERC20Contract.connect(tokensHolder).approve(MosaicVaultContract.address, allowance);
  await MosaicVaultContract.connect(tokensHolder).provideActiveLiquidity(
    allowance,
    ERC20Contract.address,
    blocksForActiveLiquidity
  );
  await mine_blocks(blocksForActiveLiquidity);
  const { chainId } = await ethers.provider.getNetwork();
  NETWORK_ID = chainId;
}

async function deployERC20(name, symbol, initialSupply) {
  const factory = await ethers.getContractFactory("SampleTokenERC20");
  return factory.connect(owner).deploy(name, symbol, initialSupply);
}

async function getERC20Balance(tokenAddress, user) {
  const erc20Contract = await ethers.getContractAt("IERC20", tokenAddress);
  return erc20Contract.balanceOf(user);
}
