import { Asset, BondOffer } from "shared";
import { useCallback, useEffect, useMemo, useState } from "react";
import { calculateBondROI, DEFAULT_NETWORK_ID } from "@/defi/utils";
import { useParachainApi } from "substrate-react";
import useStore from "@/store/useStore";
import BigNumber from "bignumber.js";
import {
  updateExistingBondOffer,
  useBondedOfferVestingScheduleIds,
  useBondedOfferVestingSchedules,
  useBondOffersSlice,
} from "@/store/bond/bond.slice";
import { useBondedAsset } from "./useBondedAsset";
import { useAssetIdOraclePrice, useAssetPrice } from "../assets";

export default function useBondOffer(offerId: string) {
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);
  const { substrateTokens } = useStore();
  const { tokens } = substrateTokens;
  const { bondOffers } = useBondOffersSlice();
  const vestingScheduleIds = useBondedOfferVestingScheduleIds(offerId);
  const vestingSchedules = useBondedOfferVestingSchedules(offerId);

  const [selectedBondOffer, setSelectedBondOffer] = useState<
    BondOffer | undefined
  >(undefined);

  useEffect(() => {
    let offer = bondOffers.find((o) => o.getBondOfferId() === offerId);
    if (offer) {
      setSelectedBondOffer(offer);
    }
  }, [bondOffers, offerId]);

  const bondedAsset_s = useBondedAsset(selectedBondOffer);

  const rewardAsset = useMemo<Asset | undefined>(() => {
    const assets = Object.values(tokens);
    if (assets.length > 0 && selectedBondOffer) {
      return assets.find((asset) =>
        (asset.getPicassoAssetId(true) as BigNumber).eq(
          selectedBondOffer.getRewardAssetId(true) as BigNumber
        )
      );
    }
  }, [tokens, selectedBondOffer]);

  const rewardAssetPerBond = useMemo(() => {
    if (selectedBondOffer) {
      return (selectedBondOffer.getRewardAssetAmount(true) as BigNumber).div(
        selectedBondOffer.getNumberOfBonds(true) as BigNumber
      );
    }
    return new BigNumber(0);
  }, [selectedBondOffer]);

  const principalAssetPerBond = useMemo(() => {
    if (selectedBondOffer) {
      return selectedBondOffer.getBondPrice(true) as BigNumber;
    }
    return new BigNumber(0);
  }, [selectedBondOffer]);

  const updateBondInfo = useCallback(async () => {
    if (parachainApi && selectedBondOffer) {
      try {
        const bondOffer = await parachainApi.query.bondedFinance.bondOffers(
          selectedBondOffer.getBondOfferId() as string
        );

        const [beneficiary, _offer] = bondOffer.toJSON() as any;
        updateExistingBondOffer(
          BondOffer.fromJSON(
            (selectedBondOffer.getBondOfferId(true) as BigNumber).toNumber(),
            beneficiary,
            _offer
          )
        );
      } catch (err) {
        console.error(err);
      }
    }
  }, [selectedBondOffer, parachainApi]);

  const bondedAssetValue = useAssetPrice(bondedAsset_s);
  const rewardAssetValue = useAssetIdOraclePrice(
    rewardAsset ? rewardAsset.getPicassoAssetId()?.toString() : undefined
  );

  const roi = useMemo(() => {
    if (
      principalAssetPerBond.gt(0) &&
      rewardAssetPerBond.gt(0) &&
      selectedBondOffer
    ) {
      return calculateBondROI(
        bondedAssetValue,
        rewardAssetValue,
        principalAssetPerBond,
        rewardAssetPerBond
      );
    }
    return new BigNumber(0);
  }, [
    bondedAssetValue,
    principalAssetPerBond,
    rewardAssetPerBond,
    rewardAssetValue,
    selectedBondOffer,
  ]);

  return {
    selectedBondOffer,
    rewardAsset,
    updateBondInfo,
    principalAssetPerBond,
    rewardAssetPerBond,
    roi,
    vestingSchedules,
    vestingScheduleIds,
    bondedAsset_s,
  };
}

export type SelectedBondOffer = ReturnType<typeof useBondOffer>;
