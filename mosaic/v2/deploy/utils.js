const { deployments, getNamedAccounts, run, ethers, getChainId, artifacts } = require("hardhat");
const { chainIdToName } = require("../scripts/utils");
const readline = require("readline");

async function deployUpgradable(
  contractName,
  initializeFunctionArgs = [],
  artifactName = contractName,
  librariesName,
  verify = true,
  log = true,
  skipIfAlreadyDeployed = true
) {
  const { deployer, user1 } = await getNamedAccounts();
  const chainId = await getChainId();

  console.log("Starting deployment to", chainIdToName(chainId), "network");
  console.log("Deployer account:", deployer);
  const libraries = await getLibraries(librariesName);
  const deployment = await deployments.deploy(artifactName, {
    from: deployer,
    skipIfAlreadyDeployed, /// TODO this does not work. Need to test again with another version of hardhat-deploy
    contract: contractName,
    libraries,
    proxy: {
      owner: user1,
      proxyContract: "OpenZeppelinTransparentProxy",
      execute: { init: { methodName: "initialize", args: initializeFunctionArgs } },
    },
    waitConfirmations: +process.env.CONFIRMATIONS_NUMBER,
    log,
    autoMine: true,
  });

  const { implementation } = deployment;

  // causing errors - needs to skip if already initialized
  try {
    const implementationDeployment = await ethers.getContractAt(contractName, implementation);
    const tx = await implementationDeployment.initialize(...initializeFunctionArgs);
    await tx.wait();
    console.log("implementation initialize done");
  } catch (e) {
    console.log("implementation already initialized");
  }

  console.log(contractName, "proxy address:", deployment.address);
  console.log(contractName, "implementation address:", implementation);

  await verifyContract(implementation, artifactName, libraries);

  return ethers.getContractAt(contractName, deployment.address);
}

async function deployNonUpgradable(
  contractName,
  constructorArgs,
  artifactName = contractName,
  librariesName,
  verify = true,
  log = true,
  skipIfAlreadyDeployed = true
) {
  const { deployer } = await getNamedAccounts();
  const chainId = await getChainId();

  console.log("Starting deployment to", chainIdToName(chainId), "network");
  console.log("Deployer account:", deployer);

  const libraries = await getLibraries(librariesName);
  const deployment = await deployments.deploy(artifactName, {
    from: deployer,
    skipIfAlreadyDeployed,
    contract: contractName,
    libraries,
    args: constructorArgs,
    waitConfirmations: +process.env.CONFIRMATIONS_NUMBER,
    log,
  });

  console.log(`${contractName} deployed at address: ${deployment.address}`);

  await verifyContract(deployment.address, artifactName, libraries, constructorArgs);

  return ethers.getContractAt(contractName, deployment.address);
}

async function verifyContract(address, artifactName, libraries, constructorArguments) {
  const ans = await askIfVerify();
  if (["y", "Y", "yes", "YES"].includes(ans) == false) {
    console.log("Not verifying the contract");
    return;
  }

  const artifact = await artifacts.readArtifact(artifactName);
  const fullyQualifiedName = `${artifact.sourceName}:${artifact.contractName}`;
  console.log("Contract verification start");
  try {
    await run("verify:verify", {
      contract: fullyQualifiedName,
      address,
      constructorArguments,
      libraries,
    });
    console.log(`Contract ${artifactName} successfully verified`);
  } catch (error) {
    console.warn("\n !!! Contract not verified");
    console.error(`Error: ${error.message}\n`);
  }
}

async function getLibraries(libraries) {
  if (!libraries || libraries.length === 0) {
    return;
  }
  const deployedLibraries = {};
  for (let i = 0; i < libraries.length; i++) {
    const libraryName = libraries[i];
    const libraryDeployment = await deployments.get(libraryName);
    deployedLibraries[libraryName] = libraryDeployment.address;
  }
  return deployedLibraries;
}

function askIfVerify() {
  // https://nodejs.org/en/knowledge/command-line/how-to-prompt-for-command-line-input/
  const query = "Do you want to verify the contract? (y/N) ";
  const rl = readline.createInterface({
    input: process.stdin,
    output: process.stdout,
  });

  return new Promise((resolve) =>
    rl.question(query, (ans) => {
      rl.close();
      resolve(ans);
    })
  );
}

module.exports = {
  deployUpgradable,
  deployNonUpgradable,
};
