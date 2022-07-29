import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";

export async function borrowHandler(
  api:ApiPromise,
  wallet: KeyringPair,
  marketId,
  amount
) {
  return await sendAndWaitForSuccess(api, wallet,
    api.events.lending.Borrowed.is,
    api.tx.lending.borrow(marketId, amount))
}
