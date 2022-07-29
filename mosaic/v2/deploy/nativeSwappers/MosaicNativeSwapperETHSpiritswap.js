const { chainIdToName } = require("../../scripts/utils");
const { deployNonUpgradable } = require("../utils");

const swapRouterAddress = process.env.SPIRITSWAP_ROUTER_ADDRESS;

module.exports = async ({ getChainId }) => {
  const chainId = await getChainId();
  const networkName = chainIdToName(chainId);

  if (swapRouterAddress === undefined) {
    throw `Please provide SPIRITSWAP_ROUTER_ADDRESS in the ${networkName.toLowerCase()}.env file`;
  }
  await deployNonUpgradable("MosaicNativeSwapperETH", [swapRouterAddress]);
};

module.exports.tags = ["MosaicNativeSwapperETHSpiritswap_deploy"];
