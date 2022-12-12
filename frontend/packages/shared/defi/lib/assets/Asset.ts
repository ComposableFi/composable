import { ApiPromise } from "@polkadot/api";
import { fromChainIdUnit } from "../../unit";
import { DropdownOptionWithIcon } from "../types";
import BigNumber from "bignumber.js";

export class Asset {
  protected __api?: ApiPromise;
  protected readonly __name: string;
  protected readonly __symbol: string;
  protected readonly __iconUrl: string;
  protected __price: BigNumber = new BigNumber(0);
  protected __decimals: Map<string, number>;
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
        icon: asset.getIconUrl(),
      } as DropdownOptionWithIcon;
    });
  }

  constructor(name: string, symbol: string, iconUrl: string, api?: ApiPromise) {
    this.__api = api;
    this.__name = name;
    this.__symbol = symbol;
    this.__iconUrl = iconUrl;
    this.__parachainAssetIds = new Map<string, BigNumber>();
    this.__decimals = new Map<string, number>();
  }

  /**
   * Returns asset id of this asset
   * on picasso parachain
   * string by default
   * @param inBn boolean
   * @returns BigNumber | string
   */
  getPicassoAssetId(inBn: boolean = false): BigNumber | string {
    const picassoAssetId = this.__parachainAssetIds.get("picasso");
    if (!picassoAssetId) throw new Error("Asset Unavailable on Picasso");
    return inBn ? picassoAssetId : picassoAssetId.toString();
  }

  isSupportedOn(network: string): boolean {
    const id = this.__parachainAssetIds.get(network);
    return !!id;
  }

  getSymbol(): string {
    return this.__symbol;
  }

  getName() {
    return this.__name;
  }

  getDecimals(network: string) {
    return this.__decimals.get(network);
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
      if (!this.__api) throw new Error("API Unavailable.");

      const picassoId = this.getPicassoAssetId();

      const _assetId = this.__api.createType(
        "CustomRpcCurrencyId",
        picassoId.toString()
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
      if (!this.__api) throw new Error("API Unavailable.");

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
    this.__parachainAssetIds.set(chainId, assetId);
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

  setApi(api: ApiPromise) {
    this.__api = api;
  }

  setDecimals(network: string, decimals: number) {
    this.__decimals.set(network, decimals);
  }

  getApi(): ApiPromise {
    if (!this.__api) throw new Error("API Unavailable.");
    return this.__api;
  }

  getPrice(): BigNumber {
    return this.__price;
  }

  setPrice(price: BigNumber | string) {
    this.__price = new BigNumber(price);
  }
}

export class OwnedAsset extends Asset {
  protected __balance: BigNumber;

  static fromAsset(asset: Asset, balance: BigNumber): OwnedAsset {
    const ownedAsset = new OwnedAsset(
      asset.getName(),
      asset.getSymbol(),
      asset.getIconUrl(),
      balance,
      asset.getApi()
    );
    ownedAsset.setIdOnChain(
      "picasso",
      asset.getPicassoAssetId(true) as BigNumber
    );
    return ownedAsset;
  }

  constructor(
    name: string,
    symbol: string,
    iconUrl: string,
    balance: BigNumber,
    api?: ApiPromise
  ) {
    super(name, symbol, iconUrl, api);

    this.__balance = balance;
  }

  getBalance(): BigNumber {
    return this.__balance;
  }
}
