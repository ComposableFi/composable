const { deployUpgradable } = require("../../utils");

module.exports = async ({ getNamedAccounts, deployments, ethers }) => {
  const { deployer } = await getNamedAccounts();

  const mosaicHoldingProxy = await deployments.get("MosaicHolding_Proxy");
  const sushiswapLiquidityProvider = await deployUpgradable("SushiswapLiquidityProvider", [
    deployer,
    mosaicHoldingProxy.address,
    process.env.SUSHISWAP_ROUTER_ADDRESS,
    process.env.SUSHISWAP_V2_FACTORY,
  ]);

  const mosaicHolding = await ethers.getContractAt("MosaicHolding", mosaicHoldingProxy.address);
  await mosaicHolding.addInvestmentStrategy(sushiswapLiquidityProvider.address);
};

module.exports.tags = ["SushiswapLiquidityProvider_deploy"];
module.exports.dependencies = ["MosaicVault_deploy", "MosaicHolding_deploy"];
