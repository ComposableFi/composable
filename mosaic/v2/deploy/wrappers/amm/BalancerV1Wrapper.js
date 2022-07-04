const { deployUpgradable } = require("../../utils");
const { addSupportedAmm } = require("../../protocols/amm-utils");

module.exports = async () => {
  const balancerV1Wrapper = await deployUpgradable("BalancerV1Wrapper", [
    process.env.BALANCER_EXCHANGE_PROXY_ADDRESS,
  ]);

  await addSupportedAmm(balancerV1Wrapper.address);

  console.log("Balancer V1 Wrapper AMM Added");
};

module.exports.tags = ["BalancerV1Wrapper_deploy"];
module.exports.dependencies = ["MosaicVaultConfig_deploy"];
