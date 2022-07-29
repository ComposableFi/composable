require("@openzeppelin/hardhat-upgrades");
require("@nomiclabs/hardhat-ethers");
require("@nomiclabs/hardhat-etherscan");
require("hardhat-deploy");
require("hardhat-deploy-ethers");
require("hardhat-contract-sizer");
require("hardhat-dependency-compiler");
require("hardhat-change-network");

require("./scripts/tasks");

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

module.exports = {
  defaultNetwork: "hardhat",

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
      accounts: [PRIVATE_KEY_1],
    },
  },
  etherscan: {
    apiKey: process.env.BLOCK_EXPLORER_API_KEY,
  },
  mocha: {
    timeout: 1000000,
    bail: true,
  },
};
