import useStore from "../useStore";

import { useMemo } from "react";
import { useParachainApi } from "substrate-react";
import { useBlockInterval } from "../../utils/defi/hooks/polkadot/useBlockInterval";
import { getAppoloPriceInUSD } from "../../utils/defi/apollo";
import { ISupplySummary } from "../bonds/bonds.types";
import { fromPica } from "../../utils/defi/fromPica";
import { fetchVesitngPeriod } from "../bonds/fetchVestingPeriod";

type Props = {
  offerId: number;
};

export function useSupplySummary({
  offerId,
}: Props): "no-summary" | ISupplySummary {
  const { parachainApi } = useParachainApi("picasso");
  const { allBonds } = useStore();
  const interval = useBlockInterval();

  const selectedBond = useMemo(
    () => allBonds.find((bond) => bond.offerId === offerId),
    [allBonds, offerId]
  );

  if (!selectedBond || !parachainApi) {
    return "no-summary";
  }

  return {
    principalAsset: selectedBond.bondOffer.asset,
    rewardAsset: selectedBond.bondOffer.reward.asset,
    marketPriceInUSD: async () => {
      const rewardPriceInUSD = (
        await getAppoloPriceInUSD(
          parachainApi,
          selectedBond.bondOffer.reward.currencyId
        )
      ).toNumber();
      return fromPica(selectedBond.bondOffer.reward.amount)
        .times(rewardPriceInUSD)
        .toNumber();
    },
    bondPriceInUSD: async () => {
      const principalPriceInUSD = (
        await getAppoloPriceInUSD(
          parachainApi,
          selectedBond.bondOffer.currencyId
        )
      ).toNumber();
      return fromPica(selectedBond.bondOffer.bondPrice)
        .times(principalPriceInUSD)
        .toNumber();
    },
    roi: selectedBond.roi.toNumber(),
    vestingPeriod: fetchVesitngPeriod({
      interval: interval ? interval.toNumber() : undefined,
      bondMaturity: selectedBond.bondOffer.maturity,
    }),
  };
}
