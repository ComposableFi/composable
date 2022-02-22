import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";

export async function borrowHandler(wallet, marketId, amount) {
  return await sendAndWaitForSuccess(api, wallet,
    api.events.lending.Borrowed.is,
    api.tx.lending.borrow(marketId, amount))
}
