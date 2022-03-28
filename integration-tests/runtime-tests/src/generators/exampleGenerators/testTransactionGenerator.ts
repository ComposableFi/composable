import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';

/**
 * Generates a test transaction.
 * This is mainly an example on how to add a new generator.
**/
export class testTransactionGenerator {
  /**
   * Sends test transaction from Alice to Bob.
   * @param {ApiPromise} api Connected API Promise.
   * @param {Keyring} walletSender Wallet sending asset.
   * @param {Keyring} walletReceiverAddress Wallet receiving asset.
  **/
  public static async testTransaction(api: ApiPromise, walletSender, walletReceiverAddress) {
    const transfer = api.tx.assets.transferNative(walletReceiverAddress, 12345678910, true);
    const hash = await transfer.signAndSend(walletSender, { nonce: -1 });
    console.debug('Transfer sent with hash', hash.toHex());
  }
}

async function main() {
  const endpoint = `ws://${process.env.ENDPOINT}`;
  // Instantiate the API
  const provider = new WsProvider(endpoint);
  const api = await ApiPromise.create({ provider: provider });
  // Constuct the keyring after the API (crypto has an async init)
  const keyring = new Keyring({ type: 'sr25519' });
  const walletAlice = keyring.addFromUri('//Alice');
  const walletBob = keyring.addFromUri('//Bob');
  await testTransactionGenerator.testTransaction(api, walletAlice, walletBob.address);
  console.info("setStartBalance finished!");
  await api.disconnect();
}

if (require.main === module) {
  main().catch(console.error).finally(() => process.exit());
}
