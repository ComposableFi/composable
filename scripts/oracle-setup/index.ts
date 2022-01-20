import { ApiPromise } from "@polkadot/api";
import { stringToHex } from "@polkadot/util/string";
import { getApi, sendAndWait, getKeypair } from "./helpers";
import { endpoint, nodes, priceFeed } from "./config";
import { AddressOrPair } from "@polkadot/api/types";

// Inserts Keys
const insertKeys = async (api: ApiPromise, seed: string, publicKey: string) => {
  const insert = await api.rpc.author.insertKey("orac", seed, publicKey);
  console.log("  Insert Keys  ");
  console.log(` ${publicKey}`);
  console.log(" Successful");
};

// Bonds the controller account
const bond = async (
  api: ApiPromise,
  address: string,
  keyring: AddressOrPair
) => {
  const bondTx = api.tx.oracle.setSigner(address);
  const message = await sendAndWait(api, bondTx, keyring);
  console.log({ message });
};

const setURL = async (api: ApiPromise) => {
  const url = priceFeed;
  const key = "0x6f63772d75726c";
  const value = stringToHex(url);
  await api.rpc.offchain.localStorageSet("PERSISTENT", key, value);
  console.log("  Set Feed  " + ` ${url}` + " Successful");
};

// Registers an asset ID with a threshold from `sudo` origin
const addAsset = async (api: ApiPromise, keyring: AddressOrPair) => {
  const asset = 0;
  const threshold = 10;
  const minAnswers = 2;
  const maxAnswers = 3;
  const blockInterval = 10;
  const reward = 10;
  const slash = 10;

  const tx = api.tx.oracle.addAssetAndInfo(
    asset,
    threshold,
    minAnswers,
    maxAnswers,
    blockInterval,
    reward,
    slash
  );
  const su = api.tx.sudo.sudo(tx);
  const message = await sendAndWait(api, su, keyring);

  console.log({ message });
  console.log(
    "  Register Asset  " +
      `ID: ${asset} ` +
      "with threshold" +
      `${threshold}% ` +
      " Successful"
  );
};

// Sends funds to the given account
const sendBalance = async (
  api: ApiPromise,
  keyring: AddressOrPair,
  destinationAddress: string,
  amount: string
) => {
  const tx = await api.tx.assets.transfer(
    "1",
    destinationAddress,
    amount,
    false
  );
  const message = await sendAndWait(api, tx, keyring);
  console.log({ message });
};

const main = async () => {
  const api = await getApi("alice", endpoint);
  const keyring = await getKeypair("Alice");
  await addAsset(api, keyring);
  await setURL(api);
  await api.disconnect();

  for (let i = 0; i <= nodes.length; i++) {
    const api = await getApi(nodes[i].name.toLocaleLowerCase(), endpoint);
    const keyring = await getKeypair(nodes[i].name);
    await insertKeys(api, nodes[i].mnemonic, nodes[i].publicKey);
    try {
      await bond(api, nodes[i].address, keyring);
    } catch (e) {
      console.log("error with bond, could be already bonded", nodes[i].address);
    }
    console.log("bond done");
    await sendBalance(api, keyring, nodes[i].address, "1000000000000");
    console.log("balance issue");
    await api.disconnect();
    console.log({ i });
  }
};

main();
