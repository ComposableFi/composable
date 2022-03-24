import { ApiPromise, Keyring, WsProvider } from '@polkadot/api';
import {expect} from "chai";
import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";


/**
 * Mints balance for specified wallets.
 **/
export class startBalanceGenerator {
  /**
   * Sends test transaction from Alice to Bob.
   * @param {ApiPromise} api Connected API Promise.
   * @param {Keyring} sudoKey Sudo key minting asset.
   * @param {Keyring} walletReceiverAddress Wallet receiving asset.
   **/
  public static async setBalance(api: ApiPromise, sudoKey, walletReceiverAddress) {
    const {data: [result1],} = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(
        api.tx.assets.mintInto(1, walletReceiverAddress.publicKey, 555555555555)
      )
    );
    expect(result1.isOk).to.be.true;
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
  await startBalanceGenerator.setBalance(api, walletAlice, walletAlice.address);
  await startBalanceGenerator.setBalance(api, walletAlice, walletBob.address);
  console.info("setStartBalance finished!");
  await api.disconnect();
}

if (require.main === module) {
  main().catch(console.error).finally(() => process.exit());
}
