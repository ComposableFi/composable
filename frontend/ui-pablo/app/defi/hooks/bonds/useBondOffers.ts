import { ConstantProductPool, OfferRow, StableSwapPool } from "@/defi/types";
import { useEffect, useMemo } from "react";
import useStore from "@/store/useStore";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";
import BigNumber from "bignumber.js";

import { MockedAsset } from "@/store/assets/assets.types";
import { DEFAULT_NETWORK_ID, fetchBondOffers, matchAssetByPicassoId } from "@/defi/utils";
import { useParachainApi } from "substrate-react";

export type BondPrincipalAsset = {
  lpPrincipalAsset:
    | {
        baseAsset: MockedAsset | undefined;
        quoteAsset: MockedAsset | undefined;
      };
  simplePrincipalAsset: MockedAsset | undefined;
};

export default function useBondOffers(): OfferRow[] {
  const { bondOffers, supportedAssets, apollo } = useStore();
  const lpRewardingPools = useAllLpTokenRewardingPools();

  const { putBondOffers } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  useEffect(() => {
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
      BondPrincipalAsset = {
        lpPrincipalAsset: {
          baseAsset: undefined,
          quoteAsset: undefined
        },
        simplePrincipalAsset: undefined
      };
      if (isLpBasedBond) {
        const baseAsset = supportedAssets.find(
          (asset) =>
            matchAssetByPicassoId(asset, isLpBasedBond.pair.base.toString())
        );
        const quoteAsset = supportedAssets.find(
          (asset) =>
            matchAssetByPicassoId(asset, isLpBasedBond.pair.quote.toString())
        );

        principalAsset.lpPrincipalAsset = { baseAsset, quoteAsset };
      } else {
        principalAsset.simplePrincipalAsset = supportedAssets.find(
          (asset) => matchAssetByPicassoId(asset, bondOffer.asset)
        );
      }

      const rewardAssetPerBond = bondOffer.reward.amount.div(
        bondOffer.nbOfBonds
      );
      const principalAssetPerBond = bondOffer.bondPrice;
      let roi = new BigNumber(0),
      principalPriceUsd = new BigNumber(apollo[bondOffer.asset] || 0),
      rewardPriceUsd = new BigNumber(apollo[bondOffer.reward.asset] || 0);

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
