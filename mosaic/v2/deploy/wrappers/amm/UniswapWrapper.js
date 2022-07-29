const { chainIdToName } = require("../../../scripts/utils");
const { deployUpgradable } = require("../../utils");

const swapRouterAddress = process.env.UNISWAP_ADDRESS;
const quoterAddress = process.env.UNISWAP_QUOTER_ADDRESS;

module.exports = async ({ getChainId }) => {
  const chainId = await getChainId();
  const networkName = chainIdToName(chainId);

  if (swapRouterAddress === undefined || quoterAddress === undefined) {
    throw `Please provide swapRouterAddress and quoterAddress in the ${networkName.toLowerCase()}.env file`;
  }

  await deployUpgradable("UniswapWrapper", [swapRouterAddress, quoterAddress]);
};

module.exports.tags = ["UniswapV3Wrapper_deploy"];
