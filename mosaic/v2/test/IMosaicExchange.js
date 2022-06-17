const { expect } = require("./test_env.js");
const { ethers, network } = require("hardhat");
const { fork_network, fork_reset } = require("./utils/network_fork");

const SUSHISWAP_ROUTER_ADDRESS = process.env.SUSHISWAP_ROUTER_ADDRESS;
const CURVE_ADDRESS_PROVIDER_ADDRESS = process.env.CURVE_ADDRESS_PROVIDER_ADDRESS;
const BALANCER_EXCHANGE_PROXY_ADDRESS = process.env.BALANCER_EXCHANGE_PROXY_ADDRESS;

const WETH_ADDRESS = process.env.WETH_ADDRESS;
const USDC_ADDRESS = process.env.USDC_ADDRESS;
const DAI_ADDRESS = process.env.DAI_ADDRESS;

let owner;
let sushiswapWrapper, curveWrapper, usdcContract;

describe("IMosaicExchange", async () => {
  async function buyUSDc() {
    // initialize the sushiswap router to swap ether for some USDC
    const sushiswapRouter = await ethers.getContractAt(
      "IUniswapV2Router02",
      SUSHISWAP_ROUTER_ADDRESS
    );
    // exchange eth for some usdc tokens using sushiswap
    const blockNum = await ethers.provider.getBlockNumber();
    const curBlock = await ethers.provider.getBlock(blockNum);
    sushiswapRouter
      .connect(owner)
      .swapExactETHForTokens(
        0,
        [WETH_ADDRESS, USDC_ADDRESS],
        owner.address,
        curBlock.timestamp + 1000,
        {
          value: ethers.utils.parseEther("1").toHexString(),
        }
      );
  }

  beforeEach(async () => {
    /// Use mainnet fork as provider
    await fork_network();

    [owner] = await ethers.getSigners();

    // set 10 ether as the owner balance
    await network.provider.send("hardhat_setBalance", [
      owner.address,
      ethers.utils.parseEther("10").toHexString(),
    ]);

    usdcContract = await ethers.getContractAt("IERC20", USDC_ADDRESS);

    // deploy the sushiswap wrapper contract and initialize it
    const sushiswapWrapperFactory = await ethers.getContractFactory("SushiswapWrapper");
    sushiswapWrapper = await sushiswapWrapperFactory.connect(owner).deploy();
    await sushiswapWrapper.connect(owner).initialize(SUSHISWAP_ROUTER_ADDRESS);
    // console.log("Sushiswap wrapper deploy at: ", sushiswapWrapper.address);
    await usdcContract
      .connect(owner)
      .approve(sushiswapWrapper.address, new ethers.BigNumber.from(1000).mul(10 ** 6));
  });

  after(async () => {
    await fork_reset();
  });

  describe("testing sushiswap wrapper swap method", async () => {
    it("failed swap between usdc and dai", async () => {
      await expect(
        sushiswapWrapper.connect(owner).swap(USDC_ADDRESS, DAI_ADDRESS, 10 ** 6, 0, [])
      ).to.revertedWith("ERC20: transfer amount exceeds balance");
    });

    it("successful fetch expected amount of dai received", async () => {
      const amountsOut = await sushiswapWrapper
        .connect(owner)
        .getAmountsOut(USDC_ADDRESS, DAI_ADDRESS, 10 ** 6, []);
      expect(amountsOut).to.above(0);
    });

    it("successful swap usdc to dai", async () => {
      await buyUSDc();

      const prevUsdcBalance = await usdcContract.connect(owner).balanceOf(owner.address);
      const timestamp = await getCurrentTimestamp();
      const amount = new ethers.BigNumber.from(10).mul(10 ** 6);
      const deadline = ethers.utils.defaultAbiCoder.encode(["uint"], [timestamp + 15]);
      await sushiswapWrapper.connect(owner).swap(USDC_ADDRESS, DAI_ADDRESS, amount, 0, deadline);

      const curUsdcBalance = await usdcContract.connect(owner).balanceOf(owner.address);
      expect(curUsdcBalance).to.be.equal(prevUsdcBalance.sub(amount));

      const daiContract = await ethers.getContractAt("IERC20", DAI_ADDRESS);
      const daiBalance = await daiContract.connect(owner).balanceOf(owner.address);
      expect(daiBalance.gte(0)).to.equal(true);
    });
  });

  describe("testing curve wrapper swap method", async () => {
    beforeEach(async function () {
      // deploy the curveWrapper contract and initialize it
      const curveWrapperFactory = await ethers.getContractFactory("CurveWrapper");
      curveWrapper = await curveWrapperFactory.connect(owner).deploy();
      await curveWrapper.connect(owner).initialize();

      // approve transfer of 1000 usdc token to curveWrapper contract
      await usdcContract
        .connect(owner)
        .approve(curveWrapper.address, new ethers.BigNumber.from(10000).mul(10 ** 6));
    });

    it("successful swap usdc to dai", async () => {
      await buyUSDc();
      const prevUsdcBalance = await usdcContract.balanceOf(owner.address);
      const dt = ethers.utils.defaultAbiCoder.encode(
        ["address", "uint256", "int128", "int128", "uint256", "uint256"],
        ["0xbebc44782c7db0a1a60cb6fe97d0b483032ff1c7", "0", "1", "0", "1", "0"]
      );
      await curveWrapper.connect(owner).swap(USDC_ADDRESS, DAI_ADDRESS, prevUsdcBalance, 0, dt);

      const curUsdcBalance = await usdcContract.balanceOf(owner.address);
      expect(curUsdcBalance.isZero()).to.equal(true);

      const daiContract = await ethers.getContractAt("IERC20", DAI_ADDRESS);
      const daiBalance = await daiContract.balanceOf(owner.address);
      expect(daiBalance.gte(0)).to.equal(true);
    });

    it("successful fetch expected amount of dai received", async () => {
      const dt = ethers.utils.defaultAbiCoder.encode(
        ["address", "uint256", "int128", "int128", "uint256", "uint256"],
        ["0xbebc44782c7db0a1a60cb6fe97d0b483032ff1c7", "0", "1", "0", "1", "0"]
      );

      const amountsOut = await curveWrapper
        .connect(owner)
        .getAmountsOut(USDC_ADDRESS, DAI_ADDRESS, 10 ** 6, dt);
      expect(amountsOut).to.above(0);
    });

    it("failed swap between usdc and dai", async () => {
      const dt = ethers.utils.defaultAbiCoder.encode(
        ["address", "uint256", "int128", "int128", "uint256", "uint256"],
        ["0xbebc44782c7db0a1a60cb6fe97d0b483032ff1c7", "0", "1", "0", "1", "0"]
      );
      await expect(
        curveWrapper.connect(owner).swap(USDC_ADDRESS, DAI_ADDRESS, 10 ** 6, 0, dt)
      ).to.be.revertedWith("ERC20: transfer amount exceeds balance");
    });
  });

  describe("testing balancerV1 wrapper swap method", async () => {
    let balancerWrapper;
    beforeEach(async function () {
      const wrapperFactory = await ethers.getContractFactory("BalancerV1Wrapper");
      balancerWrapper = await wrapperFactory.connect(owner).deploy();
      await balancerWrapper.connect(owner).initialize(BALANCER_EXCHANGE_PROXY_ADDRESS);

      // approve transfer of 1000 usdc token to wrapper contract
      await usdcContract
        .connect(owner)
        .approve(balancerWrapper.address, new ethers.BigNumber.from(10000).mul(10 ** 6));
    });

    it("successful fetch expected amount of dai received", async () => {
      const amountsOut = await balancerWrapper
        .connect(owner)
        .getAmountsOut(
          USDC_ADDRESS,
          DAI_ADDRESS,
          10 ** 6,
          ethers.utils.defaultAbiCoder.encode(["uint"], [5])
        );
      expect(amountsOut).to.above(0);
    });

    it("failed swap between usdc and dai", async () => {
      await expect(
        balancerWrapper
          .connect(owner)
          .swap(
            USDC_ADDRESS,
            DAI_ADDRESS,
            10 ** 6,
            0,
            ethers.utils.defaultAbiCoder.encode(["uint"], [5])
          )
      ).to.be.revertedWith("ERC20: transfer amount exceeds balance");
    });

    it("swap between usdc and dai", async () => {
      await buyUSDc();

      const prevUsdcBalance = await usdcContract.connect(owner).balanceOf(owner.address);

      const amount = new ethers.BigNumber.from(10).mul(10 ** 6);

      await balancerWrapper
        .connect(owner)
        .swap(
          USDC_ADDRESS,
          DAI_ADDRESS,
          amount,
          0,
          ethers.utils.defaultAbiCoder.encode(["uint"], [5])
        );

      const curUsdcBalance = await usdcContract.connect(owner).balanceOf(owner.address);
      expect(curUsdcBalance).to.be.equal(prevUsdcBalance.sub(amount));

      const daiContract = await ethers.getContractAt("IERC20", DAI_ADDRESS);
      const daiBalance = await daiContract.connect(owner).balanceOf(owner.address);
      expect(daiBalance.gte(0)).to.equal(true);
    });
  });

  describe("testing BalancerV2 wrapper swap method", async () => {
    let balancerWrapper;
    const allowance = new ethers.BigNumber.from(1000).mul(10 ** 6);
    const poolId = process.env.BALANCER_WETH_USDC_POOL_ID_50_50;

    beforeEach(async function () {
      const wrapperFactory = await ethers.getContractFactory("BalancerVaultV2Wrapper");
      balancerWrapper = await wrapperFactory.connect(owner).deploy();
      await balancerWrapper.connect(owner).initialize(process.env.BALANCER_VAULT_V2);

      await usdcContract.connect(owner).approve(balancerWrapper.address, allowance);
    });

    it("failed swap between usdc and dai", async () => {
      const timestamp = await getCurrentTimestamp();
      await expect(
        balancerWrapper
          .connect(owner)
          .swap(
            USDC_ADDRESS,
            WETH_ADDRESS,
            allowance,
            0,
            ethers.utils.defaultAbiCoder.encode(["bytes32", "uint256"], [poolId, timestamp + 15])
          )
      ).to.be.revertedWith("ERC20: transfer amount exceeds balance");
    });

    it("swap between usdc and dai", async () => {
      await buyUSDc();

      const prevUsdcBalance = await usdcContract.connect(owner).balanceOf(owner.address);

      const timestamp = await getCurrentTimestamp();

      await balancerWrapper
        .connect(owner)
        .swap(
          USDC_ADDRESS,
          WETH_ADDRESS,
          allowance,
          0,
          ethers.utils.defaultAbiCoder.encode(["bytes32", "uint256"], [poolId, timestamp + 15])
        );

      const curUsdcBalance = await usdcContract.connect(owner).balanceOf(owner.address);
      expect(curUsdcBalance).to.be.equal(prevUsdcBalance.sub(allowance));

      const wethContract = await ethers.getContractAt("IERC20", WETH_ADDRESS);
      const wethBalance = await wethContract.connect(owner).balanceOf(owner.address);
      expect(wethBalance.gte(0)).to.equal(true);
    });
  });
});

async function getCurrentTimestamp() {
  const blockNum = await ethers.provider.getBlockNumber();
  const curBlock = await ethers.provider.getBlock(blockNum);
  return curBlock.timestamp;
}
