import { sendWithBatchAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { expect } from "chai";
import { KeyringPair } from "@polkadot/keyring/types";

/***
 * This mints all specified assets to a specified wallet.
 *
 * @param wallet The wallet receiving the assets.
 * @param sudoKey The sudo key making the transaction.
 * @param assetIDs All assets to be minted to wallet.
 * @param amount Mint amount.
 */
export async function mintAssetsToWallet(
  wallet: KeyringPair,
  sudoKey: KeyringPair,
  assetIDs: number[],
  amount = BigInt(300000000000000000000000)
) {
  const tx = [];
  const balancesBefore = [];
  for (const asset of assetIDs) {
    const pAsset = api.createType("u128", asset);
    balancesBefore.push(parseInt((await api.rpc.assets.balanceOf(pAsset.toString(), wallet.publicKey)).toString()));
    tx.push(api.tx.sudo.sudo(
      api.tx.assets.mintInto(pAsset, wallet.publicKey, amount)
    ));
  }
  const { data: [result] } = await sendWithBatchAndWaitForSuccess(api, sudoKey, api.events.sudo.Sudid.is, tx, false);
  expect(result.isOk).to.be.true;
  for (let i = 0; i < assetIDs.length; i++) {
    const newBalance = await api.rpc.assets.balanceOf(assetIDs[i].toString(), wallet.publicKey);
    // ToDo: Enhance comparison by comparing `newBalance = (balanceBefore + amount) - transactionFee`
    expect(parseInt(newBalance.toString())).to.be.greaterThan(balancesBefore[i]);
  }
}
