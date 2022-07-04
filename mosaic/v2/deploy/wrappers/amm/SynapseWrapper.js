const { chainIdToName } = require("../../../scripts/utils");
const { deployUpgradable } = require("../../utils");

const synapseRouterAddress = process.env.SYNAPSE_ROUTER_ADDRESS;

module.exports = async ({ getChainId }) => {
  const chainId = await getChainId();
  const networkName = chainIdToName(chainId);

  if (synapseRouterAddress === undefined) {
    throw `Please provide SYNAPSE_ROUTER_ADDRESS in the ${networkName.toLowerCase()}.env file`;
  }
  await deployUpgradable("SynapseWrapper", [synapseRouterAddress]);
};

module.exports.tags = ["SynapseWrapper_deploy"];
