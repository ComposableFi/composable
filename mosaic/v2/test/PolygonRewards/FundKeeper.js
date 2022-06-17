const { expect } = require("../test_env.js");

describe("Test network", async () => {
  let fundKeeper, amountToSend, address;
  beforeEach(async () => {
    [owner, tokensHolder] = await ethers.getSigners();

    const FundKeeper = await ethers.getContractFactory("FundKeeper");
    fundKeeper = await FundKeeper.deploy();

    await owner.sendTransaction({
      to: fundKeeper.address,
      value: ethers.utils.parseEther("1"), // 1 ether
    });

    amountToSend = "10000";
    address = "";
  });

  it("Should fail if sender is not admin", async () => {
    await expect(fundKeeper.connect(tokensHolder).setAmountToSend(amountToSend)).to.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("Should set amount to send", async () => {
    await expect(fundKeeper.connect(owner).setAmountToSend(amountToSend))
      .to.emit(fundKeeper, "NewAmountToSend")
      .withArgs(amountToSend);

    const setAmountToSendInContract = await fundKeeper.connect(owner).amountToSend();
    expect(setAmountToSendInContract.toString()).to.eq(amountToSend);
  });

  it("Should fail if sender is not admin", async () => {
    await expect(fundKeeper.connect(tokensHolder).sendFunds(address)).to.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("Should send funds to user", async () => {
    const userBalanceBeforeTx = await ethers.provider.getBalance(address);
    await expect(fundKeeper.connect(owner).sendFunds(address))
      .to.emit(fundKeeper, "FundSent")
      .withArgs("50000000000000000", address);

    const userBalanceAfterTx = await ethers.provider.getBalance(address);

    expect(userBalanceBeforeTx).to.eq(0);
    expect(userBalanceAfterTx).to.eq("50000000000000000");
  });

  it("Should fail if user already receive reward", async () => {
    await fundKeeper.connect(owner).sendFunds(address);
    await expect(fundKeeper.connect(owner).sendFunds(address)).to.revertedWith(
      "reward already sent"
    );
  });

  it("Should fail if amount to send is above contract balance", async () => {
    await fundKeeper.connect(owner).setAmountToSend("2000000000000000000");
    await expect(fundKeeper.connect(owner).sendFunds(address)).to.revertedWith(
      "Contract balance low"
    );
  });
});
