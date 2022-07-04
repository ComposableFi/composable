const { ethers, upgrades } = require("hardhat");

const { expect, deployProxy } = require("./test_env.js");
const {
  fork_network,
  fork_reset,
  mine_blocks,
  increase_block_timestamp,
} = require("./utils/network_fork");
const impersonateAccount = require("./utils/impersonate_account");

const rebalancingAmount = ethers.utils.parseEther("1");
const rebalancingThreshold = ethers.utils.parseEther("10");
const initialSupply = ethers.utils.parseEther("100");
const BINANCE_WALLET_ADDRESS = process.env.BINANCE_WALLET_ADDRESS;
const USDC_ADDRESS = process.env.USDC_ADDRESS;
const USDT_ADDRESS = process.env.USDT_ADDRESS;
const DAI_ADDRESS = process.env.DAI_ADDRESS;
const CUSDC_ADDRESS = process.env.CUSDC_ADDRESS;

let owner, vaultAccount, vaultAccount2, rebalancingBot, user1;
let mosaicHolding, ERC20Contract;
let compoundInvestmentStrategy, USDc, DAI, aaveStrategy, sushiStrategy;
let binanceWallet;

const ADMIN_ROLE = ethers.constants.HashZero;
const VAULT_ROLE = ethers.utils.id("MOSAIC_VAULT");
const REBALANCING_ROLE = ethers.utils.id("REBALANCING_BOT");

