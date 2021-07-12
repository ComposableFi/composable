const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const chalk = require("chalk");

const keyring = new Keyring({ type: "sr25519" });

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

const sleep = (ms) =>
  new Promise((resolve) => {
    setTimeout(() => resolve(), ms);
  });

const requestPrice = async (api) => {
  const asset = 0;
  const tx = await api.tx.oracle.requestPrice(asset);
  await tx.signAndSend(keyring.addFromUri("//Alice"));
};

const main = async () => {
  const api = await connect("ws://172.28.1.5:9948");
  const rounds = 10;
  for (let i = 1; i <= rounds; i++) {
    console.log(chalk.bgBlack(chalk.red(`  ${i}  `)));
    await requestPrice(api);
    console.log(
      chalk.bgBlack(chalk.red("  Request Price  ")) +
        chalk.red(` Asset ID: ${0}`) +
        chalk.green(" Successful")
    );
    await sleep(6000);
  }
  console.log(chalk.bgBlack(chalk.red(`  DONE  `)));
  await api.disconnect();
  console.log(
    chalk.bgBlack(chalk.red(`  Disconnecting ws://172.28.1.5:9948  `))
  );
};

main();
