import moment from "moment";
import useStore from "../useStore";

import { useEffect, useMemo, useState } from "react";
import { useParachainApi } from "substrate-react";
import { DEFAULT_DECIMALS } from "../../updaters/constants";
import { useBlockInterval } from "../../utils/defi/hooks/polkadot/useBlockInterval";
import { getAppoloPriceInUSD } from "../../utils/defi/apollo";
import { ISupplySummary } from "../bonds/bonds.types";

const DEFAULT_BLOCK_TIME = 6 * 1000;

type Props = {
  offerId: number;
};

export function useSupplySummary({
  offerId,
}: Props): "no-summary" | ISupplySummary {
  const { parachainApi } = useParachainApi("picasso");
  const { allBonds } = useStore();
  const [principalPriceInUSD, setPrincipalPriceInUSD] = useState(0);
  const [rewardPriceInUSD, setRewardPriceInUSD] = useState(0);
  const interval = useBlockInterval();

  const selectedBond = useMemo(
    () => allBonds.find((bond) => bond.offerId === offerId),
    [allBonds, offerId]
  );

  //Calculate reward price in USD
  useEffect(() => {
    (async () => {
      if (parachainApi && selectedBond) {
        const rewardCurrencyId = selectedBond.bondOffer.reward.currencyId;
        const rewardPriceInUSD = await getAppoloPriceInUSD(
          parachainApi,
          rewardCurrencyId
        );
        const principalCurrencyId = selectedBond.bondOffer.currencyId;
        const principalPriceInUSD = await getAppoloPriceInUSD(
          parachainApi,
          principalCurrencyId
        );
        setRewardPriceInUSD(rewardPriceInUSD.toNumber());
        setPrincipalPriceInUSD(principalPriceInUSD.toNumber());
      }
    })();
  }, [parachainApi, selectedBond]);

  if (!selectedBond) {
    return "no-summary";
  }

  const marketPriceInUSD = selectedBond.bondOffer.reward.amount
    .div(DEFAULT_DECIMALS)
    .times(rewardPriceInUSD)
    .toNumber();

  const bondPriceInUSD = selectedBond.bondOffer.bondPrice
    .div(DEFAULT_DECIMALS)
    .times(principalPriceInUSD)
    .toNumber();

  const discountInPercentage = (marketPriceInUSD / bondPriceInUSD) * 100;

  const bondMaturity = selectedBond.bondOffer.maturity;

  let vestingPeriod = bondMaturity
    ? "Infinite"
    : moment(bondMaturity * DEFAULT_BLOCK_TIME).format("d[D] h[H] m[M] s[S]");

  if (interval) {
    vestingPeriod =
      bondMaturity === "Infinite"
        ? "Infinite"
        : moment(bondMaturity * interval.toNumber()).format(
            "d[D] h[H] m[M] s[S]"
          );
  }

  return {
    principalAsset: selectedBond.bondOffer.asset,
    rewardAsset: selectedBond.bondOffer.reward.asset,
    marketPriceInUSD,
    roi: selectedBond.roi.toNumber(),
    discountInPercentage,
    vestingPeriod,
  };
}
