import BigNumber from "bignumber.js";
import { ApiPromise } from "@polkadot/api";
import { Asset } from "./assets/Asset";
import { TokenId } from "tokens";

export class LiquidityProviderToken extends Asset {
  protected __underlyingAssets: Asset[];

  constructor(
    underlyingAssets: Asset[],
    tokenAssetId: BigNumber,
    tokenId: TokenId,
    api?: ApiPromise
  ) {
    super(
      `LP ${underlyingAssets.map((x) => x.getSymbol()).join("/")}`,
      `${underlyingAssets.map((x) => x.getSymbol()).join("/")}`,
      "-",
      tokenId,
      api
    );
    this.setIdOnChain("picasso", tokenAssetId);
    this.__underlyingAssets = underlyingAssets;
  }

  getUnderlyingAssets(): Asset[] {
    return this.__underlyingAssets;
  }

  getUnderlyingAssetJSON(): Array<{ icon: string; label: string }> {
    return this.__underlyingAssets.map((asset) => {
      return {
        icon: asset.getIconUrl(),
        label: asset.getSymbol(),
      };
    });
  }
}

export class OwnedLiquidityProviderToken extends LiquidityProviderToken {
  protected __balance: BigNumber;

  constructor(
    api: ApiPromise,
    underlyingAssets: Asset[],
    tokenAssetId: BigNumber,
    tokenId: TokenId,
    balance: BigNumber
  ) {
    super(underlyingAssets, tokenAssetId, tokenId, api);
    this.__balance = balance;
  }

  public setBalance(balance: BigNumber) {
    this.__balance = balance;
  }

  public getBalance(): BigNumber {
    return this.__balance;
  }
}
