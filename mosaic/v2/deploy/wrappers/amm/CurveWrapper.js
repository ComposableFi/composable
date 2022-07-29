const { deployUpgradable } = require("../../utils");
const { addSupportedAmm } = require("../../protocols/amm-utils");

module.exports = async () => {
  const curveWrapper = await deployUpgradable("CurveWrapper", [
    process.env.CURVE_ADDRESS_PROVIDER_ADDRESS,
  ]);

  await addSupportedAmm(curveWrapper.address);

  console.log("curve Wrapper AMM Added");
};

module.exports.tags = ["CurveWrapper_deploy"];
module.exports.dependencies = ["MosaicVaultConfig_deploy"];
