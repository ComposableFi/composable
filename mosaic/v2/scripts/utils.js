const chainsList = {
  HARDHAT: { chainId: 31337 },

  MOONRIVER_MAINNET: { chainId: 1285, l1ChainId: 1 },
  MOONRIVER_KOVAN: { chainId: 9999, l1ChainId: 42 }, //chainId not known
  AURORA_TESTNET: { chainId: 1313161555, l1ChainId: 42 }, //NEAR
  AURORA_MAINNET: { chainId: 1313161554, l1ChainId: 1 }, //NEAR

  BOBA_MAINNET: { chainId: 0, l1ChainId: 1 }, //not on mainnet yet
  BOBA_TESTNET: { chainId: 28, l1ChainId: 4 }, //rinkeby

  ARBITRUM_RINKEBY: { chainId: 421611, l1ChainId: 4 },
  ARBITRUM_MAINNET: { chainId: 42161, l1ChainId: 1 },

  OPTIMISM: { chainId: 10, l1ChainId: 1 },
  OPTIMISM_KOVAN: { chainId: 69, l1ChainId: 42 },

  MATIC: { chainId: 137, l1ChainId: 1 },
  MUMBAI: { chainId: 80001, l1ChainId: 5 },

  // ZkSync has the same id as the l1 chain
  // todo fix clash with l1 chain id - for now added 000
  ZKSYNC: { chainId: 1000, l1ChainId: 1 },
  ZKSYNC_RINKEBY: { chainId: 4000, l1ChainId: 4 },

  // https://docs.celo.org/getting-started/wallets/using-metamask-with-celo/manual-setup
  CELO: { chainId: 42220, l1ChainId: 1 },
  // Alfajores testnet
  CELO_RINKEBY: { chainId: 44787, l1ChainId: 4 },

  SOKOL: { chainId: 77, l1ChainId: 42 },
  XDAI: { chainId: 100, l1ChainId: 1 },

  EVMOS: { chainId: 9000, l1ChainId: 1 },

  RINKEBY: { chainId: 4 },
  GOERLI: { chainId: 5 },
  KOVAN: { chainId: 42 },
  MAINNET: { chainId: 1 },
};

function chainIdToName(chainId) {
  const networkName = Object.keys(chainsList).find(
    (key) => chainsList[key].chainId === parseInt(chainId)
  );
  if (networkName) {
    return networkName;
  }
  return "";
}

function chainNameToId(chainName) {
  const network = chainsList[chainName];
  if (network) {
    return network.chainId;
  }
  return 0;
}

async function getTestNetContractsAddresses() {
  const { body } = await got("https://static.matic.network/network/testnet/mumbai/index.json");
  const addressesJson = JSON.parse(body);
  const RootChainManager = addressesJson.Main.POSContracts.RootChainManagerProxy;
  const ERC20Predicate = addressesJson.Main.POSContracts.ERC20PredicateProxy;
  return { RootChainManager, ERC20Predicate };
}

async function getContractInstance(contractName) {
  const deployment = await deployments.getOrNull(contractName);
  if (!deployment) {
    throw `Deployment ${contractName} does not exist`;
  }
  return new ethers.Contract(deployment.address, deployment.abi, ethers.provider);
}

async function getGasPrice(network) {
  const provider = new ethers.providers.JsonRpcProvider(network);
  return provider.getGasPrice();
}

function isMainnet() {
  return process.env.NETWORK_NAME === "mainnet";
}

module.exports = {
  getTestNetContractsAddresses,
  getContractInstance,
  chainIdToName,
  chainNameToId,
  isMainnet,
  chainsList,
  getGasPrice,
};
