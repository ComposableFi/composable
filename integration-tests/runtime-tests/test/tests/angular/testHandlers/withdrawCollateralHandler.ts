import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { ApiPromise } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";


export async function withdrawCollateralHandler(
  api: ApiPromise,
  wallet: KeyringPair,
  marketId,
  amount
) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.lending.CollateralWithdrawn.is,
    api.tx.lending.withdrawCollateral(marketId, amount)
  );
}
