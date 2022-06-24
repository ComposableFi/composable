const { deployUpgradable } = require("../utils");

module.exports = async ({ getNamedAccounts }) => {
  const { deployer } = await getNamedAccounts();
  await deployUpgradable("MosaicHolding", [deployer]);
};

module.exports.tags = ["MosaicHolding_deploy"];
