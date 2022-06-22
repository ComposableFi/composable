import useStore from "../../useStore";

import { useMemo } from "react";
import { useParachainApi } from "substrate-react";
import { ISupplySummary } from "../../bonds/bonds.types";

import { fetchVesitngPeriod } from "../../bonds/fetchVestingPeriod";
import { useBlockInterval } from "@/defi/hooks";
import { DEFAULT_NETWORK_ID, fetchApolloPriceByAssetId, fromChainUnits } from "@/defi/utils";

type Props = {
  offerId: number;
};

export function useSupplySummary({
  offerId,
}: Props): "no-summary" | ISupplySummary {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
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
      const rewardPriceInUSD = 
        await fetchApolloPriceByAssetId(
          parachainApi,
          selectedBond.bondOffer.reward.currencyId.toString()
        );
      return fromChainUnits(selectedBond.bondOffer.reward.amount.toString())
        .times(rewardPriceInUSD)
        .toNumber();
    },
    bondPriceInUSD: async () => {
      const principalPriceInUSD = 
        await fetchApolloPriceByAssetId(
          parachainApi,
          selectedBond.bondOffer.currencyId.toString()
        );
      return fromChainUnits(selectedBond.bondOffer.bondPrice.toString())
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
