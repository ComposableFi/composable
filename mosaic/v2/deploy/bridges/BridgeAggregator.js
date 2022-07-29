const { deployUpgradable } = require("../utils");

module.exports = async ({ deployments }) => {
  const mosaicHoldingContract = await deployments.get("MosaicHolding");

  await deployUpgradable("BridgeAggregator", [mosaicHoldingContract.address]);
};

module.exports.tags = ["BridgeAggregator"];
module.exports.dependencies = ["MosaicHolding_deploy"];
