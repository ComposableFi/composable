const { expect } = require("./test_env.js");
const { upgrades, ethers } = require("hardhat");

describe("Msg Receiver", async () => {
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

    const MsgReceiver = await ethers.getContractFactory("MsgReceiver");
    msgReceiver = await MsgReceiver.deploy(userAddress, msgReceiverFactory.address);

    const ERC20Token = await ethers.getContractFactory("SampleTokenERC20");

    erc20_token = await ERC20Token.deploy("yoyo", "yy", 10000000000);
    const ERC20Token2 = await ethers.getContractFactory("SampleTokenERC20");

    erc20_token2 = await ERC20Token2.deploy("yoyo1", "yy1", 10000000000);
    //await erc20_token.connect(deployer).transfer(msgSender.address, amount);
  });

  it("contract creation", async () => {
    expect(await msgReceiver.getRelayer()).to.equal(relayerAddress);
    expect(await msgReceiver.user()).to.equal(userAddress);
    expect(await msgReceiver.msgReceiverFactory()).to.equal(msgReceiverFactory.address);
  });

  it("forward call", async () => {
    const DummySetterFactory = await ethers.getContractFactory("DummySetter");
    const dummySetter = await DummySetterFactory.deploy();
    await msgReceiverFactory.addFeeToken(erc20_token.address);
    const ABI = ["function setX(uint256)"];
    const iface = new ethers.utils.Interface(ABI);
    const callData = iface.encodeFunctionData("setX", [1]);
    const feeAddress = deployer.getAddress();
    const id = ethers.utils.formatBytes32String("1");
    await expect(
      msgReceiver.forwardCall(
        100,
        erc20_token.address,
        feeAddress,
        id,
        ethers.utils.parseEther("1"),
        dummySetter.address,
        callData
      )
    ).to.be.revertedWith("Not enough tokens for the fee");

    await erc20_token.transfer(msgReceiver.address, 1000);
    await msgReceiver.forwardCall(
      100,
      erc20_token.address,
      feeAddress,
      id,
      ethers.utils.parseEther("1"),
      dummySetter.address,
      callData
    );
    expect((await dummySetter.x()).toString()).to.equal("1");
  });

  it("save eth", async () => {
    const receiverAddress = await deployer.getAddress();
    const id = ethers.utils.formatBytes32String("1");
    await msgReceiverFactory.addFeeToken(erc20_token.address);
    await erc20_token.transfer(msgReceiver.address, 100);
    await expect(
      msgReceiver
        .connect(user)
        .saveETH(receiverAddress, 100, id, erc20_token.address, 10, receiverAddress)
    ).to.be.revertedWith("Exceeds balance");
    await expect(
      msgReceiver
        .connect(deployer)
        .saveETH(receiverAddress, 100, id, erc20_token.address, 10, receiverAddress)
    ).to.be.revertedWith("Only user or relayer");
    await deployer.sendTransaction({
      to: msgReceiver.address,
      value: ethers.utils.parseEther("1"), // 1 ether
    });
    await msgReceiver
      .connect(user)
      .saveETH(receiverAddress, 100, id, erc20_token.address, 10, receiverAddress);
  });

  it("save nft", async () => {
    const id = ethers.utils.formatBytes32String("1");
    await msgReceiverFactory.addFeeToken(erc20_token.address);
    await erc20_token.transfer(msgReceiver.address, 100);
    const SampleNftFactory = await ethers.getContractFactory("SampleNFT");
    const receiverAddress = await deployer.getAddress();
    const nft = await SampleNftFactory.deploy("test", "t");
    await nft.mintNft(msgReceiver.address, "url");
    await expect(
      msgReceiver.saveNFT(
        nft.address,
        2,
        receiverAddress,
        id,
        erc20_token.address,
        10,
        receiverAddress
      )
    ).to.be.revertedWith("Only user or relayer");
    await msgReceiver
      .connect(user)
      .saveNFT(nft.address, 1, receiverAddress, id, erc20_token.address, 10, receiverAddress);
  });

  it("save tokens", async () => {
    const id = ethers.utils.formatBytes32String("1");
    await msgReceiverFactory.addFeeToken(erc20_token2.address);
    await erc20_token2.transfer(msgReceiver.address, 10);
    const receiverAddress = await deployer.getAddress();
    await expect(
      msgReceiver
        .connect(user)
        .saveTokens(
          erc20_token2.address,
          receiverAddress,
          100,
          id,
          erc20_token.address,
          10,
          receiverAddress
        )
    ).to.be.revertedWith("Exceeds balance");
    await erc20_token.transfer(msgReceiver.address, 100);
    await expect(
      msgReceiver
        .connect(user)
        .saveTokens(
          erc20_token2.address,
          receiverAddress,
          100,
          id,
          erc20_token.address,
          10,
          receiverAddress
        )
    ).to.be.revertedWith("Exceeds balance");
    await msgReceiver
      .connect(user)
      .saveTokens(
        erc20_token.address,
        receiverAddress,
        50,
        id,
        erc20_token.address,
        10,
        receiverAddress
      );
    await msgReceiver
      .connect(user)
      .saveTokens(
        erc20_token.address,
        receiverAddress,
        50,
        id,
        erc20_token.address,
        10,
        receiverAddress
      );
    await expect(
      msgReceiver
        .connect(user)
        .saveTokens(
          erc20_token.address,
          receiverAddress,
          1,
          id,
          erc20_token.address,
          10,
          receiverAddress
        )
    ).to.be.revertedWith("Exceeds balance");
  });

  it("fee token configuration", async () => {
    await expect(
      msgReceiverFactory.connect(user).addFeeToken(erc20_token.address)
    ).to.be.revertedWith("Ownable: caller is not the owner");
    await msgReceiverFactory.addFeeToken(erc20_token.address);
    expect(
      (await msgReceiverFactory.whitelistedFeeTokens(erc20_token.address)).toString()
    ).to.equal("true");
  });

  it("approve token", async () => {
    const userAddress = await user.getAddress();
    await msgReceiverFactory.addFeeToken(erc20_token.address);
    const feeAddress = await deployer.getAddress();
    const id = ethers.utils.formatBytes32String("1");
    await expect(
      msgReceiver
        .connect(user)
        .approveERC20Token(
          100,
          erc20_token.address,
          feeAddress,
          id,
          erc20_token.address,
          userAddress,
          123456
        )
    ).to.be.revertedWith("Only owner or relayer");
    await expect(
      msgReceiver
        .connect(relayer)
        .approveERC20Token(
          100,
          erc20_token.address,
          feeAddress,
          id,
          erc20_token.address,
          userAddress,
          123456
        )
    ).to.be.revertedWith("Not enough tokens for the fee");
    await erc20_token.transfer(msgReceiver.address, 100);
    await msgReceiver
      .connect(relayer)
      .approveERC20Token(
        100,
        erc20_token.address,
        feeAddress,
        id,
        erc20_token.address,
        userAddress,
        123456
      );
    expect((await erc20_token.allowance(msgReceiver.address, userAddress)).toString()).to.equal(
      "123456"
    );
  });

  it("approve token and forward call", async () => {
    const DummySetterFactory = await ethers.getContractFactory("DummySetter");
    const dummySetter = await DummySetterFactory.deploy();
    await erc20_token2.transfer(msgReceiver.address, 100000);
    await msgReceiverFactory.addFeeToken(erc20_token.address);
    const ABI = ["function transferTokensAndSetX(address,uint256,uint256)"];
    const iface = new ethers.utils.Interface(ABI);
    const callData = iface.encodeFunctionData("transferTokensAndSetX", [
      erc20_token2.address,
      10000,
      1,
    ]);
    const feeAddress = deployer.getAddress();
    const id = ethers.utils.formatBytes32String("1");
    await expect(
      msgReceiver.forwardCall(
        100,
        erc20_token.address,
        feeAddress,
        id,
        ethers.utils.parseEther("1"),
        dummySetter.address,
        callData
      )
    ).to.be.revertedWith("Not enough tokens for the fee");
    await expect(
      msgReceiver.approveERC20TokenAndForwardCall(
        100,
        erc20_token.address,
        feeAddress,
        erc20_token2.address,
        10000,
        id,
        ethers.utils.parseEther("1"),
        dummySetter.address,
        callData
      )
    ).to.be.revertedWith("Not enough tokens for the fee");

    await erc20_token.transfer(msgReceiver.address, 1000);
    await msgReceiver.approveERC20TokenAndForwardCall(
      100,
      erc20_token.address,
      feeAddress,
      erc20_token2.address,
      10000,
      id,
      ethers.utils.parseEther("1"),
      dummySetter.address,
      callData
    );
    expect((await dummySetter.x()).toString()).to.equal("1");
    expect((await erc20_token2.balanceOf(dummySetter.address)).toString()).to.equal("10000");
  });
});