describe("Test network", async () => {
  before(async () => {
    await getSigners();
  });

  beforeEach(async () => {
    await deployHoldingsContract();
    await mosaicHolding.setUniqRole(VAULT_ROLE, vaultAccount.address);
    await mosaicHolding.setUniqRole(REBALANCING_ROLE, rebalancingBot.address);

    ERC20Contract = await (await ethers.getContractFactory("SampleTokenERC20"))
      .connect(owner)
      .deploy("Token", "TKN", initialSupply);

    await ERC20Contract.connect(owner).transfer(mosaicHolding.address, initialSupply);

    await mosaicHolding.setRebalancingThreshold(ERC20Contract.address, rebalancingThreshold);
  });

  describe("Test Mosaic Holding upgradable functionality", async () => {
    it("should upgrade", async () => {
      const MosaicHolding = await ethers.getContractFactory("MosaicHolding");
      const MosaicHoldingV2 = await ethers.getContractFactory("MosaicHolding");

      const initial = await upgrades.deployProxy(MosaicHolding, [owner.address]);
      const upgraded = await upgrades.upgradeProxy(initial.address, MosaicHoldingV2);
      await expect(initial.address).to.be.eq(upgraded.address);
    });
  });

  describe("Test Mosaic Holding permissions", async () => {
    it("should have owner set", async () => {
      const isAdmin = await mosaicHolding.hasRole(ADMIN_ROLE, owner.address);
      expect(isAdmin).be.true;
    });

    it("should set rebalancing bot role", async () => {
      await mosaicHolding.setUniqRole(REBALANCING_ROLE, rebalancingBot.address);
      const isBot = await mosaicHolding.hasRole(REBALANCING_ROLE, rebalancingBot.address);
      expect(isBot).be.true;
    });

    it("should set vault address and role", async () => {
      await mosaicHolding.setUniqRole(VAULT_ROLE, vaultAccount.address);
      const isVault = await mosaicHolding.hasRole(VAULT_ROLE, vaultAccount.address);
      expect(isVault).be.true;
    });

    it("should change vault address and remove old one", async () => {
      await mosaicHolding.setUniqRole(VAULT_ROLE, vaultAccount2.address);
      const isVault = await mosaicHolding.hasRole(VAULT_ROLE, vaultAccount2.address);
      expect(isVault).be.true;
    });

    it("should change vault address only by the admin", async () => {
      await expect(
        mosaicHolding.connect(vaultAccount2).setUniqRole(VAULT_ROLE, vaultAccount.address)
      ).to.revertedWith("ERR: PERMISSIONS A");
    });

    it("should allow owner to transfer tokens", async () => {
      const userBalanceBefore = await ERC20Contract.balanceOf(vaultAccount2.address);
      await mosaicHolding
        .connect(owner)
        .transfer(ERC20Contract.address, vaultAccount2.address, initialSupply);
      const userBalanceAfter = await ERC20Contract.balanceOf(vaultAccount2.address);
      expect(userBalanceAfter.sub(userBalanceBefore)).to.eq(initialSupply);
    });

    it("should allow vault to transfer tokens", async () => {
      const userBalanceBefore = await ERC20Contract.balanceOf(vaultAccount2.address);
      await mosaicHolding
        .connect(vaultAccount)
        .transfer(ERC20Contract.address, vaultAccount2.address, initialSupply);
      const userBalanceAfter = await ERC20Contract.balanceOf(vaultAccount2.address);
      expect(userBalanceAfter.sub(userBalanceBefore)).to.eq(initialSupply);
    });

    it("should allow the rebalancing bot to transfer tokens", async () => {
      const userBalanceBefore = await ERC20Contract.balanceOf(rebalancingBot.address);
      await mosaicHolding
        .connect(rebalancingBot)
        .extractLiquidityForRebalancing(
          ERC20Contract.address,
          rebalancingAmount,
          rebalancingBot.address
        );
      const userBalanceAfter = await ERC20Contract.balanceOf(rebalancingBot.address);
      expect(userBalanceAfter.sub(userBalanceBefore)).to.eq(rebalancingAmount);
    });

    it("should not allow other accounts to transfer tokens", async () => {
      await expect(
        mosaicHolding
          .connect(vaultAccount2)
          .transfer(ERC20Contract.address, vaultAccount2.address, initialSupply)
      ).to.revertedWith("ERR: PERMISSIONS A-V");
    });
  });

  describe("Save Funds", async () => {
    const time = 20;
    it("Should fail to caller is not owner", async () => {
      await expect(
        mosaicHolding.connect(user1).startSaveFundsLockUpTimerChange(time)
      ).to.revertedWith("ERR: PERMISSIONS A");

      await expect(
        mosaicHolding.connect(user1).startSaveFunds(ERC20Contract.address, user1.address)
      ).to.revertedWith("ERR: PERMISSIONS A");
    });

    it("should start Save Funds LockUp Timer Change", async () => {
      await mosaicHolding.connect(owner).startSaveFundsLockUpTimerChange(time);
      await expect(time).to.be.equals(await mosaicHolding.newSaveFundsLockUpTime());
    });

    it("Should fail to change setSaveFundsLockUpTime if duration time less than current time", async () => {
      await mosaicHolding.connect(owner).startSaveFundsLockUpTimerChange(time);

      await expect(mosaicHolding.connect(owner).setSaveFundsLockUpTime()).to.revertedWith(
        "ERR: TIMELOCK"
      );
    });

    it("Should change setSaveFundsLockUpTime", async () => {
      await mosaicHolding.connect(owner).startSaveFundsLockUpTimerChange(time);
      const count = 60 * 60 * 13;

      await expect(await mosaicHolding.connect(owner).saveFundsLockupTime()).to.be.equals(
        60 * 60 * 12
      );

      await increase_block_timestamp(count);

      await expect(mosaicHolding.connect(owner).setSaveFundsLockUpTime())
        .to.emit(mosaicHolding, "SaveFundsLockUpTimeSet")
        .withArgs(owner.address, time, 0);
      await expect(await mosaicHolding.connect(owner).saveFundsLockupTime()).to.be.equals(time);
    });

    it("Should fail to startSaveFunds", async () => {
      await expect(
        mosaicHolding.connect(owner).startSaveFunds(ERC20Contract.address, user1.address)
      ).to.revertedWith("Pausable: not paused");
    });

    it("Should startSaveFunds", async () => {
      await mosaicHolding.connect(owner).pause();
      await expect(
        mosaicHolding.connect(owner).startSaveFunds(ERC20Contract.address, user1.address)
      )
        .to.emit(mosaicHolding, "SaveFundsStarted")
        .withArgs(owner.address, ERC20Contract.address, user1.address);

      await expect(ERC20Contract.address).to.be.equals(
        await mosaicHolding.tokenAddressToSaveFunds()
      );
      await expect(user1.address).to.be.equals(await mosaicHolding.userAddressToSaveFundsTo());
    });

    it("Should fail to executeSaveFunds", async () => {
      await mosaicHolding.connect(owner).pause();
      await mosaicHolding.connect(owner).startSaveFunds(ERC20Contract.address, user1.address);

      await expect(mosaicHolding.connect(owner).executeSaveFunds()).to.revertedWith(
        "ERR: TIMELOCK"
      );
    });

    it("Should executeSaveFunds", async () => {
      const count = 60 * 60 * 13;

      await mosaicHolding.connect(owner).pause();
      await mosaicHolding.connect(owner).startSaveFunds(ERC20Contract.address, user1.address);

      await increase_block_timestamp(count);

      const contractBalance = await ERC20Contract.balanceOf(mosaicHolding.address);

      await expect(mosaicHolding.connect(owner).executeSaveFunds())
        .to.emit(mosaicHolding, "LiquidityMoved")
        .withArgs(
          await mosaicHolding.userAddressToSaveFundsTo(),
          ERC20Contract.address,
          contractBalance
        );

      await expect(await mosaicHolding.saveFundsTimer()).to.be.equals(0);
    });
  });
});

