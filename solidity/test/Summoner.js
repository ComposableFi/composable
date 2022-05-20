const { ethers, upgrades, network } = require("hardhat");
const { expect } = require("./test_env.js");

const randomAddress = ethers.utils.getAddress("<add the address>");
const randomAddress3 = ethers.utils.getAddress("<add the address>");
const tokenSupply = ethers.utils.parseEther("100");
const feeAmountToken = ethers.BigNumber.from(10).pow(17);
const feeAmountNative = ethers.BigNumber.from(10).pow(6);

let owner, nftHolder;
let summoner, sampleNft, eRC20Contract, config, mosaicNft;

describe("Summoner - Test network", async () => {
  before(async () => {
    [owner, nftHolder] = await ethers.getSigners();
    // set 10 ether as the owner balance
    await network.provider.send("hardhat_setBalance", [
      owner.address,
      ethers.utils.parseEther("10").toHexString(),
    ]);
    await network.provider.send("hardhat_setBalance", [
      nftHolder.address,
      ethers.BigNumber.from(10).pow(18).mul(2).toHexString(),
    ]);
    await mockContracts();
  });

  after(async () => {
    await network.provider.request({
      method: "hardhat_reset",
      params: [],
    });
  });

  describe("Test transferERC721ToLayer", async () => {
    it("successful transfer summon to destination network", async () => {
      await sampleNft.connect(nftHolder).approve(summoner.address, 1);
      const tx = await summoner
        .connect(nftHolder)
        .transferERC721ToLayer(
          sampleNft.address,
          1,
          randomAddress,
          137,
          0,
          ethers.constants.AddressZero,
          { value: feeAmountNative }
        );
      const isRelease = await getEventParam(tx, "TransferInitiated", "isRelease");
      expect(isRelease).to.eq(false);
    });

    it("successful transfer release to original network", async () => {
      // transfer NFT to summoner
      // https://ethereum.stackexchange.com/a/87204
      await sampleNft.connect(nftHolder).approve(owner.address, 6);
      await sampleNft["safeTransferFrom(address,address,uint256)"](
        nftHolder.address,
        summoner.address,
        6
      );

      // summon NFT
      await summoner
        .connect(owner)
        .summonNFT(
          "randomUri6",
          nftHolder.address,
          sampleNft.address,
          137,
          6,
          ethers.utils.formatBytes32String("random6")
        );

      // if nft meta data has been set properly
      const nftMetadata = await mosaicNft.getOriginalNftInfo(1);
      expect(nftMetadata[0]).eq(sampleNft.address);
      expect(nftMetadata[1]).eq(137);
      expect(nftMetadata[2]).eq(6);

      await mosaicNft.connect(nftHolder).approve(summoner.address, 1);
      // transfer back to original layer
      let tx = await summoner
        .connect(nftHolder)
        .transferERC721ToLayer(
          mosaicNft.address,
          1,
          randomAddress,
          137,
          0,
          ethers.constants.AddressZero,
          { value: feeAmountNative }
        );
      const isRelease = await getEventParam(tx, "TransferInitiated", "isRelease");
      expect(isRelease).to.eq(true);
    });

    it("successful transfer summon to source network", async () => {
      // transfer NFT to summoner
      // https://ethereum.stackexchange.com/a/87204
      await sampleNft.connect(nftHolder).approve(owner.address, 8);
      await sampleNft["safeTransferFrom(address,address,uint256)"](
        nftHolder.address,
        summoner.address,
        8
      );

      const originalNftAddress = randomAddress3;

      // summon NFT
      await summoner
        .connect(owner)
        .summonNFT(
          "randomUri8",
          nftHolder.address,
          originalNftAddress,
          81,
          9,
          ethers.utils.formatBytes32String("random8")
        );

      const nftInfo = await mosaicNft.getOriginalNftInfo(2);
      await expect(nftInfo[0]).to.eql(originalNftAddress);
      await expect(nftInfo[1].toNumber()).to.eql(81);
      await expect(nftInfo[2].toNumber()).to.eql(9);

      await mosaicNft.connect(nftHolder).approve(summoner.address, 2);

      // transfer mosaic nft to another layer
      const tx = await summoner
        .connect(nftHolder)
        .transferERC721ToLayer(
          mosaicNft.address,
          2,
          randomAddress,
          137,
          0,
          ethers.constants.AddressZero,
          { value: feeAmountNative }
        );
      const param0 = await getEventParam(tx, "TransferInitiated", "originalNftAddress");
      await expect(param0).to.eql(originalNftAddress);
      const param1 = await getEventParam(tx, "TransferInitiated", "originalNetworkID");
      await expect(param1.toNumber()).to.eql(81);
      const param2 = await getEventParam(tx, "TransferInitiated", "originalNftId");
      await expect(param2.toNumber()).to.eql(9);
      const isRelease = await getEventParam(tx, "TransferInitiated", "isRelease");
      expect(isRelease).to.eq(false);

      let curOwner = await mosaicNft.ownerOf(2);
      expect(curOwner).eq(summoner.address);

      await summoner
        .connect(owner)
        .summonNFT(
          "randomUri8",
          nftHolder.address,
          originalNftAddress,
          81,
          9,
          ethers.utils.formatBytes32String("random800")
        );

      curOwner = await mosaicNft.ownerOf(2);
      expect(curOwner).eq(nftHolder.address);
    });

    it("fail: Not authorized to lock this NFT", async () => {
      await expect(
        summoner
          .connect(nftHolder)
          .transferERC721ToLayer(
            sampleNft.address,
            4,
            randomAddress,
            137,
            0,
            ethers.constants.AddressZero,
            { value: feeAmountNative }
          )
      ).to.be.revertedWith("ERC721: transfer caller is not owner nor approved");
    });

    it("fail: transfer not yet possible", async () => {
      await sampleNft.connect(nftHolder).approve(summoner.address, 2);
      await sampleNft.connect(nftHolder).approve(summoner.address, 3);
      await summoner
        .connect(nftHolder)
        .transferERC721ToLayer(
          sampleNft.address,
          2,
          randomAddress,
          137,
          0,
          ethers.constants.AddressZero,
          { value: feeAmountNative }
        );
      await config.connect(owner).setTransferLockupTime(15);
      await expect(
        summoner
          .connect(nftHolder)
          .transferERC721ToLayer(
            sampleNft.address,
            3,
            randomAddress,
            137,
            0,
            ethers.constants.AddressZero,
            { value: feeAmountNative }
          )
      ).to.be.revertedWith("TIMESTAMP");
      await config.connect(owner).setTransferLockupTime(0);
    });
  });

  describe("Test summonNFT", async () => {
    it("successful summonNFT", async () => {
      await expect(
        summoner
          .connect(owner)
          .summonNFT(
            "randomUri1",
            nftHolder.address,
            sampleNft.address,
            1,
            10,
            ethers.utils.formatBytes32String("random1")
          )
      ).to.emit(summoner, "SummonCompleted");

      const uri = await mosaicNft.tokenURI(3);
      expect(uri).to.eql("randomUri1");
    });

    it("fail: Already summoned", async () => {
      await expect(
        summoner
          .connect(owner)
          .summonNFT(
            "randomUri1",
            nftHolder.address,
            sampleNft.address,
            137,
            10,
            ethers.utils.formatBytes32String("random1")
          )
      ).to.be.revertedWith("SUMMONED");
    });

    it("successful summonNFT using a pre minted NFT", async () => {
      await summoner.connect(owner).preMintNFT(3);
      const prev = await summoner.connect(owner).getPreMintedCount();
      expect(prev.toNumber()).to.eql(3);

      await expect(
        summoner
          .connect(owner)
          .summonNFT(
            "randomUri12",
            nftHolder.address,
            sampleNft.address,
            1,
            12,
            ethers.utils.formatBytes32String("random2")
          )
      ).to.emit(summoner, "SummonCompleted");

      // should use the preminted one
      const uri = await mosaicNft.tokenURI(6);
      expect(uri).to.eql("randomUri12");
      const nftInfo = await mosaicNft.getOriginalNftInfo(6);
      await expect(nftInfo[0]).to.eql(sampleNft.address);
      await expect(nftInfo[1].toNumber()).to.eql(1);
      await expect(nftInfo[2].toNumber()).to.eql(12);

      const mintedNFTid = await mosaicNft.getNftId(nftInfo[0], nftInfo[1], nftInfo[2]);
      expect(mintedNFTid.toNumber()).eq(6);

      const curOwner = await mosaicNft.ownerOf(6);
      expect(curOwner).eq(nftHolder.address);

      const cur = await summoner.connect(owner).getPreMintedCount();
      expect(cur.toNumber()).to.eql(2);
    });
  });

  describe("Test releaseSeal", async () => {
    it("successful releaseSeal", async () => {
      await expect(
        summoner
          .connect(owner)
          .releaseSeal(
            nftHolder.address,
            sampleNft.address,
            1,
            ethers.utils.formatBytes32String("random1"),
            false
          )
      ).to.emit(summoner, "SealReleased");
    });

    it("fail: Already released", async () => {
      await expect(
        summoner
          .connect(owner)
          .releaseSeal(
            nftHolder.address,
            sampleNft.address,
            1,
            ethers.utils.formatBytes32String("random1"),
            false
          )
      ).to.be.revertedWith("RELEASED");
    });

    it("fail: The NFT is not locked", async () => {
      await expect(
        summoner
          .connect(owner)
          .releaseSeal(
            nftHolder.address,
            sampleNft.address,
            3,
            ethers.utils.formatBytes32String("random3"),
            false
          )
      ).to.be.revertedWith("NOT LOCKED");
    });

    it("successful: re transfer after releasing NFT", async () => {
      await sampleNft.connect(nftHolder).approve(summoner.address, 1);
      await expect(
        summoner
          .connect(nftHolder)
          .transferERC721ToLayer(
            sampleNft.address,
            1,
            randomAddress,
            137,
            0,
            ethers.constants.AddressZero,
            { value: feeAmountNative }
          )
      ).to.emit(summoner, "TransferInitiated");
    });
  });

  describe("Testing fee operations", async () => {
    it("fail: not enough fees", async () => {
      await sampleNft.connect(nftHolder).approve(summoner.address, 3);
      await expect(
        summoner
          .connect(nftHolder)
          .transferERC721ToLayer(
            sampleNft.address,
            3,
            randomAddress,
            137,
            0,
            ethers.constants.AddressZero
          )
      ).to.be.revertedWith("FEE");
    });

    it("fail: token not accepted", async () => {
      await sampleNft.connect(nftHolder).approve(summoner.address, 3);
      await expect(
        summoner
          .connect(nftHolder)
          .transferERC721ToLayer(
            sampleNft.address,
            3,
            randomAddress,
            137,
            0,
            process.env.USDC_ADDRESS
          )
      ).to.be.revertedWith("FEE TOKEN");
    });

    it("successfully take fee in native token", async () => {
      await sampleNft.connect(nftHolder).approve(summoner.address, 3);
      await summoner
        .connect(nftHolder)
        .transferERC721ToLayer(
          sampleNft.address,
          3,
          randomAddress,
          137,
          0,
          ethers.constants.AddressZero,
          { value: feeAmountNative }
        );
      const balance = await ethers.provider.getBalance(summoner.address);
      expect(balance.gt(0)).to.be.true;
    });

    it("successfully withdraw native fee to address", async () => {
      const prevBalanceHolder = await ethers.provider.getBalance(nftHolder.address);
      const prevBalanceContract = await ethers.provider.getBalance(summoner.address);
      await summoner
        .connect(owner)
        .withdrawFees(ethers.constants.AddressZero, nftHolder.address, prevBalanceContract);
      const curBalanceHolder = await ethers.provider.getBalance(nftHolder.address);
      expect(prevBalanceHolder.add(prevBalanceContract)).to.be.eq(curBalanceHolder);

      const curBalanceContract = await ethers.provider.getBalance(summoner.address);
      expect(curBalanceContract.eq(0)).to.be.true;
    });

    it("successfully take fee in token", async () => {
      eRC20Contract = await (await ethers.getContractFactory("SampleTokenERC20"))
        .connect(owner)
        .deploy("Token", "TKN", tokenSupply);
      await eRC20Contract.connect(owner).transfer(nftHolder.address, tokenSupply);
      await config.connect(owner).setFeeToken(137, eRC20Contract.address, feeAmountToken);
      await config.connect(owner).setFeeToken(1, eRC20Contract.address, feeAmountToken.mul(2));
      await eRC20Contract.connect(nftHolder).approve(summoner.address, feeAmountToken);
      await sampleNft.connect(nftHolder).approve(summoner.address, 5);
      await summoner
        .connect(nftHolder)
        .transferERC721ToLayer(sampleNft.address, 5, randomAddress, 137, 0, eRC20Contract.address);
      const b1 = await eRC20Contract.balanceOf(summoner.address);
      expect(b1).to.be.eq(feeAmountToken);
      const b2 = await eRC20Contract.balanceOf(nftHolder.address);
      expect(b2).to.be.eq(tokenSupply.sub(feeAmountToken));
    });

    it("successfully withdraw token fee to address", async () => {
      const b0 = await eRC20Contract.balanceOf(summoner.address);
      await summoner.connect(owner).withdrawFees(eRC20Contract.address, nftHolder.address, b0);
      const b1 = await eRC20Contract.balanceOf(nftHolder.address);
      expect(b1.gt(0)).to.be.true;

      const b2 = await eRC20Contract.balanceOf(summoner.address);
      expect(b2.eq(0)).to.be.true;
    });

    it("successfully refund fee", async () => {
      await sampleNft.connect(nftHolder).approve(summoner.address, 7);
      const tx = await summoner
        .connect(nftHolder)
        .transferERC721ToLayer(
          sampleNft.address,
          7,
          randomAddress,
          137,
          0,
          ethers.constants.AddressZero,
          { value: feeAmountNative }
        );
      const id = await getEventParam(tx, "TransferInitiated", "id");

      let balance = await ethers.provider.getBalance(summoner.address);
      expect(balance.eq(feeAmountNative)).to.be.true;

      // change the fee for the native token
      await config
        .connect(owner)
        .setFeeToken(137, ethers.constants.AddressZero, feeAmountNative.mul(3));

      const prevBalanceHolder = await ethers.provider.getBalance(nftHolder.address);
      await summoner.connect(owner).releaseSeal(nftHolder.address, sampleNft.address, 7, id, true);
      const curBalanceHolder = await ethers.provider.getBalance(nftHolder.address);
      expect(prevBalanceHolder.add(feeAmountNative)).to.be.eq(curBalanceHolder);

      balance = await eRC20Contract.balanceOf(summoner.address);
      expect(balance.eq(0)).to.be.true;
    });
  });

  describe("Test MosaicNFT transfers", async () => {
    it("failed transfer", async () => {
      const nft = await (await ethers.getContractFactory("MosaicNFT"))
        .connect(owner)
        .deploy(owner.address);

      await nft.connect(owner).preMintNFT();

      await expect(
        nft["safeTransferFrom(address,address,uint256)"](owner.address, summoner.address, 1)
      ).to.revertedWith("METADATA NOT SET");
    });

    it("successful transfer", async () => {
      const nft = await (await ethers.getContractFactory("MosaicNFT"))
        .connect(owner)
        .deploy(owner.address);

      await nft.connect(owner).preMintNFT();

      await nft.connect(owner).setNFTMetadata(1, "hello world", ethers.constants.AddressZero, 1, 2);

      await expect(
        nft["safeTransferFrom(address,address,uint256)"](owner.address, summoner.address, 1)
      ).to.emit(nft, "Transfer");
    });
  });

  describe("Test cannot set MosaicNFT", async () => {
    it("failed set", async () => {
      await expect(summoner.connect(owner).setMosaicNft(sampleNft.address)).to.revertedWith(
        "ALREADY PRE-MINTED"
      );
    });
  });
});

