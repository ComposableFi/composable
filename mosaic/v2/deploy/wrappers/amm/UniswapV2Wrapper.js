const { chainIdToName } = require("../../../scripts/utils");
const { deployUpgradable } = require("../../utils");

const uniswapRouterAddress = process.env.UNISWAP_ROUTER_02_ADDRESS;

module.exports = async ({ getChainId }) => {
  const chainId = await getChainId();
  const networkName = chainIdToName(chainId);

  if (uniswapRouterAddress === undefined) {
    throw `Please provide uniswapRouterAddress in the ${networkName.toLowerCase()}.env file`;
  }
  await deployUpgradable("UniswapV2Wrapper", [uniswapRouterAddress]);
};

module.exports.tags = ["UniswapV2Wrapper_deploy"];
