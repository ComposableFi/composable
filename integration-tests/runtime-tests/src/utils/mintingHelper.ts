import {sendAndWaitForWithBatch} from "@composable/utils/polkadotjs";
import {expect} from "chai";

/***
 * This mints all specified assets to a specified wallet.
 *
 * @param wallet The wallet receiving the assets.
 * @param sudoKey The sudo key making the transaction.
 * @param assetIDs All assets to be minted to wallet.
 * @param amount Mint amount.
 */
export async function mintAssetsToWallet(wallet, sudoKey, assetIDs:number[], amount= BigInt(300000000000000000000000)) {
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
