const { network } = require("hardhat");

async function fork_network(blockNumber = 13036180) {
  /// Use mainnet fork as provider
  return network.provider.request({
    method: "hardhat_reset",
    params: [
      {
        forking: {
          jsonRpcUrl: process.env.ALCHEMY_API,
          blockNumber: blockNumber,
        },
      },
    ],
  });
}

async function fork_reset() {
  return network.provider.request({
    method: "hardhat_reset",
    params: [],
  });
}

async function mine_blocks(numberOfBlocks) {
  for (let i = 0; i < numberOfBlocks; i++) {
    await network.provider.send("evm_mine");
  }
}

async function increase_block_timestamp(time) {
  return network.provider.send("evm_increaseTime", [time]);
}

module.exports = {
  fork_network,
  fork_reset,
  mine_blocks,
  increase_block_timestamp,
};
