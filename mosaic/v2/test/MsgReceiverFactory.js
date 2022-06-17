const { expect } = require("./test_env.js");
const { upgrades, ethers } = require("hardhat");

describe("Msg Receiver Factory", async () => {
  let msgReceiverFactory, msgReceiver, erc20_token, erc20_token2, amount;
  let deployer, user, relayer;
  let relayerAddress, userAddress, deployerAddress;

  beforeEach(async () => {
    [deployer, user, relayer] = await ethers.getSigners();
    deployerAddress = await deployer.getAddress();
    userAddress = await user.getAddress();
    relayerAddress = await relayer.getAddress();
    amount = "100000000";

    const MsgReceiverFactory = await ethers.getContractFactory("MsgReceiverFactory");
    msgReceiverFactory = await upgrades.deployProxy(MsgReceiverFactory, [relayerAddress]);

    const ERC20Token = await ethers.getContractFactory("SampleTokenERC20");

    erc20_token = await ERC20Token.deploy("yoyo", "yy", 10000000000);
    const ERC20Token2 = await ethers.getContractFactory("SampleTokenERC20");

    erc20_token2 = await ERC20Token2.deploy("yoyo1", "yy1", 10000000000);
    //await erc20_token.connect(deployer).transfer(msgSender.address, amount);
  });

  it("contract creation", async () => {
    expect(await msgReceiverFactory.owner()).to.equal(deployerAddress);
  });

  it("create MsgReceiver", async () => {
    const expectedMsgReceiverAddress = await msgReceiverFactory
      .connect(deployer)
      .callStatic.createPersona(userAddress);
    await msgReceiverFactory.connect(deployer).createPersona(userAddress);
    const msgReceiverAddress = await msgReceiverFactory.retrievePersona(userAddress);
    expect(expectedMsgReceiverAddress).to.equal(msgReceiverAddress);
    await expect(
      msgReceiverFactory.connect(deployer).createPersona(userAddress)
    ).to.be.revertedWith("Already created");
  });

  it("create MsgReceiver by other user", async () => {
    const expectedMsgReceiverAddress = await msgReceiverFactory
      .connect(user)
      .callStatic.createPersona(userAddress);
    await msgReceiverFactory.connect(user).createPersona(userAddress);
    const msgReceiverAddress = await msgReceiverFactory.retrievePersona(userAddress);
    expect(expectedMsgReceiverAddress).to.equal(msgReceiverAddress);
  });

  it("remove MsgReceiver", async () => {
    const expectedMsgReceiverAddress = await msgReceiverFactory
      .connect(deployer)
      .callStatic.createPersona(userAddress);
    await msgReceiverFactory.connect(deployer).createPersona(userAddress);
    const msgReceiverAddress = await msgReceiverFactory.retrievePersona(userAddress);
    expect(expectedMsgReceiverAddress).to.equal(msgReceiverAddress);
    await msgReceiverFactory.connect(deployer).removePersona(userAddress);
    expect(await msgReceiverFactory.retrievePersona(userAddress)).to.equal(
      "0x0000000000000000000000000000000000000000"
    );
  });

  it("setRelayer", async () => {
    await msgReceiverFactory.connect(deployer).setRelayer(userAddress);
    expect(await msgReceiverFactory.relayer()).to.equal(userAddress);
  });

  it("feeToken", async () => {
    const tokenAddress = userAddress;
    await msgReceiverFactory.connect(deployer).addFeeToken(tokenAddress);
    expect((await msgReceiverFactory.whitelistedFeeTokens(tokenAddress)).toString()).to.equal(
      "true"
    );
    await msgReceiverFactory.connect(deployer).removeFeeToken(tokenAddress);
    expect((await msgReceiverFactory.whitelistedFeeTokens(tokenAddress)).toString()).to.equal(
      "false"
    );
  });
});
