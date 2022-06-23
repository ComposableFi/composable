import { OfferRow } from "@/defi/types";
import { useEffect, useMemo } from "react";
import useStore from "@/store/useStore";
import { useAllLpTokenRewardingPools } from "@/store/hooks/useAllLpTokenRewardingPools";
import BigNumber from "bignumber.js";
import { ConstantProductPool, StableSwapPool } from "@/store/pools/pools.types";
import { MockedAsset } from "@/store/assets/assets.types";
import { DEFAULT_NETWORK_ID, fetchBondOffers } from "@/defi/utils";
import { useParachainApi } from "substrate-react";
import { useBlockInterval } from "../useBlockInterval";

export default function useBondOffers(): OfferRow[] {
  const { bondOffers, supportedAssets } = useStore();
  const lpRewardingPools = useAllLpTokenRewardingPools();

  const { putBondOffers } = useStore();
  const { parachainApi } = useParachainApi(DEFAULT_NETWORK_ID);

  useEffect(() => {
    console.log(`Update Bond Offers`)
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

      return {
        offerId: bondOffer.offerId,
        roi: new BigNumber(0),
        totalPurchased: new BigNumber(0),
        bondPrice: bondOffer.bondPrice,
        principalAsset,
      };
    });
  }, [bondOffers, lpRewardingPools, supportedAssets]);

  return _bondOffers;
}
