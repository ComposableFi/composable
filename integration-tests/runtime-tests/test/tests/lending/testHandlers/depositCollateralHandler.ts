import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";



export async function depositCollateralHandler(wallet, marketId, amount) {
  return await sendAndWaitForSuccess(api, wallet,
    api.events.lending.CollateralDeposited.is,
    api.tx.lending.depositCollateral(marketId, amount))
}