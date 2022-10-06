import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "../unit";
import BigNumber from "bignumber.js";

export class Asset {
  protected __api: ApiPromise;
  protected __picassoAssetId: BigNumber;
  protected __name: string;
  protected __symbol: string;
  protected __iconUrl: string;
  protected __parachainAssetIds: Record<string, BigNumber>;

  constructor(
    api: ApiPromise,
    picassoAssetId: BigNumber,
    name: string,
    symbol: string,
    iconUrl: string
  ) {
    this.__api = api;
    this.__picassoAssetId = picassoAssetId;
    this.__name = name;
    this.__symbol = symbol;
    this.__iconUrl = iconUrl;
    this.__parachainAssetIds = {};
  }

  // get picassoAssetId(): string {
  //     return this.__picassoAssetId.toString();
  // }

  // get symbol() {
  //     return this.__symbol;
  // }

  // get name() {
  //     return this.__name;
  // }

  // get iconUrl() {
  //     return this.__iconUrl;
  // }

  async balanceOf(account: string): Promise<BigNumber> {
    try {
      const _assetId = this.__api.createType(
        "CustomRpcCurrencyId",
        this.__picassoAssetId.toString()
      );
      const _accountId32 = this.__api.createType("AccountId32", account);

      // @ts-ignore
      const balance = await this.__api.rpc.assets.balanceOf(
        _assetId,
        _accountId32
      );
      return fromChainIdUnit(balance.toString());
    } catch (err: any) {
      console.error("[balanceOf]", err.message);
      return new BigNumber(0);
    }
  }
}
