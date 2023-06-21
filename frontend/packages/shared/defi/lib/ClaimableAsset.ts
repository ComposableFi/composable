import BigNumber from "bignumber.js";
import { Asset } from "./assets/Asset";
import { ApiPromise } from "@polkadot/api";
import { TokenId } from "tokens";

export class ClaimableAsset extends Asset {
  protected __claimable: BigNumber;

  static fromAsset(asset: Asset, claimable: BigNumber): ClaimableAsset {
    return new ClaimableAsset(
      asset.getPicassoAssetId(true) as BigNumber,
      asset.getName(),
      asset.getSymbol(),
      asset.getIconUrl(),
      claimable,
      asset.getTokenId(),
      asset.getApi()
    );
  }

  constructor(
    picassoAssetId: BigNumber,
    name: string,
    symbol: string,
    iconUrl: string,
    claimableAmount: BigNumber,
    tokenId: TokenId,
    api?: ApiPromise
  ) {
    super(name, symbol, iconUrl, tokenId, api);
    this.setIdOnChain("picasso", picassoAssetId);
    this.__claimable = claimableAmount;
  }

  setClaimable(claimableAmount: BigNumber) {
    this.__claimable = claimableAmount;
  }

  getClaimable(): BigNumber {
    return this.__claimable;
  }
}