async function mockContracts() {
  const SummonerConfig = await ethers.getContractFactory("SummonerConfig");
  config = await SummonerConfig.connect(owner).deploy([]);

  const Summoner = await ethers.getContractFactory("Summoner");
  summoner = await upgrades.deployProxy(Summoner, [config.address]);
  await summoner.deployed();

  await config.connect(owner).setTransferLockupTime(0);
  await config.connect(owner).setFeeToken(1, ethers.constants.AddressZero, feeAmountNative.mul(2));
  await config.connect(owner).setFeeToken(137, ethers.constants.AddressZero, feeAmountNative);

  sampleNft = await (await ethers.getContractFactory("SampleNFT"))
    .connect(owner)
    .deploy("SampleNFT", "NFT");

  mosaicNft = await (await ethers.getContractFactory("MosaicNFT"))
    .connect(owner)
    .deploy(summoner.address);

  await summoner.connect(owner).setMosaicNft(mosaicNft.address);

  for (let i = 0; i < 20; i++) {
    await sampleNft.connect(nftHolder).mintNft(nftHolder.address, `randomUri${i}`);
  }
}

async function getEventParam(tx, eventName, paramName) {
  const receipt = await tx.wait();
  let param;
  for (let i = 0; i < receipt.events.length; i++) {
    let event = receipt.events[i];
    if (event.event === eventName) {
      param = event.args[paramName];
    }
  }
  return param;
}
