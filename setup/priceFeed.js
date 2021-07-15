const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const chalk = require("chalk");

// Connects to a provider endpoint and return returns the api provider instance with the unique chain types injected
const connect = async (endpoint) => {
  const wsProvider = new WsProvider(endpoint);
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {
      AssetTypes: "u128",
      AssetCount: "u64",
      AssetInfo: {
        threshold: "Percent",
        min_answers: "u64",
        max_answers: "u64",
      },
      AccountInfo: "AccountInfoWithDualRefCount",
      Settlements: "Vec<Settlement>",
      Withdraw: {
        stake: "Balance",
        unlock_block: "BlockNumber",
      },
      PrePrice: {
        price: "u64",
        block: "BlockNumber",
        who: "AccountId",
      },
      Price: {
        price: "u64",
        block: "BlockNumber",
      },
      Settlement: {
        who: "AccountId",
        truthful: "bool",
      },
    },
  });
  return api;
};

const main = async () => {
  const api = await connect("ws://172.28.1.1:9944");

  const unsub = api.query.oracle.prices(0, ({ price, block }) => {
    console.log(
      chalk.bgBlack(chalk.green(`Asset ID: 0 Price:`)) +
        chalk.green(` $${price / 100} `) +
        `at block #` +
        chalk.red(`${block}`)
    );
  });
};

main();
