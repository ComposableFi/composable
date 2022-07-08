const { deployUpgradable } = require("../utils");

module.exports = async ({ ethers, deployments, getNamedAccounts }) => {
  const { deployer } = await getNamedAccounts();
  const relayer = await ethers.getSigner(deployer);
  await deployUpgradable("MsgReceiverFactory", [await relayer.getAddress()]);
};

module.exports.tags = ["MsgReceiverFactory_Deploy"];
