import { sendWithBatchAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { expect } from "chai";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";
import BN from "bn.js";

/***
 * This mints a specific `amount` to all specified `assetIDs` to a defined `wallet`.
 *
 * @param api Connected api client
 * @param wallet The wallet receiving the assets.
 * @param sudoKey The sudo key making the transaction.
 * @param assetIDs All assets to be minted to wallet.
 * @param amount Mint amount.
 */
export async function mintAssetsToWallet(
  api: ApiPromise,
  wallet: KeyringPair,
  sudoKey: KeyringPair,
  assetIDs: number[],
  amount: bigint | BN = Pica(900000000)
) {
  const tx = [];
  const balancesBefore = [];
  for (const asset of assetIDs) {
    const pAsset = api.createType("u128", asset);
    balancesBefore.push(parseInt((await api.rpc.assets.balanceOf(pAsset.toString(), wallet.publicKey)).toString()));
    tx.push(api.tx.sudo.sudo(api.tx.assets.mintInto(pAsset, wallet.publicKey, amount)));
  }
  const {
    data: [result]
  } = await sendWithBatchAndWaitForSuccess(api, sudoKey, api.events.sudo.Sudid.is, tx, false);
  expect(result.isOk).to.be.true;
  for (let i = 0; i < assetIDs.length; i++) {
    const newBalance = await api.rpc.assets.balanceOf(assetIDs[i].toString(), wallet.publicKey);
    // ToDo: Enhance comparison by comparing `newBalance = (balanceBefore + amount) - transactionFee`
    expect(parseInt(newBalance.toString())).to.be.greaterThan(balancesBefore[i]);
  }
}

/***
 * Returns the passed amount as 12 decimal tokens for better readability
 * @param Accepts either string or number
 * @returns valid tokens with 12 decimals omitted
 */
export function Pica(value: string | number) {
  return BigInt(value) * BigInt(10 ** 12);
}
