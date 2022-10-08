import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "../unit";
import { DropdownOptionWithIcon } from "./types";
import BigNumber from "bignumber.js";

export class Asset {
  protected __api: ApiPromise;
  protected __picassoAssetId: BigNumber;
  protected __name: string;
  protected __symbol: string;
  protected __iconUrl: string;
  protected __parachainAssetIds: Map<string, BigNumber>;
  /**
   * Transform assets list
   * to dropdown options
   * @param {Array<Asset>} assets 
   * @returns {DropdownOptionWithIcon[]}
   */
  static toDropdownList(assets: Asset[]): DropdownOptionWithIcon[] {
    return assets.map((asset) => {
      return {
        label: asset.getName(),
        shortLabel: asset.getSymbol(),
        value: asset.getPicassoAssetId(),
        icon: asset.getIconUrl()
      } as DropdownOptionWithIcon
    })
  }

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
    this.__parachainAssetIds = new Map<string, BigNumber>();
  }

  getPicassoAssetId(inBn: boolean = false): BigNumber | string {
    return inBn ? this.__picassoAssetId : this.__picassoAssetId.toString();
  }

  getSymbol(): string {
    return this.__symbol;
  }

  getName() {
    return this.__name;
  }

  getIconUrl(): string {
    return this.__iconUrl;
  }
  /**
   * Fetch balance of an account
   * @param {string} account 
   * @returns {Promise<BigNumber>}
   */
  async balanceOf(account: string): Promise<BigNumber> {
    try {
      const _assetId = this.__api.createType(
        "CustomRpcCurrencyId",
        this.__picassoAssetId.toString()
      );
      const _accountId32 = this.__api.createType("AccountId32", account);
      const balance = await this.__api.rpc.assets.balanceOf(
        _assetId,
        _accountId32
      );
      return fromChainIdUnit(BigInt(balance.toString()));
    } catch (err: any) {
      console.error("[balanceOf]", err.message);
      return new BigNumber(0);
    }
  }
  /**
   * Fetch total issued amount
   * of this asset
   * @returns {Promise<BigNumber>}
   */
  async totalIssued(): Promise<BigNumber> {
    try {
      const assetId = this.__api.createType(
        "u128",
        this.getPicassoAssetId() as string
      );
      const totalIssued = await this.__api.query.tokens.totalIssuance(assetId);
      return fromChainIdUnit(BigInt(totalIssued.toString()));
    } catch (err: any) {
      return new BigNumber(0);
    }
  }
  /**
   * Set id on a different chain
   * @param {string} chainId 
   * @param {BigNumber} assetId
   */
  setIdOnChain(chainId: string, assetId: BigNumber) {
    this.__parachainAssetIds.set(chainId, assetId)
  }
  /**
   * Get Asset Id on a different chain
   * @param {string} chainId 
   * @param {boolean} inBn 
   * @returns {BigNumber | string}
   */
  getIdOnChain(chainId: string, inBn: boolean = false): BigNumber | string {
    const id = this.__parachainAssetIds.get(chainId);
    if (!id) throw new Error(`Id not set for ${chainId}`);
    return inBn ? id : id.toString();
  }
}
