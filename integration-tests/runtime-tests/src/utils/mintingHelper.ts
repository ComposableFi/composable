import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";
import {expect} from "chai";

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
export async function mintAssetsToWallet(wallet, sudoKey, assetIDs:number[], amount=999999999999999) {
  for (const asset of assetIDs) {
    const {data: [result]} = await sendAndWaitForSuccess(
      api,
      sudoKey,
      api.events.sudo.Sudid.is,
      api.tx.sudo.sudo(
        api.tx.assets.mintInto(asset, wallet.publicKey, amount)
      )
    )
    expect(result.isOk).to.be.true;
  }
}
