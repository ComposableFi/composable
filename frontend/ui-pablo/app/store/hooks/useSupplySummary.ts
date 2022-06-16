import moment from "moment";
import useStore from "../useStore";

import { useEffect, useMemo, useState } from "react";
import { useParachainApi } from "substrate-react";
import { DEFAULT_DECIMALS } from "../../updaters/constants";
import { useBlockInterval } from "../../utils/defi/hooks/polkadot/useBlockInterval";
import { stringToBigNumber } from "../../utils/stringToBigNumber";
import { fetchApolloPriceByAssetId } from "../../utils/defi/apollo";

const DEFAULT_BLOCK_TIME = 6 * 1000;

type Props = {
  offerId: number;
};

export function useSupplySummary({ offerId }: Props) {
  const { parachainApi } = useParachainApi("picasso");
  const { allBonds } = useStore();
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
        const oracleRewardPrice = await fetchApolloPriceByAssetId(
          parachainApi,
          rewardCurrencyId
        );
        const rewardPriceInUSD = stringToBigNumber(oracleRewardPrice)
          .div(DEFAULT_DECIMALS)
          .toNumber();

        setRewardPriceInUSD(rewardPriceInUSD);
      }
    })();
  }, [parachainApi, selectedBond]);

  if (!selectedBond) {
    return "no-summary";
  }

  const marketPrice = selectedBond.bondOffer.reward.amount
    .div(DEFAULT_DECIMALS)
    .times(rewardPriceInUSD)
    .toNumber();

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
    offerAsset: selectedBond.bondOffer.asset,
    rewardAsset: selectedBond.bondOffer.reward.asset,
    marketPrice,
    vestingPeriod,
  };
}