describe("Tests mainnet fork", async () => {
  beforeEach(async () => {
    await fork_network();
    await getSigners();
    await impersonateAccount(BINANCE_WALLET_ADDRESS);
    binanceWallet = await ethers.getSigner(BINANCE_WALLET_ADDRESS);

    await deployHoldingsContract();

    USDc = await ethers.getContractAt("ERC20", USDC_ADDRESS);
    DAI = await ethers.getContractAt("ERC20", DAI_ADDRESS);
    await USDc.connect(binanceWallet).transfer(mosaicHolding.address, 1000);
  });

  afterEach(async () => {
    await fork_reset();
  });

  describe("Compound", async () => {
    beforeEach(async () => {
      await deployCompoundInvestmentStrategy();
    });

    it("should not add investment if corresponding cToken address is 0", async () => {
      const USDt = await ethers.getContractAt("IERC20", USDT_ADDRESS);
      await USDt.connect(binanceWallet).transfer(mosaicHolding.address, 1000);
      const balance = await USDt.balanceOf(mosaicHolding.address);
      const investments = [{ token: USDT_ADDRESS, amount: balance }];
      await expect(
        mosaicHolding
          .connect(owner)
          .invest(investments, compoundInvestmentStrategy.address, ethers.utils.toUtf8Bytes(""))
      ).to.revertedWith("cToken address not set");
    });

    it("should add investment", async () => {
      const balanceBefore = await USDc.balanceOf(mosaicHolding.address);
      const investments = [{ token: USDC_ADDRESS, amount: balanceBefore }];
      await expect(
        mosaicHolding
          .connect(owner)
          .invest(investments, compoundInvestmentStrategy.address, ethers.utils.toUtf8Bytes(""))
      ).to.emit(mosaicHolding, "FoundsInvested");
      const balanceAfter = await USDc.balanceOf(mosaicHolding.address);
      expect(balanceAfter).to.eq(0);
    });

    it("compound: should withdraw the investment", async () => {
      const balanceBefore = await USDc.balanceOf(mosaicHolding.address);
      const investments = [{ token: USDC_ADDRESS, amount: balanceBefore }];
      await mosaicHolding
        .connect(owner)
        .invest(investments, compoundInvestmentStrategy.address, ethers.utils.toUtf8Bytes(""));
      const cUSDc = await ethers.getContractAt("CErc20Interface", CUSDC_ADDRESS);
      const cTokenBalance = await cUSDc.balanceOf(compoundInvestmentStrategy.address);

      const transaction = await mosaicHolding
        .connect(owner)
        .withdrawInvestment(
          investments,
          compoundInvestmentStrategy.address,
          ethers.utils.toUtf8Bytes("")
        );
      await transaction.wait();
      const balanceAfter = await USDc.balanceOf(mosaicHolding.address);
      expect(balanceAfter).to.eq(balanceBefore);
    });

    it("investment should grow once block number increase", async () => {
      const balanceBefore = await USDc.balanceOf(mosaicHolding.address);
      const investments = [{ token: USDC_ADDRESS, amount: balanceBefore }];
      await mosaicHolding
        .connect(owner)
        .invest(investments, compoundInvestmentStrategy.address, ethers.utils.toUtf8Bytes(""));
      const cUSDc = await ethers.getContractAt("CErc20Interface", CUSDC_ADDRESS);

      /// This require some times to mine. Hardhat not support this functionality yet.
      await mine_blocks(100000);

      const cTokenBalance = await cUSDc.balanceOf(compoundInvestmentStrategy.address);
      const exchangeRate = await cUSDc.callStatic.exchangeRateCurrent();
      investments[0].amount = Math.trunc(cTokenBalance * ethers.utils.formatEther(exchangeRate));
      const transaction = await mosaicHolding
        .connect(owner)
        .withdrawInvestment(
          investments,
          compoundInvestmentStrategy.address,
          ethers.utils.toUtf8Bytes("")
        );
      await transaction.wait();
      const balanceAfter = await USDc.balanceOf(mosaicHolding.address);
      expect(balanceAfter.toNumber()).to.greaterThan(balanceBefore.toNumber());
    });

    it("should automatically withdraw from investment strategy", async () => {
      const userDeposit = 10_000;
      const vaultConfig = await deployProxy("MosaicVaultConfig", [mosaicHolding.address]);
      const mosaicVault = await deployProxy("MosaicVault", [vaultConfig.address]);
      await mosaicHolding.setUniqRole(ethers.utils.id("MOSAIC_VAULT"), mosaicVault.address);
      await vaultConfig.connect(owner).setVault(mosaicVault.address);
      const iouTokenFactory = await (await ethers.getContractFactory("TokenFactory"))
        .connect(owner)
        .deploy(mosaicVault.address, vaultConfig.address, USDc.address);

      await vaultConfig.connect(owner).setTokenFactoryAddress(iouTokenFactory.address);
      await vaultConfig.connect(owner).addWhitelistedToken(USDC_ADDRESS, 0, initialSupply);

      await USDc.connect(binanceWallet).transfer(user1.address, userDeposit);
      const userBalance = await USDc.balanceOf(user1.address);

      await USDc.connect(user1).approve(mosaicVault.address, userDeposit);
      const blocksForActiveLiquidity = 10;
      await mosaicVault
        .connect(user1)
        .provideActiveLiquidity(userDeposit, USDC_ADDRESS, blocksForActiveLiquidity);

      await mine_blocks(blocksForActiveLiquidity);
      const mosaicHoldingBalance = await USDc.balanceOf(mosaicHolding.address);
      const investments = [{ token: USDC_ADDRESS, amount: mosaicHoldingBalance }];
      await mosaicHolding
        .connect(owner)
        .invest(investments, compoundInvestmentStrategy.address, ethers.utils.toUtf8Bytes(""));

      const usdcAddressReceipt = await vaultConfig.whitelistedTokens(USDC_ADDRESS);
      const withdrawRequestData = {
        amountOutMin: 0,
        maxDelay: 0,
        _swapToNative: false,
      };
      const { chainId } = await ethers.provider.getNetwork();

      await mosaicVault.connect(user1).withdrawLiquidityRequest(
        usdcAddressReceipt.underlyingIOUAddress, // _receiptToken
        userBalance, //_amountIn
        USDC_ADDRESS, //_tokenOut
        user1.address, //_destinationAddress
        0, //_ammID
        ethers.utils.toUtf8Bytes(""), //data
        chainId, //_destinationNetworkId
        withdrawRequestData //_withdrawRequestData
      );

      const withdrawData = {
        feePercentage: 0,
        baseFee: 0,
        investmentStrategies: [compoundInvestmentStrategy.address],
        investmentStrategiesData: [ethers.utils.toUtf8Bytes("")],
        ammId: 0,
        id: ethers.utils.formatBytes32String("random"),
        amountToSwapToNative: 0,
        minAmountOutNative: 0,
        nativeSwapperId: 0,
      };

      await expect(
        mosaicVault
          .connect(owner)
          .withdrawLiquidity(
            user1.address,
            userDeposit,
            userDeposit,
            USDC_ADDRESS,
            USDC_ADDRESS,
            0,
            withdrawData,
            ethers.utils.toUtf8Bytes("")
          )
      ).to.emit(mosaicVault, "LiquidityWithdrawn");
    });
  });

  describe("AAVE", async () => {
    const aUsdcAddress = process.env.AUSDC_ADDRESS;

    beforeEach(async () => {
      await deployAaveInvestmentStrategy();
    });

    it("aave: should make investment", async () => {
      const balanceBefore = await USDc.balanceOf(mosaicHolding.address);
      const investments = [
        {
          token: USDC_ADDRESS,
          amount: balanceBefore,
        },
      ];
      await expect(
        mosaicHolding
          .connect(owner)
          .invest(investments, aaveStrategy.address, ethers.utils.toUtf8Bytes(""))
      ).to.emit(mosaicHolding, "FoundsInvested");

      const balanceAfter = await USDc.balanceOf(mosaicHolding.address);
      expect(balanceAfter).to.eq(0);

      const aUsdcBalance = await aaveStrategy.connect(owner).investmentAmount(USDC_ADDRESS);
      expect(aUsdcBalance).to.eq(balanceBefore);
    });

    it("aave: should withdraw the investment & claim rewards", async () => {
      const balanceBefore = await USDc.balanceOf(mosaicHolding.address);
      const investments = [
        {
          token: USDC_ADDRESS,
          amount: balanceBefore,
          // tokenOut: "0xbcca60bb61934080951369a648fb03df4f96263c",
        },
      ];
      await mosaicHolding
        .connect(owner)
        .invest(investments, aaveStrategy.address, ethers.utils.toUtf8Bytes(""));

      const withdrawInvestments = [{ token: USDC_ADDRESS, amount: balanceBefore }];
      const transaction = await mosaicHolding
        .connect(owner)
        .withdrawInvestment(
          withdrawInvestments,
          aaveStrategy.address,
          ethers.utils.toUtf8Bytes("")
        );
      await transaction.wait();
      const balanceAfter = await USDc.balanceOf(mosaicHolding.address);
      expect(balanceAfter).to.eq(balanceBefore);

      await aaveStrategy.claimRewards("0xbcca60bb61934080951369a648fb03df4f96263c");

      const rewardTokenContract = await ethers.getContractAt(
        "IERC20",
        "0x4da27a545c0c5B758a6BA100e3a049001de870f5"
      );
      let rewardBalance = await rewardTokenContract.balanceOf(mosaicHolding.address);
      console.log(`   ➡️ Reward balance holding: ${rewardBalance}`);
    });

    it("aave: should claim the investment tokens", async () => {
      const balanceBefore = await USDc.balanceOf(mosaicHolding.address);
      const aUsdc = await ethers.getContractAt("IERC20", aUsdcAddress);
      const investments = [
        {
          token: USDC_ADDRESS,
          amount: balanceBefore,
          // tokenOut: "0xbcca60bb61934080951369a648fb03df4f96263c",
        },
      ];
      await mosaicHolding
        .connect(owner)
        .invest(investments, aaveStrategy.address, ethers.utils.toUtf8Bytes(""));

      await mosaicHolding
        .connect(owner)
        .claim(
          aaveStrategy.address,
          ethers.utils.defaultAbiCoder.encode(["address"], [USDC_ADDRESS])
        );

      const balanceAfter = await aUsdc.balanceOf(mosaicHolding.address);
      expect(balanceAfter).to.eq(balanceBefore);
    });
  });

  describe("SushiLP", async () => {
    const SUSHI_ROUTER = process.env.SUSHISWAP_ROUTER_ADDRESS;
    const SUSHI_ADDRESS = process.env.SUSHI_TOKEN;
    const MATIC_ADDRESS = process.env.MATIC_ADDRESS;

    let sushiToken, sushiRouter, MATIC;

    beforeEach(async () => {
      await deploySushiInvestmentStrategy();
      sushiToken = await ethers.getContractAt("IERC20", SUSHI_ADDRESS);
      MATIC = await ethers.getContractAt("ERC20", MATIC_ADDRESS);
      sushiRouter = await ethers.getContractAt("IUniswapV2Router02", SUSHI_ROUTER);
      await DAI.connect(binanceWallet).transfer(
        mosaicHolding.address,
        await DAI.balanceOf(BINANCE_WALLET_ADDRESS)
      );
      await MATIC.connect(binanceWallet).transfer(
        mosaicHolding.address,
        await MATIC.balanceOf(BINANCE_WALLET_ADDRESS)
      );
    });

    it("should add investment", async () => {
      const balanceBeforeDAI = await DAI.balanceOf(mosaicHolding.address);
      const balanceBeforeMATIC = await MATIC.balanceOf(mosaicHolding.address);

      const DaiAmount = 1_000;
      const MaticAmount = await sushiStrategy.getTokenPrice(DaiAmount, DAI_ADDRESS, MATIC_ADDRESS);

      const blockNum = await ethers.provider.getBlockNumber();
      const currentBlock = await ethers.provider.getBlock(blockNum);
      const deadline = currentBlock.timestamp + 60 * 30; // 30 min
      const data = ethers.utils.defaultAbiCoder.encode(
        ["uint256", "uint256", "uint256"],
        [deadline, DaiAmount, MaticAmount]
      );

      const investments = [
        { token: DAI_ADDRESS, amount: DaiAmount },
        { token: MATIC_ADDRESS, amount: MaticAmount },
      ];
      await expect(
        mosaicHolding.connect(owner).invest(investments, sushiStrategy.address, data)
      ).to.emit(mosaicHolding, "FoundsInvested");

      const balanceAfterDAI = await DAI.balanceOf(mosaicHolding.address);
      const balanceAfterMATIC = await MATIC.balanceOf(mosaicHolding.address);

      expect(balanceAfterDAI).to.eq(balanceBeforeDAI.sub(DaiAmount));
      expect(balanceAfterMATIC).to.eq(balanceBeforeMATIC.sub(MaticAmount));

      const pairAddress = await sushiStrategy.getPair(DAI_ADDRESS, MATIC_ADDRESS);
      expect(pairAddress).to.be.properAddress;

      const receiptTokenBalance = await sushiStrategy.investmentAmount(pairAddress);
      expect(receiptTokenBalance).to.not.equal(0);
    });

    it("sushi: should withdraw the investment", async () => {
      const DaiAmount = 1_000;
      const MaticAmount = await sushiStrategy.getTokenPrice(DaiAmount, DAI_ADDRESS, MATIC_ADDRESS);
      const blockNum = await ethers.provider.getBlockNumber();
      const currentBlock = await ethers.provider.getBlock(blockNum);
      const deadline = currentBlock.timestamp + 60 * 30; // 30 min
      let data = ethers.utils.defaultAbiCoder.encode(
        ["uint256", "uint256", "uint256"],
        [deadline, DaiAmount, MaticAmount]
      );

      const investments = [
        { token: DAI_ADDRESS, amount: DaiAmount },
        { token: MATIC_ADDRESS, amount: MaticAmount },
      ];
      await expect(
        mosaicHolding.connect(owner).invest(investments, sushiStrategy.address, data)
      ).to.emit(mosaicHolding, "FoundsInvested");
      const balanceBeforeDAI = await DAI.balanceOf(mosaicHolding.address);
      const balanceBeforeMATIC = await MATIC.balanceOf(mosaicHolding.address);

      const pairAddress = await sushiStrategy.getPair(DAI_ADDRESS, MATIC_ADDRESS);
      const receiptTokenBalanceBefore = await sushiStrategy.investmentAmount(pairAddress);
      data = ethers.utils.defaultAbiCoder.encode(
        ["uint256", "uint256"],
        [deadline, receiptTokenBalanceBefore]
      );

      const withdrawInvestments = [
        { token: DAI_ADDRESS, amount: 0 }, // min amount
        { token: MATIC_ADDRESS, amount: 0 }, // min amount
      ];
      await expect(
        mosaicHolding
          .connect(owner)
          .withdrawInvestment(withdrawInvestments, sushiStrategy.address, data)
      ).to.emit(mosaicHolding, "InvestmentWithdrawn");

      const receiptTokenBalanceAfter = await sushiStrategy.investmentAmount(pairAddress);
      expect(receiptTokenBalanceAfter).to.eq(0);

      const balanceAfterDAI = await DAI.balanceOf(mosaicHolding.address);
      const balanceAfterMATIC = await MATIC.balanceOf(mosaicHolding.address);
      expect(balanceAfterDAI).to.be.gt(balanceBeforeDAI);
      expect(balanceAfterMATIC).to.be.gt(balanceBeforeMATIC);
    });
  });
});

