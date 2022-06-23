import { OfferRow } from "@/defi/types";
import { useEffect, useMemo } from "react";
import useStore from "@/store/useStore";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";
import BigNumber from "bignumber.js";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import { MockedAsset } from "@/store/assets/assets.types";
import { DEFAULT_NETWORK_ID, fetchBondOffers } from "@/defi/utils";
import { useParachainApi } from "substrate-react";

export default function useBondOffers(): OfferRow[] {
  const { bondOffers, supportedAssets, apollo } = useStore();
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

  const _bondOffers = useMemo(() => {
    return bondOffers.list.map((bondOffer) => {
      const isLpBasedBond: ConstantProductPool | StableSwapPool | undefined =
        lpRewardingPools.find(
          (pool: ConstantProductPool | StableSwapPool) =>
            pool.lpToken === bondOffer.asset
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
          (a) => a.network[DEFAULT_NETWORK_ID] === bondOffer.asset
        );
      }

      const rewardAssetPerBond = bondOffer.reward.amount.div(
        bondOffer.nbOfBonds
      );
      const principalAssetPerBond = bondOffer.bondPrice;
      let roi = new BigNumber(0),
        principalPriceUsd = new BigNumber(0),
        rewardPriceUsd = new BigNumber(0);

      if (apollo[bondOffer.asset]) {
        principalPriceUsd = new BigNumber(apollo[bondOffer.asset]);
      }
      if (apollo[bondOffer.reward.asset]) {
        rewardPriceUsd = new BigNumber(apollo[bondOffer.reward.asset]);
      }

      if (principalPriceUsd.gt(0) && rewardPriceUsd.gt(0)) {
        const initialInv = principalPriceUsd.times(principalAssetPerBond);
        const finalInv = rewardPriceUsd.times(rewardAssetPerBond);
        roi = finalInv.minus(initialInv).div(initialInv).times(100);
      }

      return {
        offerId: bondOffer.offerId,
        roi,
        totalPurchased: new BigNumber(0),
        bondPrice: principalPriceUsd.times(principalAssetPerBond),
        principalAsset,
        rewardAssetPerBond,
        principalAssetPerBond,
      };
    });
  }, [bondOffers, lpRewardingPools, supportedAssets, apollo]);

  return _bondOffers;
}
