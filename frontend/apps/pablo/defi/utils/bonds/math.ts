import BigNumber from "bignumber.js";

export function calculateBondROI(
  principalAssetPriceInUSD: BigNumber,
  rewardAssetPriceInUSD: BigNumber,
  principalAssetAmountPerBond: BigNumber,
  rewardAssetAmountPerBond: BigNumber
): BigNumber {
  const _investment = principalAssetPriceInUSD.times(
    principalAssetAmountPerBond
  );
  const _return = rewardAssetPriceInUSD.times(rewardAssetAmountPerBond);

  if (_investment.gt(0)) {
    return _return.minus(_investment).div(_investment).times(100);
  }

  return new BigNumber(0);
}

export function calculateTotalPurchasedValue(
  principalAssetPriceInUSD: BigNumber,
  principalAssetPerBond: BigNumber,
  amountOfBondsPurchased: BigNumber
): BigNumber {
  return principalAssetPerBond
    .times(principalAssetPriceInUSD)
    .times(amountOfBondsPurchased);
}
