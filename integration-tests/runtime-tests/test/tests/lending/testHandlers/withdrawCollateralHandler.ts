import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";



export async function withdrawCollateralHandler(wallet, marketId, amount) {
  return await sendAndWaitForSuccess(api, wallet,
    api.events.lending.CollateralWithdrawn.is,
    api.tx.lending.withdrawCollateral(marketId, amount));
}
