const { ApiPromise, WsProvider, Keyring } = require("@polkadot/api");
const chalk = require("chalk");

const keyring = new Keyring({ type: "sr25519" });

const nodes = [
  {
    name: "Alice",
    address: "5F6h9fXgSjPdmZDZQSsFyKUL1sPbuzTRn3TwbhGuSvPecB7d",
    publicKey:
      "0x8638cfb3584d75b38a0c10ed14fd5db134fa1764679b80c4f7a80a9c06895244",
    derivation:
      "van theme secret toddler rapid skirt pigeon hedgehog exhibit address guilt motor",
    endpoint: "ws://172.28.1.1:9944",
    initialized: false,
  },
  {
    name: "Bob",
    address: "5Gc2R35GvWAJ2uSHcLUceJudMJftbVp6Y788xzRpv8qy86sD",
    publicKey:
      "0xc8d4036beca0c173a594419cf664e3812128a1aed5fc06880a090b20607ec73e",
    derivation:
      "prevent mushroom elevator thumb stable unfair alcohol find leg fly couple deny",
    endpoint: "ws://172.28.1.2:9945",
    initialized: false,
  },
  {
    name: "Charlie",
    address: "5H1payfDS728ksrRi9D88RPQmyQFsZVdEFHYM4BKEiwfVJY9",
    publicKey:
      "0xdafa159e6e763f0183255ffbeab9de0ea617bc924273555cdd605c1362129177",
    derivation:
      "panda party toe child advance lawsuit meadow burden access below brown lift",
    endpoint: "ws://172.28.1.3:9946",
    initialized: false,
  },
  {
    name: "Dave",
    address: "5FkQP1FCvGVRX9QXu4oyxW9EjroC8eaTbJ8GLRbbQXv7AZfj",
    publicKey:
      "0xa2fbaf101f96828932535d5dcc59c84dbb15657ee53403a70fc451faadad183b",
    derivation:
      "physical glance describe mandate consider cricket detail excuse steak artwork broccoli diesel",
    endpoint: "ws://172.28.1.4:9947",
    initialized: false,
  },
  {
    name: "Eve",
    address: "5CXru9Vt1fPCnwyxqqcXwyvB6ibybjkAWBwzqaRgH5MV66Ax",
    publicKey:
      "0x14b6110477c9aa1851a005b9b030f72f3ebd1378af1f245048a3ae7d491c152b",
    derivation:
      "cruel join arch wrap stereo cement roast frame fog drill mandate loyal",
    endpoint: "ws://172.28.1.5:9948",
    initialized: false,
  },
];

// Inserts Keys
const insertKeys = async (api, seed, publicKey) => {
  const insert = await api.rpc.author.insertKey("oracle", seed, publicKey);
  console.log(
    chalk.bgBlack(chalk.red("  Insert Keys  ")) +
      chalk.red(` ${publicKey}`) +
      chalk.green(" Successful")
  );
};

// Bonds the controller account
const bond = async (api, address, keyring) => {
  const bondTx = api.tx.oracle.setSigner(address);
  try {
    await bondTx.signAndSend(keyring);
    console.log(
      chalk.bgBlack(chalk.red("  Bond  ")) +
        chalk.red(` ${address}`) +
        chalk.green(" Successful")
    );
  } catch {
    console.log(chalk.bgGreen("Bond") + "tx failed...");
  }
  await sleep(6000);
};

// Adds Stake to Node
const stake = async (api, amount, keyring) => {
  const stakeTx = api.tx.oracle.addStake(amount);
  try {
    await stakeTx.signAndSend(keyring);
    console.log(
      chalk.bgBlack(chalk.red("  Stake  ")) +
        chalk.red(` ${amount}`) +
        chalk.green(" Successful")
    );
  } catch {
    console.log(chalk.bgGreen("Stake") + "tx failed...");
  }
  await sleep(6000);
};

// Sets price feed URL
const setURL = async (api) => {
  const url = "http://172.28.1.13:3001/price/";
  const key = "0x6f63772d75726c";
  const value =
    "0x687474703a2f2f3137322e32382e312e31333a333030312f70726963652f";
  await api.rpc.offchain.localStorageSet("PERSISTENT", key, value);
  console.log(
    chalk.bgBlack(chalk.red("  Set Feed  ")) +
      chalk.red(` ${url}`) +
      chalk.green(" Successful")
  );
};

// Sends funds to the given account
const sendBalance = async (api, destinationAddress, amount) => {
  const tx = await api.tx.balances.transfer(destinationAddress, amount);
  try {
    await tx.signAndSend(keyring.addFromUri("//Alice"));
    console.log(
      chalk.bgBlack(chalk.red("  Transfer  ")) +
        chalk.red(` ${destinationAddress}`) +
        chalk.green(" Successful")
    );
  } catch {
    console.log(chalk.bgGreen("Transfer") + "transfer tx failed...");
  }
  await sleep(6000);
};

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

// Registers an asset ID with a threshold from `sudo` origin
const addAsset = async (api) => {
  const asset = 0;
  const threshold = 10;
  const minAnswers = 3;
  const maxAnswers = 5;
  const tx = api.tx.oracle.addAssetAndInfo(
    asset,
    threshold,
    minAnswers,
    maxAnswers
  );
  const su = api.tx.sudo.sudo(tx);
  await su.signAndSend(keyring.addFromUri("//Alice"));
  console.log(
    chalk.bgBlack(chalk.red("  Register Asset  ")) +
      chalk.red(` ID: ${asset} `) +
      "with threshold" +
      chalk.red(` ${threshold}% `) +
      chalk.green(" Successful")
  );
};

const main = async () => {
  console.log(chalk.bgBlack(chalk.blue("   Starting Setup   ")));

  for (const node of nodes) {
    console.log(
      chalk.bgBlack(chalk.blue("  Initializing   ")) +
        chalk.bgBlack(chalk.red(`${node.name}   `))
    );
    console.log(
      chalk.bgBlack(chalk.blue("  Connecting to    ")) +
        chalk.bgBlack(chalk.red(`${node.endpoint}   `))
    );
    const api = await connect(node.endpoint);
    const key = keyring.addFromUri(node.derivation);
    await sendBalance(api, node.address, 1234567891234);
    await bond(api, node.address, key);
    await stake(api, "10000000", key);
    await insertKeys(api, node.derivation, node.publicKey);
    await setURL(api);
    await api.disconnect();
    console.log(
      chalk.bgBlack(chalk.blue("  Disconnecting    ")) +
        chalk.bgBlack(chalk.red(`${node.endpoint}   `))
    );
  }

  const api = await connect(nodes[0].endpoint);
  await addAsset(api);
  await api.disconnect();
};

const sleep = (ms) =>
  new Promise((resolve) => {
    setTimeout(() => resolve(), ms);
  });

main();
