import {ApiPromise} from "@polkadot/api";
import {KeyringPair} from "@polkadot/keyring/types";
import {sendWithBatchAndWaitForSuccess} from "./txClient";

export async function mintAssetsToWallet(
  api: ApiPromise,
  wallet: KeyringPair,
  sudoKey: KeyringPair,
  assetIDs: number[] | string[],
  amount: string,
  chain: string
) {
  const tx = [];
  for (const asset of assetIDs) {
    const pAsset = api.createType("u128", asset);
    if(chain === 'picasso'){
      tx.push(api.tx.sudo.sudo(api.tx.assets.mintInto(pAsset, wallet.publicKey, amount)));
    } else {
      tx.push(api.tx.sudo.sudo(api.tx.assets.mintInto(pAsset, wallet.publicKey, amount)));
    }
  }
  const {
    data: [result]
  } = await sendWithBatchAndWaitForSuccess(api, sudoKey, api.events.sudo.Sudid.is, tx, false);
  return result;
}