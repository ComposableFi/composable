import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";
import { KeyringPair } from "@polkadot/keyring/types";
import { ApiPromise } from "@polkadot/api";

// ToDo: Add types!
export async function createLendingMarketHandler(
  api: ApiPromise,
  wallet:KeyringPair,
  collateralFactor,
  underCollateralizedWarnPercent,
  maxPriceAge,
  liquidators,
  interestRateModel,
  currencyPair,
  reservedFactor
) {
  const input = api.createType('ComposableTraitsLendingCreateInput', {
    updatable: api.createType('ComposableTraitsLendingUpdateInput', {
      collateralFactor: collateralFactor,
      underCollateralizedWarnPercent: underCollateralizedWarnPercent,
      maxPriceAge: maxPriceAge,
      liquidators: liquidators,
      currencyPair: currencyPair,
      reservedFactor: reservedFactor,
      interestRateModel: interestRateModel,
    }),
  });

  return await sendAndWaitForSuccess(
    api,
    wallet,
    api.events.treasury.Deposit.is,
    api.tx.lending.createMarket(input, false),
    false
  );
}