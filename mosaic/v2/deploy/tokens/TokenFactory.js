const { deployNonUpgradable } = require("../utils");

module.exports = async ({ deployments, ethers, getNamedAccounts }) => {
  const { deployer } = await getNamedAccounts();
  const owner = await ethers.getSigner(deployer);

  const MosaicVaultConfigDeployment = await deployments.get("MosaicVaultConfig");

  if (!MosaicVaultConfigDeployment) {
    throw "Deploy the Mosaic Vault Config before";
  }

  const MosaicVaultConfig = await ethers.getContractAt(
    "MosaicVaultConfig",
    MosaicVaultConfigDeployment.address
  );
  const MosaicVaultDeployment = await deployments.get("MosaicVault");

  if (!MosaicVaultConfigDeployment) {
    throw "Deploy the Mosaic Vault before";
  }

  const sampleTokenDeployment = await deployNonUpgradable("SampleTokenERC20", ["Test", "T"]);

  const tokenFactoryDeployment = await deployNonUpgradable("TokenFactory", [
    MosaicVaultDeployment.address,
    MosaicVaultConfigDeployment.address,
    sampleTokenDeployment.address,
  ]);

  // set the token factory on vault
  console.log("Setting address of the TokenFactory on MosaicVault.....");
  const tx = await MosaicVaultConfig.connect(owner).setTokenFactoryAddress(
    tokenFactoryDeployment.address
  );
  await tx.wait();
  console.log("Address of the TokenFactory has been set.");
};

module.exports.tags = ["TokenFactory_Deploy"];
module.exports.dependencies = ["MosaicVault_deploy, MosaicVaultConfig"];
