const { deployUpgradable } = require("../../utils");

module.exports = async ({ getNamedAccounts, deployments, ethers }) => {
  const { deployer } = await getNamedAccounts();

  const mosaicHoldingProxy = await deployments.get("MosaicHolding");

  const aaveInvestmentStrategy = await deployUpgradable("AaveInvestmentStrategy", [
    deployer,
    mosaicHoldingProxy.address,
    process.env.LENDING_POOL_ADDRESS_PROVIDER,
    process.env.INCENTIVES_CONTROLLER,
  ]);

  const mosaicHolding = await ethers.getContractAt("MosaicHolding", mosaicHoldingProxy.address);
  await mosaicHolding.addInvestmentStrategy(aaveInvestmentStrategy.address);

  console.log("Aave Investment Strategy Added");
};

module.exports.tags = ["AaveInvestmentStrategy_deploy"];
module.exports.dependencies = ["MosaicHolding_deploy"];
