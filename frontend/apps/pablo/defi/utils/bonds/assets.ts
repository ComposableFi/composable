import {
  BondOffer,
  BondPrincipalAsset,
  ConstantProductPool,
  StableSwapPool,
} from "@/defi/types";
import { MockedAsset } from "@/store/assets/assets.types";
import { matchAssetByPicassoId } from "../assets";

export function getBondPrincipalAsset(
  bondOffer: BondOffer,
  supportedAssets: MockedAsset[],
  lpRewardingPools: Array<StableSwapPool | ConstantProductPool>
): BondPrincipalAsset {
  const lpBasedBond: ConstantProductPool | StableSwapPool | undefined =
    lpRewardingPools.find(
      (pool: ConstantProductPool | StableSwapPool) =>
        pool.lpToken === bondOffer.asset
    );
  let principalAsset: BondPrincipalAsset = {
    lpPrincipalAsset: {
      baseAsset: undefined,
      quoteAsset: undefined,
    },
    simplePrincipalAsset: undefined,
  };
  if (lpBasedBond !== undefined) {
    const baseAsset = supportedAssets.find((asset) =>
      matchAssetByPicassoId(asset, lpBasedBond.pair.base.toString())
    );
    const quoteAsset = supportedAssets.find((asset) =>
      matchAssetByPicassoId(asset, lpBasedBond.pair.quote.toString())
    );

    principalAsset.lpPrincipalAsset = { baseAsset, quoteAsset };
  } else {
    principalAsset.simplePrincipalAsset = supportedAssets.find((asset) =>
      matchAssetByPicassoId(asset, bondOffer.asset)
    );
  }

  return principalAsset;
}
