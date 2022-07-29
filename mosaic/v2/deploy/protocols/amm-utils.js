const { ethers, deployments } = require("hardhat");

async function addSupportedAmm(newAmmAddress) {
  const mosaicVaultConfig = await deployments.getOrNull("MosaicVaultConfig");
  if (!mosaicVaultConfig) {
    throw "Deploy the MosaicVaultConfig before";
  }

  const mosaicVaultConfigContract = await ethers.getContractAt(
    "MosaicVaultConfig",
    mosaicVaultConfig.address
  );

  let nextAmmID,
    i = 0;
  while (!nextAmmID) {
    const ammAddress = await mosaicVaultConfigContract.getAMMAddress(i);
    if (ammAddress === ethers.constants.AddressZero) {
      nextAmmID = i;
    }
  }
  await mosaicVaultConfigContract.addSupportedAMM(nextAmmID, newAmmAddress);
}

module.exports = {
  addSupportedAmm,
};
