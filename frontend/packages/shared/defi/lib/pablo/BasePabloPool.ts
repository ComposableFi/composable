import { PabloPoolFeeConfig } from "./PabloPoolFeeConfig";
import { PALLET_TYPE_ID } from "../../constants";
import { PabloPoolAssets } from "./PabloPoolAssets";
import { Exchangeable } from "./Exchangeable";
import { ApiPromise } from "@polkadot/api";
import { concatU8a } from "../../u8a";
import { Asset } from "../assets/Asset";
import BigNumber from "bignumber.js";
import { TokenId } from "tokens";

export class BasePabloPool implements Exchangeable {
  protected readonly __poolId: BigNumber;
  protected readonly __feeConfig: PabloPoolFeeConfig;
  protected readonly __assets: PabloPoolAssets;
  protected readonly __api: ApiPromise;
  protected readonly __owner: string;

  constructor(
    api: ApiPromise,
    poolId: BigNumber,
    assets: PabloPoolAssets,
    feeConfig: PabloPoolFeeConfig,
    owner: string
  ) {
    this.__api = api;
    this.__poolId = poolId;
    this.__assets = assets;
    this.__feeConfig = feeConfig;
    this.__owner = owner;
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

  getAssets(): PabloPoolAssets {
    return this.__assets;
  }

  getFeeConfig(): PabloPoolFeeConfig {
    return this.__feeConfig;
  }

  getPoolId(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__poolId : this.__poolId.toString();
  }

  async getAssetLiquidity(assetId: BigNumber): Promise<BigNumber> {
    const accountId = this.getAccountId();
    const asset = new Asset("", "", "", "" as TokenId, this.__api);
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

  async getSpotPrice(...args: unknown[]): Promise<BigNumber> {
    try {
      // const pair = this.__pair.toJSON();

      // const pricesFor = await this.__api.rpc.pablo.pricesFor(
      //   this.__api.createType("PalletPabloPoolId", this.__poolId.toString()),
      //   this.__api.createType("CustomRpcCurrencyId", pair.base.toString()),
      //   this.__api.createType("CustomRpcCurrencyId", pair.quote.toString()),
      //   this.__api.createType("CustomRpcBalance", toChainIdUnit(1).toString())
      // );

      // const spotPrice = pricesFor.get("spotPrice");
      // return fromChainIdUnit(spotPrice ? BigInt(spotPrice.toString()) : 0);
      return new BigNumber(0);
    } catch (err: any) {
      console.error("[getSpotPrice] ", err.message);
      return new BigNumber(0);
    }
  }

  getApi(): ApiPromise {
    return this.__api;
  }
}
