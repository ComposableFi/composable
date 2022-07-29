const { chainIdToName } = require("../../scripts/utils");
const { deployNonUpgradable } = require("../utils");

const swapRouterAddress = process.env.PANGOLIN_ROUTER_ADDRESS;

module.exports = async ({ getChainId }) => {
  const chainId = await getChainId();
  const networkName = chainIdToName(chainId);

  if (swapRouterAddress === undefined) {
    throw `Please provide PANGOLIN_ROUTER_ADDRESS in the ${networkName.toLowerCase()}.env file`;
  }
  await deployNonUpgradable("MosaicNativeSwapperAVAX", [swapRouterAddress]);
};

module.exports.tags = ["MosaicNativeSwapperAVAXPangolin_deploy"];
