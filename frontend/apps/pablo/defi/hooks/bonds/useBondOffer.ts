import { BondOffer, BondPrincipalAsset } from "@/defi/types";
import { useCallback, useEffect, useMemo, useState } from "react";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";
import { MockedAsset } from "@/store/assets/assets.types";
import {
  calculateBondROI,
  decodeBondOffer,
  DEFAULT_NETWORK_ID,
  fetchVestingPeriod,
  getBondPrincipalAsset,
  matchAssetByPicassoId,
} from "@/defi/utils";
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

  const principalAsset: BondPrincipalAsset = useMemo<BondPrincipalAsset>(() => {
    if (selectedBondOffer) {
      return getBondPrincipalAsset(
        selectedBondOffer,
        supportedAssets,
        lpRewardingPools
      );
    } else {
      return {
        lpPrincipalAsset: {
          baseAsset: undefined,
          quoteAsset: undefined,
        },
        simplePrincipalAsset: undefined,
      };
    }
  }, [supportedAssets, lpRewardingPools, selectedBondOffer]);

  const rewardAsset = useMemo<MockedAsset | undefined>(() => {
    if (supportedAssets.length && selectedBondOffer) {
      return supportedAssets.find((a) =>
        matchAssetByPicassoId(a, selectedBondOffer.reward.asset)
      );
    }
  }, [supportedAssets, selectedBondOffer]);

  const averageBlockTime = useBlockInterval();

  const vestingPeriod = useMemo(() => {
    if (selectedBondOffer && averageBlockTime) {
      return fetchVestingPeriod({
        interval: averageBlockTime.toString(),
        bondMaturity: selectedBondOffer.maturity,
      });
    }
  }, [selectedBondOffer, averageBlockTime]);

  const rewardAssetPerBond = useMemo(() => {
    if (selectedBondOffer) {
      return selectedBondOffer.reward.amount.div(selectedBondOffer.nbOfBonds);
    }
    return new BigNumber(0);
  }, [selectedBondOffer]);

  const principalAssetPerBond = useMemo(() => {
    if (selectedBondOffer) {
      return selectedBondOffer.bondPrice;
    }
    return new BigNumber(0);
  }, [selectedBondOffer]);

  const updateBondInfo = useCallback(async () => {
    if (parachainApi && selectedBondOffer) {
      try {
        const bondOffer = await parachainApi.query.bondedFinance.bondOffers(
          selectedBondOffer.offerId.toString()
        );
        const decodedOffer = decodeBondOffer(
          bondOffer,
          selectedBondOffer.offerId.toNumber()
        );
        putBondOffer(decodedOffer);
      } catch (err) {
        console.error(err);
      }
    }
  }, [selectedBondOffer, parachainApi, putBondOffer]);

  const roi = useMemo(() => {
    if (principalAssetPerBond.gt(0) && rewardAssetPerBond.gt(0)) {
      if (
        selectedBondOffer &&
        apollo[selectedBondOffer.asset] &&
        apollo[selectedBondOffer.reward.asset]
      ) {
        return calculateBondROI(
          new BigNumber(apollo[selectedBondOffer.asset]),
          new BigNumber(apollo[selectedBondOffer.reward.asset]),
          principalAssetPerBond,
          rewardAssetPerBond
        );
      }
    }
    return new BigNumber(0);
  }, [principalAssetPerBond, rewardAssetPerBond, apollo, selectedBondOffer]);

  return {
    selectedBondOffer,
    vestingPeriod,
    principalAsset,
    rewardAsset,
    updateBondInfo,
    principalAssetPerBond,
    rewardAssetPerBond,
    roi,
  };
}

export type SelectedBondOffer = ReturnType<typeof useBondOffer>;