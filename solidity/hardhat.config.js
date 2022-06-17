require("@openzeppelin/hardhat-upgrades");
require("@nomiclabs/hardhat-ethers");
require("@nomiclabs/hardhat-etherscan");
require("hardhat-deploy");
require("hardhat-deploy-ethers");
require("hardhat-contract-sizer");
require("hardhat-dependency-compiler");
require("hardhat-change-network");
// require("hardhat-gas-reporter");

require("./scripts/tasks");
require("./scripts/setup-network");

const dotenv = require("dotenv");
dotenv.config({ path: "./env/.env" });
const NODE_ENV = process.env.NODE_ENV;
if (!NODE_ENV || NODE_ENV === "") {
  throw `Please specify witch environment file you want to use\n \
    E.g: NODE_ENV={environmentFileHere} yarn hardhat ${process.argv
      .slice(2, process.argv.length)
      .join(" ")}`;
}
dotenv.config({ path: `./env/${process.env.NODE_ENV}.env` });

const INFURA_API_KEY = process.env.INFURA_API_KEY;

const PRIVATE_KEY_1 = process.env.PRIVATE_KEY_1;
const PUBLIC_KEY_1 = process.env.PUBLIC_KEY_1;

const PRIVATE_KEY_2 = process.env.PRIVATE_KEY_2;
const PUBLIC_KEY_2 = process.env.PUBLIC_KEY_2;

const PRIVATE_KEY_3 = process.env.PRIVATE_KEY_3;
const PUBLIC_KEY_3 = process.env.PUBLIC_KEY_3;

module.exports = {
  defaultNetwork: "hardhat",
  namedAccounts: {
    deployer: {
      default: 0, // first account in signers list
      mainnet: PUBLIC_KEY_1,
      kovan: PUBLIC_KEY_1,
      arbitrum_testnet: PUBLIC_KEY_1,
      transfer_optimism: PUBLIC_KEY_1,
      optimism_l1_local: PUBLIC_KEY_1,
    },
    user1: {
      default: 1, // second account in signers list
      mainnet: PUBLIC_KEY_2,
      kovan: PUBLIC_KEY_2,
      arbitrum_testnet: PUBLIC_KEY_2,
      transfer_optimism: PUBLIC_KEY_2,
      optimism_l1_local: PUBLIC_KEY_2,
    },
    user2: {
      default: 2, // 3rd account in signers list
      mainnet: PUBLIC_KEY_3,
      kovan: PUBLIC_KEY_3,
      arbitrum_l2_public: PUBLIC_KEY_3,
      transfer_optimism: PUBLIC_KEY_3,
      optimism_l1_local: PUBLIC_KEY_3,
    },
  },
  solidity: {
    compilers: [
      {
        version: "0.8.4",
        settings: {
          optimizer: {
            enabled: true,
            runs: 200,
          },
        },
      },
    ],
  },
  networks: {
    hardhat: {
      saveDeployments: false,
      allowUnlimitedContractSize: true,
    },
    localhost: {
      url: "http://127.0.0.1:8545/",
    },
    mainnet: {
      url: `https://mainnet.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    kovan: {
      url: `https://kovan.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    rinkeby: {
      url: `https://rinkeby.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    evmos: {
      url: `https://ethereum.rpc.evmos.org`,
      network_id: 9000,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    ropsten: {
      url: `https://ropsten.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    goerli: {
      url: `https://goerli.infura.io/v3/${INFURA_API_KEY}`,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    arbitrum_testnet: {
      url: "https://rinkeby.arbitrum.io/rpc",
      network_id: 421611,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
      companionNetworks: {
        l1: "rinkeby",
      },
    },
    arbitrum_mainnet: {
      url: "https://arb1.arbitrum.io/rpc",
      network_id: 42161,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
      companionNetworks: {
        l1: "mainnet",
      },
    },
    optimism_l2_public: {
      url: "https://kovan.optimism.io",
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
      timeout: 120000,
      companionNetworks: {
        l1: "kovan",
      },
    },
    mumbai: {
      url: "https://matic-mumbai.chainstacklabs.com",
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
      gasPrice: 8000000000,
      companionNetworks: {
        l1: "kovan",
      },
    },
    matic: {
      url: "https://rpc-mainnet.maticvigil.com",
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
      companionNetworks: {
        l1: "mainnet",
      },
    },
    sokol: {
      url: "https://sokol.poa.network",
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
      companionNetworks: {
        l1: "kovan",
      },
    },
    xdai: {
      url: "https://rpc.xdaichain.com/",
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
      companionNetworks: {
        l1: "mainnet",
      },
    },
    avalanche_cchain: {
      //mainnet
      url: "https://api.avax.network/ext/bc/C/rpc",
      network_id: 43114,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    avalanche_fuji: {
      //testnet
      url: "https://api.avax-test.network/ext/bc/C/rpc",
      network_id: 43113,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    moonriver_alpha: {
      //testnet
      url: "https://rpc.testnet.moonbeam.network",
      network_id: 1287,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    moonriver_mainnet: {
      url: "https://rpc.moonriver.moonbeam.network",
      network_id: 1285,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
    boba_testnet: {
      url: "https://rinkeby.boba.network",
      network_id: 28,
      accounts: [PRIVATE_KEY_1, PRIVATE_KEY_2, PRIVATE_KEY_3],
    },
  },
  etherscan: {
    apiKey: process.env.BLOCK_EXPLORER_API_KEY,
  },
  mocha: {
    timeout: 1000000,
    bail: true,
  },
  // gasReporter: {
  //   gasPrice: 80,
  //   coinmarketcap: "",
  // },
};
