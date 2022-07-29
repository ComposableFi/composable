const { chainIdToName } = require("../../../scripts/utils");
const { deployUpgradable } = require("../../utils");

const sushiswapRouterAddress = process.env.SUSHISWAP_ROUTER_ADDRESS;

module.exports = async ({ getChainId }) => {
  const chainId = await getChainId();
  const networkName = chainIdToName(chainId);

  if (sushiswapRouterAddress === undefined) {
    throw `Please provide sushiswapRouterAddress in the ${networkName.toLowerCase()}.env file`;
  }
  await deployUpgradable("SushiswapWrapper", [sushiswapRouterAddress]);
};

module.exports.tags = ["SushiSwapWrapper_deploy"];
