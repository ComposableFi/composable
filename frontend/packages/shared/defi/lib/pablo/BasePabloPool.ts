import { fromChainIdUnit, toChainIdUnit } from "../../unit";
import { PabloPoolFeeConfig } from "./PabloPoolFeeConfig";
import { PALLET_TYPE_ID } from "../../constants";
import { PabloPoolPair } from "./PabloPoolPair";
import { Exchangeable } from "./Exchangeable";
import { ApiPromise } from "@polkadot/api";
import { concatU8a } from "../../u8a";
import { Asset } from "../Asset";
import BigNumber from "bignumber.js";

export class BasePabloPool implements Exchangeable {
  protected __poolId: BigNumber;
  protected __feeConfig: PabloPoolFeeConfig;
  protected __pair: PabloPoolPair;
  protected __api: ApiPromise;

  constructor(
    api: ApiPromise,
    poolId: BigNumber,
    pair: PabloPoolPair,
    feeConfig: PabloPoolFeeConfig
  ) {
    this.__api = api;
    this.__poolId = poolId;
    this.__pair = pair;
    this.__feeConfig = feeConfig;
  }

  getAccountId(): string {
    const palletId = this.__api.consts.pablo.palletId.toU8a();
    const poolAccountId = this.__api.createType("([u8; 4], [u8; 8], u64)", [
      PALLET_TYPE_ID,
      palletId,
      this.__poolId.toString(),
    ]);

    const accountIdu8a = poolAccountId.toU8a();
    const poolAccount = concatU8a(
      accountIdu8a,
      new Uint8Array(32 - accountIdu8a.length).fill(0)
    );

    return this.__api.createType("AccountId32", poolAccount).toString();
  }

  getPair(): PabloPoolPair {
    return this.__pair;
  }

  getFeeConfig(): PabloPoolFeeConfig {
    return this.__feeConfig;
  }

  getPoolId(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__poolId : this.__poolId.toString();
  }

  async getAssetLiquidity(assetId: BigNumber): Promise<BigNumber> {
    const accountId = this.getAccountId();
    const asset = new Asset("", "", "", this.__api);
    asset.setIdOnChain("picasso", assetId);
    const balance = await asset.balanceOf(accountId);
    return balance;
  }

  async getLiquidity(assets: Asset[]): Promise<Map<string, BigNumber>> {
    const map = new Map<string, BigNumber>();
    const accountId = this.getAccountId();

    for (const asset of assets) {
      const balance = await asset.balanceOf(accountId);
      map.set(asset.getPicassoAssetId() as string, balance);
    }

    return map;
  }

  async getSpotPrice(): Promise<BigNumber> {
    try {
      const pair = this.__pair.toJSON();
      // @ts-ignore
      const pricesFor = await this.__api.rpc.pablo.pricesFor(
        this.__api.createType("PalletPabloPoolId", this.__poolId.toString()),
        this.__api.createType("CustomRpcCurrencyId", pair.base.toString()),
        this.__api.createType("CustomRpcCurrencyId", pair.quote.toString()),
        this.__api.createType("CustomRpcBalance", toChainIdUnit(1).toString())
      );

      const spotPrice = pricesFor.get("spotPrice");
      return fromChainIdUnit(spotPrice ? BigInt(spotPrice.toString()) : 0);
    } catch (err: any) {
      console.error("[getSpotPrice] ", err.message);
      return new BigNumber(0);
    }
  }

  getApi(): ApiPromise {
    return this.__api;
  }
}
