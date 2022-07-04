const chai = require("chai");
const { solidity, deployMockContract } = require("ethereum-waffle");
const { ethers, upgrades } = require("hardhat");

chai.use(solidity);

expect = chai.expect;

async function deployProxy(contractName, initializeFunctionArgs = []) {
  const contractFactory = await ethers.getContractFactory(contractName);
  return upgrades.deployProxy(contractFactory, initializeFunctionArgs);
}

module.exports = { expect, deployMockContract, deployProxy };
