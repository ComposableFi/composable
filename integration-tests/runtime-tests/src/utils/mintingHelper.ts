import {sendAndWaitForSuccess, sendAndWaitForWithBatch} from "@composable/utils/polkadotjs";
import {expect} from "chai";
import {SubmittableExtrinsic} from "@polkadot/api/promise/types";
import {is} from "ramda";
import {AnyTuple, IEvent} from "@polkadot/types/types";
import * as events from "events";
import {OrmlTokensAccountData} from "@composable/types/interfaces";

/***
 * This mints all specified assets to a specified wallet.
 * The best way would be to make a list of all transactions, and sending them at once.
 * But due to issues with our current handler when sending transactions at the same time (Priority to low event),
 * we send them one after another. Check [CU-20jc9ug]
 *
 * @param wallet The wallet receiving the assets.
 * @param sudoKey The sudo key making the transaction.
 * @param assetIDs All assets to be minted to wallet.
 * @param amount Mint amount.
 */
export async function mintAssetsToWallet(wallet, sudoKey, assetIDs:number[], amount= BigInt(300000000000000000000000)) {
  const pAmount = api.createType('u128', amount);
  const tx = [];
  for (const asset of assetIDs) {
    const pAsset = api.createType('u128', asset);
    tx.push(api.tx.sudo.sudo(
        api.tx.assets.mintInto(pAsset, wallet.publicKey, amount)
    ));
  }
  const {data:[result],} = await sendAndWaitForWithBatch(api, sudoKey, api.events.sudo.Sudid.is, tx, false);
  expect(result.isOk).to.be.true;
}
