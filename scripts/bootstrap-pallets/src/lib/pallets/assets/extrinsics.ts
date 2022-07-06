import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { sendAndWaitForSuccess } from "@bootstrap-pallets/lib";
import BigNumber from "bignumber.js";
import { u8aToHex } from "@polkadot/util";
import { fromChainUnits, logger } from "@bootstrap-pallets/utils";

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
export async function mintAssetsToWallets(
  api: ApiPromise,
  wallets: KeyringPair[],
  sudoKey: KeyringPair,
  assetIDs: string[],
  amount: BigNumber
) {
  for (const asset of assetIDs) {
    for (const wallet of wallets) {
      await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.assets.mintInto(asset, wallet.publicKey, amount.toString()))
      );

      logger.log("info", `Minted ${fromChainUnits(amount.toString()).toString()} ${asset} for ${u8aToHex(wallet.publicKey)}`);
    }
  }
}

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
export async function mintAssetsToAddress(
  api: ApiPromise,
  wallets: string[],
  sudoKey: KeyringPair,
  assetIDs: string[],
  amount: string
) {
  for (const asset of assetIDs) {
    for (const wallet of wallets) {
      const {
        data: [result]
      } = await sendAndWaitForSuccess(
        api,
        sudoKey,
        api.events.sudo.Sudid.is,
        api.tx.sudo.sudo(api.tx.assets.mintInto(asset, wallet, amount))
      );

      logger.log("info", `Minted ${fromChainUnits(amount.toString()).toString()} ${asset} for ${wallet}`);
    }
  }
}
