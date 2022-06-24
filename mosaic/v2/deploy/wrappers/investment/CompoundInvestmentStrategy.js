const { deployUpgradable } = require("../../utils");

module.exports = async ({ getNamedAccounts, deployments, ethers }) => {
  const { deployer } = await getNamedAccounts();

  const mosaicHoldingDeployment = await deployments.get("MosaicHolding");

  const compoundInvestmentStrategy = await deployUpgradable("CompoundInvestmentStrategy", [
    deployer,
    mosaicHoldingDeployment.address,
    process.env.COMPTROLLER_ADDRESS,
    process.env.COMP_ADDRESS,
  ]);

  const mosaicHolding = await ethers.getContractAt(
    "MosaicHolding",
    mosaicHoldingDeployment.address
  );
  await mosaicHolding.addInvestmentStrategy(compoundInvestmentStrategy.address);

  console.log("Compound Investment Strategy Added");
};

module.exports.tags = ["CompoundInvestmentStrategy"];
module.exports.dependencies = ["MosaicHolding_deploy"];
