const { expect } = require("./test_env.js");
const { ethers, upgrades } = require("hardhat");

const RANDOM_ADDRESS = ethers.utils.getAddress("0xffffffffffffffffffffffffffffffffffffffff");

let owner, admin, user;
let bridgeAggregatorContract;

before(async () => {
  [owner, admin, user] = await ethers.getSigners();
});

beforeEach(async () => {
  const bridgeAggregatorContractFactory = await ethers.getContractFactory("BridgeAggregator");
  bridgeAggregatorContract = await upgrades.deployProxy(bridgeAggregatorContractFactory, [
    RANDOM_ADDRESS,
  ]);
  await bridgeAggregatorContract.deployed();
});

describe("Test permissions", async () => {
  it("should add bridge by owner", async () => {
    await bridgeAggregatorContract.connect(owner).addBridge(1, 0, RANDOM_ADDRESS);
    const bridgeAddress = await bridgeAggregatorContract.supportedBridges(1, 0);
    expect(bridgeAddress).to.be.properAddress;
  });

  it("bridge with same id not allowed", async () => {
    await bridgeAggregatorContract.connect(owner).addBridge(1, 0, RANDOM_ADDRESS);
    await expect(
      bridgeAggregatorContract
        .connect(owner)
        .addBridge(1, 0, "0xafffffffffffffffffffffffffffffffffffffff")
    ).revertedWith("Bridge already exist");
  });

  it("only owner should remove a bridge", async () => {
    await bridgeAggregatorContract.connect(owner).addBridge(1, 0, RANDOM_ADDRESS);
    await bridgeAggregatorContract.connect(owner).removeBridge(1, 0);
    const bridgeAddress = await bridgeAggregatorContract.supportedBridges(1, 0);
    expect(bridgeAddress).to.eq(ethers.constants.AddressZero);
  });

  it("user not allow to remove a bridge", async () => {
    await bridgeAggregatorContract.connect(owner).addBridge(1, 0, RANDOM_ADDRESS);
    await expect(bridgeAggregatorContract.connect(user).removeBridge(1, 0)).revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("should not add bridge by other user", async () => {
    await expect(
      bridgeAggregatorContract.connect(user).addBridge(1, 0, RANDOM_ADDRESS)
    ).revertedWith("Ownable: caller is not the owner");
  });
});
