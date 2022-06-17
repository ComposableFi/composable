const { expect } = require("./test_env.js");
const { upgrades, ethers } = require("hardhat");

describe("Msg Sender", async () => {
  let msgSender, erc20_token, erc20_token2, amount;

  beforeEach(async () => {
    [deployer, receiver] = await ethers.getSigners();
    amount = "100000000";

    const MsgSender = await ethers.getContractFactory("MsgSender");
    msgSender = await upgrades.deployProxy(MsgSender);

    await msgSender.deployed();
    const ERC20Token = await ethers.getContractFactory("SampleTokenERC20");

    erc20_token = await ERC20Token.deploy("yoyo", "yy", 10000000000);
    const ERC20Token2 = await ethers.getContractFactory("SampleTokenERC20");

    erc20_token2 = await ERC20Token2.deploy("yoyo1", "yy1", 10000000000);
    await erc20_token.connect(deployer).transfer(msgSender.address, amount);
  });

  it("Should fail to register cross chain function to a non whitelisted contract", async () => {
    const _data = {
      destinationContract: ethers.constants.AddressZero,
      destinationData: ethers.constants.HashZero,
      fallbackContract: ethers.constants.AddressZero,
      fallbackData: ethers.constants.HashZero,
    };

    await msgSender.connect(deployer).addNetwork(1);
    await msgSender.switchAllowOnlyWhitelited();

    await expect(
      msgSender
        .connect(deployer)
        .registerCrossFunctionCall(1, ethers.constants.AddressZero, _data, false)
    ).to.revertedWith("Contract not whitelisted");
  });

  it("Should fail to register cross chain function if chainID is not whitelisted", async () => {
    const _data = {
      destinationContract: ethers.constants.AddressZero,
      destinationData: ethers.constants.HashZero,
      fallbackContract: ethers.constants.AddressZero,
      fallbackData: ethers.constants.HashZero,
    };
    await expect(
      msgSender
        .connect(deployer)
        .registerCrossFunctionCall(1, ethers.constants.AddressZero, _data, false)
    ).to.revertedWith("Unknown network");
  });

  it("Should not be able to registerd a cross function call when blacklisted", async () => {
    const _data = {
      destinationContract: ethers.constants.AddressZero,
      destinationData: ethers.constants.HashZero,
      fallbackContract: ethers.constants.AddressZero,
      fallbackData: ethers.constants.HashZero,
    };

    await msgSender.connect(deployer).addNetwork(1);
    await msgSender.updateBlacklistStatus(deployer.address, true);

    await expect(
      msgSender
        .connect(deployer)
        .registerCrossFunctionCall(1, ethers.constants.AddressZero, _data, false)
    ).to.revertedWith("Unauthorized");
  });

  it("Should  register a cross function call", async () => {
    const _data = {
      destinationContract: ethers.constants.AddressZero,
      destinationData: ethers.constants.HashZero,
      fallbackContract: ethers.constants.AddressZero,
      fallbackData: ethers.constants.HashZero,
    };

    await msgSender.connect(deployer).addNetwork(1);
    const tx = await msgSender
      .connect(deployer)
      .registerCrossFunctionCall(1, ethers.constants.AddressZero, _data, false);

    const receipt = await tx.wait();
    let id;

    for (let i = 0; i < receipt.events.length; i++) {
      let event = receipt.events[i];

      if (event.event === "ForwardCall") {
        id = event.args["id"];
      }
    }

    await expect(tx).to.emit(msgSender, "ForwardCall");

    expect(await msgSender.lastForwardedCall()).to.be.equals(id);
    expect(await msgSender.hasBeenForwarded(id)).to.be.equals(true);
  });

  it("Should  register a cross function call with token approval", async () => {
    const _data = {
      destinationContract: ethers.constants.AddressZero,
      destinationData: ethers.constants.HashZero,
      fallbackContract: ethers.constants.AddressZero,
      fallbackData: ethers.constants.HashZero,
    };

    await msgSender.connect(deployer).addNetwork(1);
    const tx = await msgSender
      .connect(deployer)
      .registerCrossFunctionCallWithTokenApproval(
        1,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        100,
        _data,
        false
      );

    const receipt = await tx.wait();
    let id;

    for (let i = 0; i < receipt.events.length; i++) {
      let event = receipt.events[i];

      if (event.event === "ForwardCallWithTokenApproval") {
        id = event.args["id"];
      }
    }

    await expect(tx).to.emit(msgSender, "ForwardCallWithTokenApproval");

    expect(await msgSender.lastForwardedCall()).to.be.equals(id);
    expect(await msgSender.hasBeenForwarded(id)).to.be.equals(true);
  });

  it("Should  register a cross token approval", async () => {
    await msgSender.connect(deployer).addNetwork(1);
    const tx = await msgSender
      .connect(deployer)
      .registerTokenApproval(
        1,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        100,
        ethers.constants.AddressZero,
        false
      );

    const receipt = await tx.wait();
    let id;

    for (let i = 0; i < receipt.events.length; i++) {
      let event = receipt.events[i];

      if (event.event === "ForwardTokenApproval") {
        id = event.args["id"];
      }
    }

    await expect(tx)
      .to.emit(msgSender, "ForwardTokenApproval")
      .withArgs(
        deployer.address,
        id,
        1,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        100,
        ethers.constants.AddressZero
      );

    expect(await msgSender.lastForwardedCall()).to.be.equals(id);
    expect(await msgSender.hasBeenForwarded(id)).to.be.equals(true);
  });

  it("Should register a cross safeETH", async () => {
    await msgSender.connect(deployer).addNetwork(1);
    const tx = await msgSender
      .connect(deployer)
      .registerSaveETH(1, ethers.constants.AddressZero, 100, ethers.constants.AddressZero, false);

    const receipt = await tx.wait();
    let id;

    for (let i = 0; i < receipt.events.length; i++) {
      let event = receipt.events[i];

      if (event.event === "ForwardSaveETH") {
        id = event.args["id"];
      }
    }

    await expect(tx)
      .to.emit(msgSender, "ForwardSaveETH")
      .withArgs(
        deployer.address,
        id,
        1,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        100
      );

    expect(await msgSender.lastForwardedCall()).to.be.equals(id);
    expect(await msgSender.hasBeenForwarded(id)).to.be.equals(true);
  });

  it("Should register a cross safeTokens", async () => {
    await msgSender.connect(deployer).addNetwork(1);
    const tx = await msgSender
      .connect(deployer)
      .registerSaveTokens(
        1,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        100,
        ethers.constants.AddressZero,
        false
      );

    const receipt = await tx.wait();
    let id;

    for (let i = 0; i < receipt.events.length; i++) {
      let event = receipt.events[i];

      if (event.event === "ForwardSaveTokens") {
        id = event.args["id"];
      }
    }

    await expect(tx)
      .to.emit(msgSender, "ForwardSaveTokens")
      .withArgs(
        deployer.address,
        id,
        1,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        100
      );

    expect(await msgSender.lastForwardedCall()).to.be.equals(id);
    expect(await msgSender.hasBeenForwarded(id)).to.be.equals(true);
  });

  it("Should register a cross safeNFT", async () => {
    await msgSender.connect(deployer).addNetwork(1);
    const tx = await msgSender
      .connect(deployer)
      .registerSaveNFT(
        1,
        ethers.constants.AddressZero,
        100,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        false
      );

    const receipt = await tx.wait();
    let id;

    for (let i = 0; i < receipt.events.length; i++) {
      let event = receipt.events[i];

      if (event.event === "ForwardSaveNFT") {
        id = event.args["id"];
      }
    }

    await expect(tx)
      .to.emit(msgSender, "ForwardSaveNFT")
      .withArgs(
        deployer.address,
        id,
        1,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        ethers.constants.AddressZero,
        100
      );

    expect(await msgSender.lastForwardedCall()).to.be.equals(id);
    expect(await msgSender.hasBeenForwarded(id)).to.be.equals(true);
  });

  it("Should fail if token has no balance", async () => {
    await expect(
      msgSender.connect(deployer).saveAirdroppedFunds(erc20_token2.address, receiver.address)
    ).to.revertedWith("No balance");
  });

  it("Should save airdropped funds", async () => {
    await expect(
      msgSender.connect(deployer).saveAirdroppedFunds(erc20_token.address, receiver.address)
    )
      .to.emit(msgSender, "FundsSaved")
      .withArgs(deployer.address, receiver.address, erc20_token.address, amount);

    const receivers_balance_after_tx = await erc20_token.balanceOf(receiver.address);
    expect(amount).to.be.equals(receivers_balance_after_tx);
  });

  it("Should fail to remove network", async () => {
    await expect(msgSender.connect(deployer).removeNetwork(1)).to.revertedWith("Not whitelisted");
    await msgSender.connect(deployer).addNetwork(1);
    await expect(msgSender.connect(receiver).removeNetwork(1)).to.revertedWith(
      "Ownable: caller is not the owner"
    );
  });

  it("Should remove network", async () => {
    await msgSender.connect(deployer).addNetwork(1);
    expect(await msgSender.whitelistedNetworks(1)).to.be.equals(true);

    await msgSender.connect(deployer).removeNetwork(1);

    expect(await msgSender.whitelistedNetworks(1)).to.be.equals(false);
    expect(await msgSender.pausedNetwork(1)).to.be.equals(false);
  });

  it("Should fail to pause network", async () => {
    await expect(msgSender.connect(receiver).pauseNetwork(1)).to.revertedWith(
      "Ownable: caller is not the owner"
    );
    await expect(msgSender.connect(deployer).pauseNetwork(1)).to.revertedWith("Unknown network");
  });

  it("Should pause network", async () => {
    await msgSender.connect(deployer).addNetwork(1);
    expect(await msgSender.pausedNetwork(1)).to.be.equals(false);

    await msgSender.connect(deployer).pauseNetwork(1);
    expect(await msgSender.pausedNetwork(1)).to.be.equals(true);
  });

  it("Should fail to unpause network", async () => {
    await expect(msgSender.connect(receiver).unpauseNetwork(1)).to.revertedWith(
      "Ownable: caller is not the owner"
    );
    await expect(msgSender.connect(deployer).unpauseNetwork(1)).to.revertedWith(
      "Network not paused"
    );
  });

  it("Should unpause network", async () => {
    await msgSender.connect(deployer).addNetwork(1);
    await msgSender.connect(deployer).pauseNetwork(1);

    expect(await msgSender.pausedNetwork(1)).to.be.equals(true);
    await msgSender.connect(deployer).unpauseNetwork(1);

    expect(await msgSender.pausedNetwork(1)).to.be.equals(false);
  });

  it("Should send forwardCallWithTokenApproval transaction", async () => {});

  it("Should send forwardCall transaction", async () => {
    await msgSender.connect(deployer).addNetwork(1);
    await msgSender.connect(deployer).pauseNetwork(1);

    expect(await msgSender.pausedNetwork(1)).to.be.equals(true);
    await msgSender.connect(deployer).unpauseNetwork(1);

    expect(await msgSender.pausedNetwork(1)).to.be.equals(false);
  });

  it("Should send registerTokenApproval transaction", async () => {});
});
