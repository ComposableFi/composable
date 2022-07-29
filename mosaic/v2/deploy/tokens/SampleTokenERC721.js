const { deployNonUpgradable } = require("../utils");

module.exports = async () => {
  await deployNonUpgradable("SampleNFT", ["My cool mosaic NFT", "coolNFT"]);
};

module.exports.tags = ["SampleNft"]; // later can use for selective deployment
