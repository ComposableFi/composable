const { deployUpgradable, deployNonUpgradable } = require("../utils");

module.exports = async () => {
  const { deployer } = await getNamedAccounts();
  const owner = await ethers.getSigner(deployer);

  console.log("deploying summoner config..");
  const summonerConfig = await deployNonUpgradable("SummonerConfig", []);
  console.log("✅ done");
  console.log("deploying summoner..");
  const summoner = await deployUpgradable("Summoner", [summonerConfig.address]);
  console.log("✅ done");
  console.log("deploying mosaic nft..");
  const mosaicNFT = await deployNonUpgradable("MosaicNFT", [summoner.address]);
  console.log("✅ done");
  console.log("setting mosaic nft on summoner..");
  let tx = await summoner.connect(owner).setMosaicNft(mosaicNFT.address);
  console.log("tx id: " + tx.hash);
  await tx.wait();
  console.log("✅ done");
};

module.exports.tags = ["NFTSummoner"];
