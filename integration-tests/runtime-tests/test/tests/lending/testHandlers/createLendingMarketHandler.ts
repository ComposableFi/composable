import {sendAndWaitForSuccess} from "@composable/utils/polkadotjs";


export async function createLendingMarketHandler(
  wallet,
  collateralFactor,
  underCollaterializedWarnPercent,
  liquidators,
  interestRateModel,
  currencyPair,
  reservedFactor
) {
  const input = api.createType('ComposableTraitsLendingCreateInput', {
    updatable: api.createType('ComposableTraitsLendingUpdateInput', {
      collateralFactor: collateralFactor,
      underCollaterializedWarnPercent: underCollaterializedWarnPercent,
      liquidators: liquidators,
      interestRateModel: interestRateModel,
      currencyPair: currencyPair
    }),
    reservedFactor: reservedFactor
  });

  return api.tx.lending.createMarket(input).signAndSend(wallet);
}