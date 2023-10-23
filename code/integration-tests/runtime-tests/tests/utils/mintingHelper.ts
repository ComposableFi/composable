import {ApiPromise} from "@polkadot/api";
import {KeyringPair} from "@polkadot/keyring/types";
import {sendAndWaitForSuccess, sendWithBatchAndWaitForSuccess} from "./txClient";

export async function mintAssetsToWallets(
  api: ApiPromise,
  wallets: KeyringPair[] | KeyringPair,
  sudoKey: KeyringPair,
  assetIDs: number[] | string[],
  amount: string,
  chain: string
) {
  const tx = [];
  for (const asset of assetIDs) {
    const pAsset = api.createType("u128", asset);
    if (isKeyringPair(wallets)) {
      tx.push(api.tx.sudo.sudo(api.tx.assets.mintInto(pAsset, wallets.publicKey, amount)));

    } else {
      for (const wallet of wallets) {
        tx.push(api.tx.sudo.sudo(api.tx.assets.mintInto(pAsset, wallet.publicKey, amount)));

      }
    }
  }
  const {
    data: [result]
  } = await sendWithBatchAndWaitForSuccess(api, sudoKey, api.events.sudo.Sudid.is, tx, false);
  return result;
}

export function isKeyringPair(wallet: object): wallet is KeyringPair {
  return "address" in wallet;
}

export async function mintAssetsOnRelays(apis: ApiPromise[], sudoKey: KeyringPair, address: string){
  await Promise.all(apis.map(async api =>{
    const dest = api.createType('MultiAddress', {
      id: api.createType('AccountId', address)
    });
    const amount = api.createType('Compact<u128>', "1000000000000000");
    await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.system.ExtrinsicSuccess.is,
      api.tx.balances.transfer(dest, amount),
      false
    )
  }));
}