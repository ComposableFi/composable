import { BondOffer } from "@/defi/types";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";
import { ConstantProductPool, StableSwapPool } from "@/defi/types";
import { MockedAsset } from "@/store/assets/assets.types";
import { decodeBondOffer, DEFAULT_NETWORK_ID, fetchVesitngPeriod } from "@/defi/utils";
import { useParachainApi } from "substrate-react";
import { useBlockInterval } from "../useBlockInterval";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";

export default function useBondOffer(offerId: string) {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { bondOffers, supportedAssets, apollo, putBondOffer } = useStore();
  const lpRewardingPools = useAllLpTokenRewardingPools();

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

  const rewardAssetPerBond = useMemo(() => {
    if (selectedBondOffer) {
      return selectedBondOffer.reward.amount.div(
        selectedBondOffer.nbOfBonds
      );
    }
    return new BigNumber(0)
  }, [selectedBondOffer])

  const principalAssetPerBond = useMemo(() => {
    if (selectedBondOffer) {
      return selectedBondOffer.bondPrice;
    }
    return new BigNumber(0)
  }, [selectedBondOffer]);

  const updateBondInfo = useCallback(async() => {
    if (parachainApi && selectedBondOffer) {
      try {
        const bondOffer = await parachainApi.query.bondedFinance.bondOffers(selectedBondOffer.offerId.toString());
        const decodedOffer = decodeBondOffer(bondOffer.toHuman(), selectedBondOffer.offerId.toNumber());
        putBondOffer(decodedOffer);
      } catch (err) {
        console.error(err)
      }
    }
  }, [selectedBondOffer, parachainApi, putBondOffer])

  const roi = useMemo(() => {
    if (principalAssetPerBond.gt(0) && rewardAssetPerBond.gt(0)) {
      if (selectedBondOffer && apollo[selectedBondOffer.asset] && apollo[selectedBondOffer.reward.asset]) {
        let rewardPrice = new BigNumber(apollo[selectedBondOffer.reward.asset]);
        let principalPrice = new BigNumber(apollo[selectedBondOffer.asset]);
        if (rewardPrice.gt(0) && principalPrice.gt(0)) {
          const initialInv = principalPrice.times(principalAssetPerBond);
          const finalInv = rewardAssetPerBond.times(rewardPrice);
          return finalInv.minus(initialInv).div(initialInv).times(100);
        }
      }
    }
    return new BigNumber(0)
  }, [principalAssetPerBond, rewardAssetPerBond, apollo, selectedBondOffer])

  return {
    selectedBondOffer,
    vestingPeriod,
    principalAsset,
    rewardAsset,
    updateBondInfo,
    principalAssetPerBond,
    rewardAssetPerBond,
    roi
  };
}

export type SelectedBondOffer = ReturnType<typeof useBondOffer>