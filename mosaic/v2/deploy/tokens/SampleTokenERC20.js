const { ethers } = require("hardhat");
const initialSupply = ethers.utils.parseEther("1000"); // 1000m
const { deployNonUpgradable } = require("../utils");

module.exports = async ({ getNamedAccounts }) => {
  const { deployer, user1, user2 } = await getNamedAccounts();

  const tokenContract = await deployNonUpgradable("SampleTokenERC20", [
    "Token2",
    "TKN2",
    initialSupply,
  ]);

  const contractInitialSupply = await tokenContract.totalSupply();
  console.log("Total supply:", contractInitialSupply.toString());

  const owner = await ethers.getSigner(deployer);

  const ownerBalance = await tokenContract.balanceOf(owner.address);

  if (ownerBalance.eq(initialSupply)) {
    // else, already deployed and transferred
    const transactionReceipt = await tokenContract
      .connect(owner)
      .transfer(user1, ethers.utils.parseEther("250"));
    const transactionReceipt2 = await tokenContract
      .connect(owner)
      .transfer(user2, ethers.utils.parseEther("250"));
    await transactionReceipt.wait();
    await transactionReceipt2.wait();
  }

  const newDeployerBalance = await tokenContract.balanceOf(deployer);
  const newUser1Balance = await tokenContract.balanceOf(user1);
  const newUser2Balance = await tokenContract.balanceOf(user2);

  console.log("Account:", deployer, "has token balance:", newDeployerBalance.toString());
  console.log("Account:", user1, "has token balance:", newUser1Balance.toString());
  console.log("Account:", user2, "has token balance:", newUser2Balance.toString());
};

module.exports.tags = ["SampleToken"]; // later can use for selective deployment