async function deployHoldingsContract() {
  const mosaicHoldingFactory = await ethers.getContractFactory("MosaicHolding");
  mosaicHolding = await upgrades.deployProxy(mosaicHoldingFactory, [owner.address]);
  await mosaicHolding.deployed();
}

async function deployAaveInvestmentStrategy() {
  const AaveInvestmentStrategy = await ethers.getContractFactory("AaveInvestmentStrategy");
  aaveStrategy = await upgrades.deployProxy(AaveInvestmentStrategy, [
    owner.address,
    mosaicHolding.address,
    process.env.LENDING_POOL_ADDRESS_PROVIDER,
    process.env.INCENTIVES_CONTROLLER,
  ]);
  await aaveStrategy.deployed();
  await mosaicHolding.addInvestmentStrategy(aaveStrategy.address);
}

async function deploySushiInvestmentStrategy() {
  const SushiInvestmentStrategy = await ethers.getContractFactory("SushiswapLiquidityProvider");
  sushiStrategy = await upgrades.deployProxy(SushiInvestmentStrategy, [
    owner.address,
    mosaicHolding.address,
    process.env.SUSHISWAP_ROUTER_ADDRESS,
    process.env.SUSHISWAP_V2_FACTORY,
  ]);
  await sushiStrategy.deployed();
  await mosaicHolding.addInvestmentStrategy(sushiStrategy.address);
}

async function deployCompoundInvestmentStrategy() {
  const compoundInvestmentStrategyFactory = await ethers.getContractFactory(
    "CompoundInvestmentStrategy"
  );
  compoundInvestmentStrategy = await upgrades.deployProxy(compoundInvestmentStrategyFactory, [
    owner.address,
    mosaicHolding.address,
    process.env.COMPTROLLER_ADDRESS,
    process.env.COMP_ADDRESS,
  ]);
  await compoundInvestmentStrategy.deployed();
  await compoundInvestmentStrategy.setCTokensAddress(USDC_ADDRESS, CUSDC_ADDRESS);
  await mosaicHolding.addInvestmentStrategy(compoundInvestmentStrategy.address);
}

async function getSigners() {
  [owner, vaultAccount, vaultAccount2, rebalancingBot, user1] = await ethers.getSigners();
}
