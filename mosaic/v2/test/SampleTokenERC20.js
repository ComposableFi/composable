const { ethers } = require("hardhat");

const { expect } = require("./test_env.js");

describe("ERC20 general test", () => {
  let account1;
  let account2;
  const initialSupply = 1000;
  const amountToSend1 = 500;
  const amountToSend2 = initialSupply - amountToSend1;
  let ERC20Contract;

  before(async function () {
    [account1, account2, account3] = await ethers.getSigners();
    ERC20Contract = await (await ethers.getContractFactory("SampleTokenERC20"))
      .connect(account1)
      .deploy("Token", "TKN", initialSupply);
  });

  afterEach("logging", async function () {});

  it("Token contract has correct totalSupply", async () => {
    const totalSupply = await ERC20Contract.totalSupply();
    expect(totalSupply).to.eq(ethers.BigNumber.from(initialSupply));
  });

  it("Owner has correct token balance", async () => {
    const ownerAddress = await account1.getAddress();
    const ownerBalance = await ERC20Contract.balanceOf(ownerAddress);
    expect(ownerBalance).to.eq(ethers.BigNumber.from(initialSupply));
  });

  it("Has correct name", async () => {
    const name = await ERC20Contract.name();
    expect(name).to.eql("Token");
  });

  it("Has correct symbol", async () => {
    const symbol = await ERC20Contract.symbol();
    expect(symbol).to.eql("TKN");
  });

  it("Has correct decimals", async () => {
    const decimals = await ERC20Contract.decimals();
    expect(decimals).to.eql(18);
  });

  it("Send tokens exceeding amount", async function () {
    const sender = account1;
    const recipientAddress = await account2.getAddress();
    const amount = initialSupply + 1;

    await expect(
      ERC20Contract.connect(sender).transfer(recipientAddress, amount)
    ).to.be.revertedWith("ERC20: transfer amount exceeds balance");
  });

  it("Send tokens successful", async function () {
    const sender = account1;
    const senderAddress = await sender.getAddress();
    const recipientAddress = await account2.getAddress();
    const senderBalanceBefore = await ERC20Contract.balanceOf(senderAddress);

    await expect(ERC20Contract.connect(sender).transfer(recipientAddress, amountToSend1))
      .to.emit(ERC20Contract, "Transfer")
      .withArgs(senderAddress, recipientAddress, amountToSend1);

    const senderBalance = await ERC20Contract.balanceOf(senderAddress);
    const recipientBalance = await ERC20Contract.balanceOf(recipientAddress);

    expect(senderBalance).to.eq(senderBalanceBefore.sub(amountToSend1));
    expect(recipientBalance).to.eq(ethers.BigNumber.from(amountToSend1));
  });

  it("Send tokens without approve failed", async function () {
    const senderAddress = await account1.getAddress();
    const recipientAddress = await account2.getAddress();
    const spender = account3;

    await expect(
      ERC20Contract.connect(spender).transferFrom(senderAddress, recipientAddress, amountToSend2)
    ).to.be.revertedWith("ERC20: transfer amount exceeds allowance");
  });

  it("Send tokens with approve successful", async function () {
    const sender = account1;
    const spender = account3;
    const senderAddress = await sender.getAddress();
    const recipientAddress = await account2.getAddress();
    const spenderAddress = await spender.getAddress();

    await ERC20Contract.connect(sender).approve(spenderAddress, amountToSend2);

    const senderBalanceBefore = await ERC20Contract.balanceOf(senderAddress);
    const recipientBalanceBefore = await ERC20Contract.balanceOf(recipientAddress);

    await expect(
      ERC20Contract.connect(spender).transferFrom(senderAddress, recipientAddress, amountToSend2)
    )
      .to.emit(ERC20Contract, "Transfer")
      .withArgs(senderAddress, recipientAddress, amountToSend2);

    const senderBalance = await ERC20Contract.balanceOf(senderAddress);
    const recipientBalance = await ERC20Contract.balanceOf(recipientAddress);

    expect(senderBalance).to.eql(senderBalanceBefore.sub(amountToSend2));
    expect(recipientBalance).to.eql(recipientBalanceBefore.add(amountToSend2));
  });
});
