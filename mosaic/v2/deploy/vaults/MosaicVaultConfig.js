const { deployUpgradable } = require("../utils");

module.exports = async ({ getNamedAccounts }) => {
  const mosaicHoldingProxy = await deployments.getOrNull("MosaicHolding");
  if (!mosaicHoldingProxy) {
    throw "Deploy the MosaicHolding before";
  }

  await deployUpgradable("MosaicVaultConfig", [mosaicHoldingProxy.address]);

  console.log("MosaicVault Config deployed");
};

module.exports.tags = ["MosaicVaultConfig_deploy"];
module.exports.dependencies = ["MosaicHolding_deploy"];
