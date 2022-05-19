import { sendAndWaitForSuccess } from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";



export async function depositCollateralHandler(
  api: ApiPromise,
  wallet: KeyringPair,
  marketId,
  amount
) {
  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.lending.CollateralDeposited.is,
    api.tx.lending.depositCollateral(marketId, amount)
  );
}
