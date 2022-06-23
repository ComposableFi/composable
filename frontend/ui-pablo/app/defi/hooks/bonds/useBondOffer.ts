import { BondOffer } from "@/defi/types";
import { useEffect, useMemo, useState } from "react";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import { MockedAsset } from "@/store/assets/assets.types";
import { DEFAULT_NETWORK_ID, fetchBondOffers, fetchVesitngPeriod } from "@/defi/utils";
import { useParachainApi } from "substrate-react";
import { useBlockInterval } from "../useBlockInterval";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

export default function useBondOffer(offerId: string) {
  const { bondOffers, supportedAssets } = useStore();
  const lpRewardingPools = useAllLpTokenRewardingPools();

  const { putBondOffers } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  useEffect(() => {
    console.log(`Update Bond Offers`);
    if (parachainApi) {
      fetchBondOffers(parachainApi).then((decodedOffers) => {
        putBondOffers(decodedOffers);
      });
    }
  }, [parachainApi, putBondOffers]);

  const [selectedBondOffer, setSelectedBondOffer] =
    useState<BondOffer | undefined>(undefined);

  useEffect(() => {
    let offer = bondOffers.list.find((o) => o.offerId.toString() === offerId);
    if (offer) {
      setSelectedBondOffer(offer);
    }
  }, [bondOffers, offerId]);

  const principalAsset = useMemo<
    | {
        baseAsset: MockedAsset | undefined;
        quoteAsset: MockedAsset | undefined;
      }
    | MockedAsset
    | undefined
  >(() => {
    if (
      supportedAssets.length &&
      lpRewardingPools.length &&
      selectedBondOffer
    ) {
      const isLpBasedBond: ConstantProductPool | StableSwapPool | undefined =
        lpRewardingPools.find(
          (pool: ConstantProductPool | StableSwapPool) =>
            pool.lpToken === selectedBondOffer.asset
        );
      let principalAsset:
        | {
            baseAsset: MockedAsset | undefined;
            quoteAsset: MockedAsset | undefined;
          }
        | MockedAsset
        | undefined = undefined;
      if (isLpBasedBond) {
        const baseAsset = supportedAssets.find(
          (a) =>
            a.network[DEFAULT_NETWORK_ID] === isLpBasedBond.pair.base.toString()
        );
        const quoteAsset = supportedAssets.find(
          (a) =>
            a.network[DEFAULT_NETWORK_ID] ===
            isLpBasedBond.pair.quote.toString()
        );

        if (baseAsset || quoteAsset) {
          principalAsset = { baseAsset: undefined, quoteAsset: undefined };

          if (baseAsset) {
            principalAsset.baseAsset = baseAsset;
          }
          if (quoteAsset) {
            principalAsset.quoteAsset = quoteAsset;
          }
        }
      } else {
        principalAsset = supportedAssets.find(
          (a) => a.network[DEFAULT_NETWORK_ID] === selectedBondOffer.asset
        );
      }
      return principalAsset;
    }
    return undefined;
  }, [supportedAssets, lpRewardingPools, selectedBondOffer]);

  const rewardAsset = useMemo<MockedAsset | undefined>(() => {
  if (
    supportedAssets.length &&
    selectedBondOffer
  ) {
    return supportedAssets.find(a => selectedBondOffer.reward.asset === a.network[DEFAULT_NETWORK_ID])
  }
  }, [supportedAssets, selectedBondOffer]);

  const averageBlockTime = useBlockInterval();

  const vestingPeriod = useMemo(() => {
    if (selectedBondOffer && averageBlockTime) {
        return fetchVesitngPeriod({
            interval: averageBlockTime.toString(),
            bondMaturity: selectedBondOffer.maturity
        })
    }
  }, [selectedBondOffer, averageBlockTime])

  const bondPriceUsd = new BigNumber(0);
  const marketPriceUsd = new BigNumber(0);

  return {
    selectedBondOffer,
    vestingPeriod,
    bondPriceUsd,
    marketPriceUsd,
    principalAsset,
    rewardAsset,
  };
}

export type SelectedBondOffer = ReturnType<typeof useBondOffer>