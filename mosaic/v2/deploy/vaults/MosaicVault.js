const { ethers, artifacts } = require("hardhat");
const { deployUpgradable, deployNonUpgradable } = require("../utils");

module.exports = async ({ getNamedAccounts, deployments }) => {
  const { deployer } = await getNamedAccounts();
  const owner = await ethers.getSigner(deployer);
  let tx;

  const mosaicHoldingDeployment = await deployments.get("MosaicHolding");
  const mosaicHolding = await ethers.getContractAt(
    "MosaicHolding",
    mosaicHoldingDeployment.address
  );

  const mosaicVaultConfigDeployment = await deployments.get("MosaicVaultConfig");
  const mosaicVaultConfig = await ethers.getContractAt(
    "MosaicVaultConfig",
    mosaicVaultConfigDeployment.address
  );
  const mosaicVault = await deployUpgradable("MosaicVault", [mosaicVaultConfig.address]);

  console.log("Setting the vault on mosaic vault config.....");
  const setVaultTx = await mosaicVaultConfig.connect(owner).setVault(mosaicVault.address);
  await setVaultTx.wait();
  console.log("✅ done");

  // set the vault on Mosaic holding
  console.log("Setting MOSAIC_VAULT on Mosaic holding.....");
  tx = await mosaicHolding
    .connect(owner)
    .grantRole(ethers.utils.id("MOSAIC_VAULT"), mosaicVault.address);
  await tx.wait();
  console.log("✅ done");

  console.log("Deployin a sample token.....");
  const sampleTokenDeployment = await deployNonUpgradable("SampleTokenERC20", [
    "Test",
    "T",
    ethers.utils.parseEther("100000"),
  ]);
  console.log("✅ done");

  // deploy the token factory
  const tokenFactoryDeployment = await deployNonUpgradable("TokenFactory", [
    mosaicVault.address,
    mosaicVaultConfig.address,
    sampleTokenDeployment.address,
  ]);

  // set the token factory on vault
  console.log("Setting address of the TokenFactory on MosaicVault....");
  tx = await mosaicVaultConfig
    .connect(owner)
    .setTokenFactoryAddress(tokenFactoryDeployment.address);
  await tx.wait();
  console.log("✅ done");
};

module.exports.tags = ["MosaicVault_deploy"]; // later can use for selective deployment
module.exports.dependencies = ["MosaicVaultConfig_deploy", "MosaicHolding_deploy"];
